use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::CliError;
use crate::logging::LogMessage;
use crate::templates::backend;

use super::ModuleConfig;

fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().to_path_buf()
}

fn backoffice_root() -> PathBuf {
    workspace_root().join("backoffice")
}

fn domain_root() -> PathBuf {
    backoffice_root().join("crates/backoffice-domain/src")
}

fn infra_root() -> PathBuf {
    backoffice_root().join("crates/backoffice-infra/src")
}

fn api_root() -> PathBuf {
    backoffice_root().join("crates/backoffice-api/src")
}

fn write_file(path: &Path, content: &str) -> Result<(), CliError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| CliError::IoError(format!("Failed to create dir {}: {e}", parent.display())))?;
    }
    fs::write(path, content)
        .map_err(|e| CliError::IoError(format!("Failed to write {}: {e}", path.display())))?;
    Ok(())
}

fn append_to_mod(mod_path: &Path, entry: &str) -> Result<(), CliError> {
    let content = fs::read_to_string(mod_path)
        .map_err(|e| CliError::IoError(format!("Failed to read {}: {e}", mod_path.display())))?;

    let trimmed = content.trim_end();
    let new_line = format!("\npub mod {};", entry);

    if trimmed.contains(&format!("pub mod {};", entry)) {
        LogMessage::warning(&format!("  {} already registered in mod.rs", entry));
        return Ok(());
    }

    let new_content = format!("{}\n", trimmed) + &new_line + "\n";
    fs::write(mod_path, new_content)
        .map_err(|e| CliError::IoError(format!("Failed to write {}: {e}", mod_path.display())))?;
    Ok(())
}

pub fn generate(cfg: &ModuleConfig) -> Result<(), CliError> {
    let root = domain_root();

    // 1. Port (repository trait)
    let port_path = root.join(format!("ports/{}_repository.rs", cfg.name));
    write_file(&port_path, &backend::port(cfg))?;
    LogMessage::step(&format!("Created {}", port_path.strip_prefix(&workspace_root()).unwrap().display()));
    append_to_mod(&root.join("ports/mod.rs"), &format!("{}_repository", cfg.name))?;

    // 2. Service
    let service_path = root.join(format!("services/{}.rs", cfg.name));
    write_file(&service_path, &backend::service(cfg))?;
    LogMessage::step(&format!("Created {}", service_path.strip_prefix(&workspace_root()).unwrap().display()));
    append_to_mod(&root.join("services/mod.rs"), &cfg.name)?;

    // 3. DTOs
    let dto_path = root.join("dto.rs");
    let dto_content = fs::read_to_string(&dto_path)
        .map_err(|e| CliError::IoError(format!("Failed to read dto.rs: {e}")))?;
    let create_dto = backend::dto_create(cfg);
    let update_dto = backend::dto_update(cfg);
    let new_dto_content = format!("{}\n{}\n", dto_content.trim_end(), create_dto.trim());
    let new_dto_content = format!("{}\n{}\n", new_dto_content.trim_end(), update_dto.trim());
    fs::write(&dto_path, new_dto_content)
        .map_err(|e| CliError::IoError(format!("Failed to write dto.rs: {e}")))?;
    LogMessage::step(&format!("Appended DTOs to {}", dto_path.strip_prefix(&workspace_root()).unwrap().display()));

    // 4. Repository (implementation)
    let repo_root = infra_root().join("database/repositories");
    let repo_path = repo_root.join(format!("{}_repository.rs", cfg.name));
    write_file(&repo_path, &backend::repository(cfg))?;
    LogMessage::step(&format!("Created {}", repo_path.strip_prefix(&workspace_root()).unwrap().display()));
    append_to_mod(&repo_root.join("mod.rs"), &format!("{}_repository", cfg.name))?;

    // 5. Handler
    let handler_path = api_root().join(format!("http/handlers/{}.rs", cfg.name_plural));
    write_file(&handler_path, &backend::handler(cfg))?;
    LogMessage::step(&format!("Created {}", handler_path.strip_prefix(&workspace_root()).unwrap().display()));
    append_to_mod(&api_root().join("http/handlers/mod.rs"), &cfg.name_plural)?;

    // 6. Extractor
    let extractor_path = api_root().join(format!("http/extractors/{}.rs", cfg.name));
    write_file(&extractor_path, &backend::extractor(cfg))?;
    LogMessage::step(&format!("Created {}", extractor_path.strip_prefix(&workspace_root()).unwrap().display()));
    append_to_mod(&api_root().join("http/extractors/mod.rs"), &cfg.name)?;

    // 7. Route
    let route_path = api_root().join(format!("http/routes/{}.rs", cfg.name));
    write_file(&route_path, &backend::route(cfg))?;
    LogMessage::step(&format!("Created {}", route_path.strip_prefix(&workspace_root()).unwrap().display()));
    append_to_mod(&api_root().join("http/routes/mod.rs"), &cfg.name)?;

    // 8. Router merge
    update_router(cfg)?;

    // 9. AppState
    update_state(cfg)?;

    Ok(())
}

