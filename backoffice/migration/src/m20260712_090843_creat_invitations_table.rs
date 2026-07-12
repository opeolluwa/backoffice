use sea_orm_migration::{prelude::*, schema::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(InvitationStatus::Type)
                    .values(vec![
                        InvitationStatus::Pending,
                        InvitationStatus::Accepted,
                        InvitationStatus::Rejected,
                        InvitationStatus::Expired,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Invitation::Table)
                    .if_not_exists()
                    .col(pk_uuid(Invitation::Identifier))
                    .col(string(Invitation::Email))
                    .col(
                        ColumnDef::new(Invitation::Status)
                            .enumeration(
                                InvitationStatus::Type,
                                vec![
                                    InvitationStatus::Pending,
                                    InvitationStatus::Accepted,
                                    InvitationStatus::Rejected,
                                    InvitationStatus::Expired,
                                ],
                            )
                            // .default(InvitationStatus::Pending.to_string()),
                    )
                    .col(string(Invitation::Token))
                    .col(date_time(Invitation::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(Invitation::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Invitation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Invitation {
    Table,
    Identifier,
    Email,
    CreatedAt,
    UpdatedAt,
    Status,
    Token,
}

#[derive(DeriveIden)]
enum InvitationStatus {
    #[sea_orm(iden = "invitation_status")]
    Type,
    Pending,
    Accepted,
    Rejected,
    Expired,
}
