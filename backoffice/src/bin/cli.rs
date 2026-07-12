use backoffice_lib::cli::LogMessage;
use backoffice_lib::cli::errors::CliError;
use backoffice_lib::domain::models::{app_config, user_roles, users};
use backoffice_lib::infrastructure::database::connection::init_db_pool;
use bcrypt::{DEFAULT_COST, hash};
use clap::{Parser, Subcommand};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use ulid::Ulid;

#[derive(Parser)]
#[command(
    name = "backoffice CLI",
    version = "0.1.0",
    about = "Command line interface for backoffice application",
    subcommand_required = true,
    arg_required_else_help = true
)]
struct BackofficeCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the backoffice application
    Init,

    /// Create a new user account
    CreateUser,
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let db = init_db_pool()
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;

    let cli = BackofficeCli::parse();
    parse_commands(cli, &db).await?;
    Ok(())
}

async fn parse_commands(cli: BackofficeCli, db: &DatabaseConnection) -> Result<(), CliError> {
    match cli.command {
        Commands::Init => {
            LogMessage::info("Initializing backoffice application...");
            init(db).await?;
        }

        Commands::CreateUser => {
            LogMessage::info("Creating a new user account...");
            create_user(db).await?;
        }
    }

    Ok(())
}

async fn init(db: &DatabaseConnection) -> Result<(), CliError> {
    let ulid = Ulid::new().to_string();
    let default_app_name = format!("backoffice-{ulid}");

    let app_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Application name")
        .default(default_app_name)
        .interact_text()
        .map_err(|e| CliError::ConfigError(e.to_string()))?;

    let email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Support email")
        .validate_with(|input: &String| {
            if input.contains('@') {
                Ok(())
            } else {
                Err("Invalid email format")
            }
        })
        .interact_text()
        .map_err(|e| CliError::ConfigError(e.to_string()))?;

    let existing = app_config::Entity::find_by_id(1i16)
        .one(db)
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if existing.is_none() {
        app_config::ActiveModel {
            identifier: Set(1),
            app_name: Set(Some(app_name)),
            maintenance_mode: Set(false),
            support_email: Set(Some(email)),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;
    }

    Ok(())
}

async fn create_role(db: &DatabaseConnection) -> Result<String, CliError> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Role name")
        .default("super_admin".into())
        .interact_text()
        .map_err(|e| CliError::ParseError(format!("Failed to read role name: {}", e)))?;

    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Role description")
        .default("Unrestricted access to all resources".into())
        .interact_text()
        .map_err(|e| CliError::ParseError(format!("Failed to read role description: {}", e)))?;

    let identifier = Ulid::new().to_string();

    user_roles::ActiveModel {
        identifier: Set(identifier.clone()),
        name: Set(name.clone()),
        description: Set(Some(description)),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|err| {
        CliError::ParseError(format!("Failed to create role '{}' due to {}", name, err))
    })?;

    Ok(identifier)
}

async fn create_user(db: &DatabaseConnection) -> Result<(), CliError> {
    let super_admin_role_id = create_role(db).await?;

    let admin_email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("user email")
        .interact_text()
        .map_err(|e| CliError::ParseError(e.to_string()))?;

    let admin_password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("User password")
        .interact()
        .map_err(|e| CliError::ParseError(e.to_string()))?;

    let existing = users::Entity::find()
        .filter(users::Column::Email.eq(&admin_email))
        .one(db)
        .await
        .map_err(|e| CliError::OperationFailed(e.to_string()))?;

    if existing.is_some() {
        println!(
            "Super admin user with email '{}' already exists",
            admin_email
        );
        return Ok(());
    }

    let new_admin_id = Ulid::new().to_string();
    let hashed_password = hash(&admin_password.trim(), DEFAULT_COST)
        .map_err(|err| CliError::ParseError(err.to_string()))?;

    users::ActiveModel {
        identifier: Set(new_admin_id),
        role_identifier: Set(Some(super_admin_role_id)),
        email: Set(admin_email.clone()),
        password: Set(hashed_password),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|err| CliError::OperationFailed(err.to_string()))?;

    println!(
        "Super admin user created successfully with email '{}'",
        admin_email
    );
    Ok(())
}