fn update_router(cfg: &ModuleConfig) -> Result<(), CliError> {
    let router_path = api_root().join("http/routes/router.rs");
    let content = fs::read_to_string(&router_path)
        .map_err(|e| CliError::IoError(format!("Failed to read router.rs: {e}")))?;

    let import_module = cfg.name.clone();
    let import_fn = format!("{}_routes", cfg.name);

    if content.contains(&format!("{}::{}", import_module, import_fn)) {
        LogMessage::warning("  Router import already exists, skipping");
        return Ok(());
    }

    // Add import
    let use_line = format!(
        "use crate::http::routes::{}::{};",
        import_module, import_fn
    );
    let new_content = if !content.contains(&use_line) {
        // Find the last `use crate::http::routes::` line and insert after it
        let lines: Vec<&str> = content.lines().collect();
        // Find the last routes import line
        let last_routes_idx = lines.iter().rposition(|l| l.starts_with("use crate::http::routes::"));
        if let Some(idx) = last_routes_idx {
            let mut result_lines: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result_lines.push(use_line);
            result_lines.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result_lines.join("\n")
        } else {
            content
        }
    } else {
        content
    };

    // Add merge line
    let merge_line = format!("                .merge({}(Arc::clone(&state)))", import_fn);
    let new_content = if new_content.contains(&merge_line) {
        new_content
    } else {
        // Find the last .merge line and insert after it
        let lines: Vec<&str> = new_content.lines().collect();
        let last_merge_idx = lines.iter().rposition(|l| l.trim().starts_with(".merge("));
        if let Some(idx) = last_merge_idx {
            let mut result_lines: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result_lines.push(merge_line);
            result_lines.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result_lines.join("\n")
        } else {
            new_content
        }
    };

    fs::write(&router_path, new_content)
        .map_err(|e| CliError::IoError(format!("Failed to write router.rs: {e}")))?;
    LogMessage::step(&format!("Updated {}", router_path.strip_prefix(&workspace_root()).unwrap().display()));
    Ok(())
}

