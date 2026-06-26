use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let query = include_str!("sqlx/20260328100000_create_uploads_table.sql");
        manager.get_connection().execute_unprepared(query).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TRIGGER IF EXISTS uploads_updated_at_trigger ON uploads")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS update_uploads_updated_at")
            .await?;
        manager
            .drop_table(Table::drop().table(Alias::new("uploads")).to_owned())
            .await
    }
}
