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
mod m20260706_222817_update_uploads_table;
mod m20260712_090843_creat_invitations_table;
mod m20260719_094541_add_file_path_and_thumbname_to_uploads;
mod m20260721_202604_create_orders_table;
mod m20260722_131248_do_rabc;
mod m20260722_133642_define_rbac_rules;
mod m20260722_134014_define_rbac_roles;
mod m20260722_140000_drop_created_by_from_products;
mod m20260722_150000_drop_marketplace;
mod m20260722_163951_seed_country_table;
mod m20260723_162400_create_admin_table;
mod m20260723_162419_create_customer_table;
mod m20260723_162500_create_invoices_table;
mod m20260723_162600_create_complaints_table;
mod m20260723_162700_change_price_to_bigint;
mod m20260723_171744_invoice_to_int;
mod m20260724_000000_add_locale_to_app_config;
mod m20260724_010000_add_profile_fields_to_users;
mod m20260724_020000_change_app_config_id_to_ulid;
mod m20260724_030000_add_brand_color_to_app_config;
mod m20260724_040000_add_logo_url_to_app_config;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> sea_orm::DynIden {
        "backoffice_server_migrations".into_iden()
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
            Box::new(m20260706_222817_update_uploads_table::Migration),
            Box::new(m20260712_090843_creat_invitations_table::Migration),
            Box::new(m20260719_094541_add_file_path_and_thumbname_to_uploads::Migration),
            Box::new(m20260721_202604_create_orders_table::Migration),
            Box::new(m20260722_131248_do_rabc::Migration),
            Box::new(m20260722_133642_define_rbac_rules::Migration),
            Box::new(m20260722_134014_define_rbac_roles::Migration),
            Box::new(m20260722_140000_drop_created_by_from_products::Migration),
            Box::new(m20260722_150000_drop_marketplace::Migration),
            Box::new(m20260722_163951_seed_country_table::Migration),
            Box::new(m20260723_162400_create_admin_table::Migration),
            Box::new(m20260723_162419_create_customer_table::Migration),
            Box::new(m20260723_162500_create_invoices_table::Migration),
            Box::new(m20260723_162600_create_complaints_table::Migration),
            Box::new(m20260723_162700_change_price_to_bigint::Migration),
            Box::new(m20260723_171744_invoice_to_int::Migration),
            Box::new(m20260724_000000_add_locale_to_app_config::Migration),
            Box::new(m20260724_010000_add_profile_fields_to_users::Migration),
            Box::new(m20260724_020000_change_app_config_id_to_ulid::Migration),
            Box::new(m20260724_030000_add_brand_color_to_app_config::Migration),
            Box::new(m20260724_040000_add_logo_url_to_app_config::Migration),
        ]
    }
}
