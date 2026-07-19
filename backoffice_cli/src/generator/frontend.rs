use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::CliError;
use crate::logging::LogMessage;
use crate::templates::frontend;

use super::ModuleConfig;

fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().to_path_buf()
}

fn web_root() -> PathBuf {
    workspace_root().join("backoffice_web/app")
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

pub fn generate(cfg: &ModuleConfig) -> Result<(), CliError> {
    let root = web_root();

    // 1. Store
    let store_path = root.join(format!("stores/{}.ts", cfg.name));
    write_file(&store_path, &frontend::store(cfg))?;
    LogMessage::step(&format!("Created {}", store_path.strip_prefix(&workspace_root()).unwrap().display()));

    // 2. Page
    let page_path = root.join(format!("pages/{}/index.vue", cfg.name_plural));
    write_file(&page_path, &frontend::page(cfg))?;
    LogMessage::step(&format!("Created {}", page_path.strip_prefix(&workspace_root()).unwrap().display()));

    // 3. Components folder
    let components_dir = root.join(format!("components/{}", cfg.name_plural));
    fs::create_dir_all(&components_dir)
        .map_err(|e| CliError::IoError(format!("Failed to create components dir: {e}")))?;
    let index_path = components_dir.join("index.ts");
    write_file(&index_path, "// Components for this module\n")?;
    LogMessage::step(&format!("Created {}", components_dir.strip_prefix(&workspace_root()).unwrap().display()));

    // 4. Sidebar entry
    update_dashboard(cfg)?;

    Ok(())
}

fn update_dashboard(cfg: &ModuleConfig) -> Result<(), CliError> {
    let dashboard_path = web_root().join("layouts/dashboard.vue");
    let content = fs::read_to_string(&dashboard_path)
        .map_err(|e| CliError::IoError(format!("Failed to read dashboard.vue: {e}")))?;

    // Check if already added
    if content.contains(&format!("to: \"/{}\"", cfg.name_plural)) {
        LogMessage::warning("  Sidebar entry already exists, skipping");
        return Ok(());
    }

    let sidebar_entry = format!(
        "  {{\n    label: \"{}\",\n    icon: \"heroicons:puzzle-piece\",\n    to: \"/{}\",\n  }},",
        cfg.name_pascal, cfg.name_plural
    );

    // Find the section to insert into
    let lines: Vec<&str> = content.lines().collect();
    let mut result_lines = Vec::new();
    let mut inserted = false;

    for (_i, line) in lines.iter().enumerate() {
        result_lines.push(line.to_string());

        // Insert before the next label section or before the closing bracket
        if !inserted {
            let is_section_label = line.contains("type: \"label\" as const");
            let is_closing = line.trim() == "];";

            if is_section_label || is_closing {
                // Insert before this line
                let last = result_lines.pop().unwrap();
                result_lines.push(sidebar_entry.clone());
                result_lines.push(last);
                inserted = true;
            }
        }
    }

    if !inserted {
        LogMessage::warning("  Could not determine insertion point in dashboard.vue");
        return Ok(());
    }

    let new_content = result_lines.join("\n");
    fs::write(&dashboard_path, new_content)
        .map_err(|e| CliError::IoError(format!("Failed to write dashboard.vue: {e}")))?;
    LogMessage::step(&format!("Updated {}", dashboard_path.strip_prefix(&workspace_root()).unwrap().display()));
    Ok(())
}
