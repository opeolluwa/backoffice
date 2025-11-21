use backoffice_lib::cli::LogMessage;
use backoffice_lib::cli::errors::CliError;
use backoffice_lib::config::database::AppDatabase;
use backoffice_lib::entities::app::App;
use backoffice_lib::entities::privileged_user::PrivilegedUser;
use bcrypt::{DEFAULT_COST, hash};
use clap::{Parser, Subcommand};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use sqlx::PgPool;
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
    let database_pool = AppDatabase::init_pool()
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;

    let cli = BackofficeCli::parse();
    parse_commands(cli, &database_pool).await?;
    Ok(())
}

async fn parse_commands(cli: BackofficeCli, db: &PgPool) -> Result<(), CliError> {
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

pub async fn init(db: &PgPool) -> Result<App, CliError> {
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

    let config = sqlx::query_as!(
        App,
        r#"
        INSERT INTO app_config (
            identifier,
            app_name,
            maintenance_mode,
            support_email
        ) VALUES (
            1, $1, $2, $3
        )
        ON CONFLICT (identifier) DO NOTHING
        RETURNING *;
        "#,
        app_name,
        false,
        email
    )
    .fetch_one(db)
    .await?;

    Ok(config)
}

pub async fn create_role(db: &PgPool) -> Result<String, CliError> {
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

    sqlx::query!(
        r#"
        INSERT INTO user_roles (identifier, name, description)
        VALUES ($1, $2, $3)
        "#,
        identifier,
        name,
        description
    )
    .execute(db)
    .await
    .map_err(|err| {
        CliError::ParseError(format!("Failed to create role '{}' due to {}", name, err))
    })?;

    Ok(identifier)
}

pub async fn super_admin_exists(db: &PgPool) -> Result<bool, CliError> {
    let privilege_user = sqlx::query_as!(
        PrivilegedUser,
        r#"
     SELECT
            u.email,
            u.identifier,
            u.first_name,
            u.last_name,
            u.is_active,
            r.name AS role_name,
            r.description AS role_description

        FROM users u
        JOIN user_roles r ON u.role_identifier = r.identifier
        WHERE r.name = 'super_admin'
        LIMIT 1
    "#
    )
    .fetch_optional(db)
    .await
    .map_err(|e| CliError::ParseError(format!("DB error: {}", e)))?;

    Ok(privilege_user.is_some())
}

pub async fn create_user(db: &PgPool) -> Result<(), CliError> {
    let super_admin_role_id = create_role(db).await?;

    let admin_email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("user email")
        .interact_text()
        .map_err(|e| CliError::ParseError(e.to_string()))?;

    let admin_password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("User password")
        .interact()
        .map_err(|e| CliError::ParseError(e.to_string()))?;

    let existing_admin: Option<(String,)> =
        sqlx::query_scalar(r#"SELECT identifier FROM users WHERE email = $1"#)
            .bind(&admin_email)
            .fetch_optional(db)
            .await
            .map_err(|e| CliError::OperationFailed(e.to_string()))?;

    if existing_admin.is_some() {
        println!(
            "Super admin user with email '{}' already exists",
            admin_email
        );
        return Ok(());
    }

    let new_admin_id = Ulid::new().to_string();
    let hashed_password = hash(&admin_password.trim(), DEFAULT_COST)
        .map_err(|err| CliError::ParseError(err.to_string()))?;

    sqlx::query!(
        r#"
        INSERT INTO users (identifier, role_identifier, email, password, is_active)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        new_admin_id,
        super_admin_role_id,
        admin_email,
        hashed_password,
        true
    )
    .execute(db)
    .await
    .map_err(|err| CliError::OperationFailed(err.to_string()))?;

    println!(
        "Super admin user created successfully with email '{}'",
        admin_email
    );
    Ok(())
}
