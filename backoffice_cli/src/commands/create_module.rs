use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use dialoguer::Select;

use crate::errors::CliError;
use crate::generator::{backend, frontend, migration, ModuleConfig};
use crate::logging::LogMessage;

pub fn run() -> Result<(), CliError> {
    LogMessage::info("Create Module Generator");
    println!();

    // 1. Module name
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Module name (snake_case)")
        .validate_with(|input: &String| {
            if input.is_empty() {
                return Err("Module name cannot be empty");
            }
            if input.contains(' ') {
                return Err("Module name cannot contain spaces");
            }
            Ok(())
        })
        .interact_text()
        .map_err(|e| CliError::ParseError(format!("Failed to read module name: {e}")))?;

    let name = name.trim().to_lowercase();
    let cfg = ModuleConfig::new(&name, "");

    // 2. Description
    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Description for {}", cfg.name_pascal))
        .default(format!("{} management", cfg.name_pascal))
        .interact_text()
        .map_err(|e| CliError::ParseError(format!("Failed to read description: {e}")))?;

    let cfg = ModuleConfig::new(&name, &description);

    // 3. Sidebar section
    let sections = vec!["Operations", "Workspace"];
    let section_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Sidebar section")
        .items(&sections)
        .default(0)
        .interact()
        .map_err(|e| CliError::ParseError(format!("Failed to read section: {e}")))?;

    LogMessage::info(&format!("  Section: {}", sections[section_idx]));
    println!();

    // Summary
    LogMessage::info("Summary:");
    LogMessage::step(&format!("Module: {} ({})", cfg.name_pascal, cfg.name));
    LogMessage::step(&format!("Plural: {}", cfg.name_plural));
    LogMessage::step(&format!("Description: {}", cfg.description));
    println!();

    // Generate backend
    LogMessage::info("Generating backend files...");
    backend::generate(&cfg)?;
    println!();

    // Generate frontend
    LogMessage::info("Generating frontend files...");
    frontend::generate(&cfg)?;
    println!();

    // Generate migration
    LogMessage::info("Generating migration...");
    migration::generate(&cfg)?;
    println!();

    LogMessage::success(&format!("Module '{}' generated successfully!", cfg.name_pascal));
    println!();
    LogMessage::info("Next steps:");
    LogMessage::step(&format!("Run `sea-orm-cli migrate generate create_{}_table` if migration didn't auto-generate", cfg.name_plural));
    LogMessage::step(&format!("Run `just generate-entities` to regenerate SeaORM models"));
    LogMessage::step(&format!("Run `just export-bindings` to update TypeScript bindings"));
    LogMessage::step(&format!("Run `cargo check --workspace` to verify compilation"));

    Ok(())
}
