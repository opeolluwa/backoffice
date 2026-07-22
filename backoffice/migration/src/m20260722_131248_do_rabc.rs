use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        sea_orm::rbac::schema::create_tables(db, Default::default()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tables = sea_orm::rbac::schema::all_tables();

        for table_name in tables {
            manager
                .drop_table(Table::drop().table(table_name).to_owned())
                .await?;
        }
        Ok(())
    }
}
