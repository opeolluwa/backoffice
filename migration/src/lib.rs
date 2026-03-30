mod m20250508_133912_create_users_table;
mod m20251109_214041_create_user_roles_table;
mod m20251109_214342_add_role_to_users;
mod m20251115_183510_create_marketplace_table;
mod m20251116_020338_rename_marketplace;
mod m20251117_222503_create_config_table;
mod m20251118_164259_create_products_table;
mod m20251118_164849_create_activities_table;
mod m20251121_234155_create_countries_table;
mod m20251125_134721_add_currency_to_product;
mod m20260328_073612_create_teams_table;
mod m20260328_073621_create_team_invitations_table;
mod m20260328_100000_create_uploads_table;
mod m20260328_100001_create_emails_table;
mod m20260329_000000_add_pk_to_activities;
mod m20260330_012735_xreate_admin_table;
mod m20260330_013623_create_newsletter_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> sea_orm::DynIden {
        "weangel_server_migrations".into_iden()
    }
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250508_133912_create_users_table::Migration),
            Box::new(m20251109_214041_create_user_roles_table::Migration),
            Box::new(m20251109_214342_add_role_to_users::Migration),
            Box::new(m20251115_183510_create_marketplace_table::Migration),
            Box::new(m20251116_020338_rename_marketplace::Migration),
            Box::new(m20251117_222503_create_config_table::Migration),
            Box::new(m20251118_164259_create_products_table::Migration),
            Box::new(m20251118_164849_create_activities_table::Migration),
            Box::new(m20251121_234155_create_countries_table::Migration),
            Box::new(m20251125_134721_add_currency_to_product::Migration),
            Box::new(m20260328_073612_create_teams_table::Migration),
            Box::new(m20260328_073621_create_team_invitations_table::Migration),
            Box::new(m20260328_100000_create_uploads_table::Migration),
            Box::new(m20260328_100001_create_emails_table::Migration),
            Box::new(m20260329_000000_add_pk_to_activities::Migration),
            Box::new(m20260330_012735_xreate_admin_table::Migration),
            Box::new(m20260330_013623_create_newsletter_table::Migration),
        ]
    }
}