fn update_state(cfg: &ModuleConfig) -> Result<(), CliError> {
    let state_path = api_root().join("state.rs");
    let content = fs::read_to_string(&state_path)
        .map_err(|e| CliError::IoError(format!("Failed to read state.rs: {e}")))?;

    let repo_struct = format!("{}Repository", cfg.name_pascal);
    let service_struct = format!("{}Service", cfg.name_pascal);
    let repo_field = format!("{}_repository", cfg.name);
    let service_field = format!("{}_service", cfg.name);

    // Check if already added
    if content.contains(&service_field) {
        LogMessage::warning("  State already configured, skipping");
        return Ok(());
    }

    // 1. Add service import
    let service_import = format!(
        "        {}::{{{}}},",
        cfg.name, service_struct
    );
    let new_content = if !content.contains(&service_struct) {
        let lines: Vec<&str> = content.lines().collect();
        let last_service_import = lines.iter().rposition(|l| {
            l.trim().starts_with("services::")
                && l.contains("},")
        });
        if let Some(idx) = last_service_import {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(service_import);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            content
        }
    } else {
        content
    };

    // 2. Add repo import
    let repo_import_line = format!(
        "        {}::{{{}}},",
        cfg.name, repo_struct
    );
    let new_content = if !new_content.contains(&repo_struct) {
        let lines: Vec<&str> = new_content.lines().collect();
        let last_repo_import = lines.iter().rposition(|l| {
            l.trim().starts_with("database::repositories::")
                && l.contains("},")
        });
        if let Some(idx) = last_repo_import {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(repo_import_line);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            new_content
        }
    } else {
        new_content
    };

    // 3. Add field to ServicesState struct
    let field_line = format!("    pub {}: {}<{}>,", service_field, service_struct, repo_struct);
    let new_content = if !new_content.contains(&field_line) {
        let lines: Vec<&str> = new_content.lines().collect();
        let last_field_idx = lines.iter().rposition(|l| {
            l.trim().starts_with("pub ") && l.contains("Service<")
        });
        if let Some(idx) = last_field_idx {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(field_line);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            new_content
        }
    } else {
        new_content
    };

    // 4. Add to ServicesState::new parameters
    let param_line = format!("        {}: {},", repo_field, repo_struct);
    let new_content = if !new_content.contains(&param_line) {
        let lines: Vec<&str> = new_content.lines().collect();
        let last_param_idx = lines.iter().rposition(|l| {
            l.trim().ends_with("Repository,") || l.trim().ends_with("Repository,")
        });
        if let Some(idx) = last_param_idx {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(param_line);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            new_content
        }
    } else {
        new_content
    };

    // 5. Add service init in new()
    let init_line = format!(
        "        let {} = {}::new({});",
        service_field, service_struct, repo_field
    );
    let new_content = if !new_content.contains(&init_line) {
        let lines: Vec<&str> = new_content.lines().collect();
        let last_init_idx = lines.iter().rposition(|l| {
            l.trim().starts_with("let ") && l.contains("Service::new(")
        });
        if let Some(idx) = last_init_idx {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(init_line);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            new_content
        }
    } else {
        new_content
    };

    // 6. Add to Self {} block
    let self_field = format!("            {},", service_field);
    let new_content = if !new_content.contains(&self_field) {
        let lines: Vec<&str> = new_content.lines().collect();
        let last_self_field_idx = lines.iter().rposition(|l| {
            l.trim().ends_with(',') && l.contains("_service,")
        });
        if let Some(idx) = last_self_field_idx {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(self_field);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            new_content
        }
    } else {
        new_content
    };

    // 7. Add repo init in AppState::new
    let repo_init = format!(
        "        let {} = {}::init(db_conn);",
        repo_field, repo_struct
    );
    let new_content = if !new_content.contains(&repo_init) {
        let lines: Vec<&str> = new_content.lines().collect();
        let last_repo_init_idx = lines.iter().rposition(|l| {
            l.trim().starts_with("let ") && l.contains("Repository::init(")
        });
        if let Some(idx) = last_repo_init_idx {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(repo_init);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            new_content
        }
    } else {
        new_content
    };

    // 8. Add repo to ServicesState::new() call
    let repo_call = format!("            {},", repo_field);
    let new_content = if !new_content.contains(&repo_call) {
        let lines: Vec<&str> = new_content.lines().collect();
        let last_repo_call_idx = lines.iter().rposition(|l| {
            l.trim().ends_with(',') && l.contains("_repository,")
        });
        if let Some(idx) = last_repo_call_idx {
            let mut result: Vec<String> = lines[..idx + 1].iter().map(|s| s.to_string()).collect();
            result.push(repo_call);
            result.extend(lines[idx + 1..].iter().map(|s| s.to_string()));
            result.join("\n")
        } else {
            new_content
        }
    } else {
        new_content
    };

    fs::write(&state_path, new_content)
        .map_err(|e| CliError::IoError(format!("Failed to write state.rs: {e}")))?;
    LogMessage::step(&format!("Updated {}", state_path.strip_prefix(&workspace_root()).unwrap().display()));
    Ok(())
}
