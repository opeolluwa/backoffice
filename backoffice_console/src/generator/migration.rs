use std::path::PathBuf;
use std::process::Command;

use crate::errors::CliError;
use crate::logging::LogMessage;

use super::ModuleConfig;

fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().to_path_buf()
}

fn backoffice_root() -> PathBuf {
    workspace_root().join("backoffice")
}

pub fn generate(cfg: &ModuleConfig) -> Result<(), CliError> {
    let migration_name = format!("create_{}_table", cfg.name_plural);
    LogMessage::info(&format!("Generating migration: {}", migration_name));

    let output = Command::new("sea-orm-cli")
        .args(["migrate", "generate", &migration_name])
        .current_dir(backoffice_root())
        .output()
        .map_err(|e| CliError::OperationFailed(format!("Failed to run sea-orm-cli: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // sea-orm-cli might not be installed, warn but don't fail
        LogMessage::warning(&format!(
            "  sea-orm-cli migration generation failed (you can run it manually):\n  sea-orm-cli migrate generate {}",
            migration_name
        ));
        if !stderr.is_empty() {
            LogMessage::warning(&format!("  {}", stderr.trim()));
        }
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        LogMessage::step(&format!("Migration generated: {}", stdout.trim()));
    }

    Ok(())
}
