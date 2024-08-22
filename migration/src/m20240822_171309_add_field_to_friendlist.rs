use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(FriendList::Table)
                    .if_not_exists()
                    .col(pk_auto(FriendList::Id))
                    .col(string(FriendList::GirlEmailId).not_null())
                    .col(string(FriendList::BoyEmailId).not_null())
                    .col(string(FriendList::Flag).not_null())
                    .col(string(FriendList::ContestScore).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(FriendList::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FriendList {
    Table,
    Id,
    GirlEmailId,
    BoyEmailId,
    Flag,
    ContestScore,
}
