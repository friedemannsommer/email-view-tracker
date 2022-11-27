use sea_orm_migration::prelude::*;

use crate::m20221127_000001_create_users::User;

#[derive(Iden)]
enum Tracker {
    #[iden = "trackers"]
    Table,
    Id,
    Name,
    Views,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tracker::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tracker::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tracker::Name).string().not_null())
                    .col(
                        ColumnDef::new(Tracker::Views)
                            .big_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Tracker::UserId).uuid().not_null())
                    .col(ColumnDef::new(Tracker::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Tracker::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(User::Table, User::Id)
                    .to(Tracker::Table, Tracker::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(Index::create().col(Tracker::Id).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tracker::Table).if_exists().to_owned())
            .await
    }
}
