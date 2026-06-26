use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let query = include_str!("sqlx/20260328100001_create_emails_table.sql");
        manager.get_connection().execute_unprepared(query).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TRIGGER IF EXISTS emails_updated_at_trigger ON emails")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS update_emails_updated_at")
            .await?;
        manager
            .drop_table(Table::drop().table(Alias::new("emails")).to_owned())
            .await
    }
}
