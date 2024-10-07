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
                    .table(PassReset::Table)
                    .if_not_exists()
                    .col(integer(PassReset::UserId).not_null())
                    .col(string(PassReset::Token).not_null())
                    .col(big_integer(PassReset::TokenExpiry).not_null())
                    // Define composite primary key
                    .primary_key(Index::create().col(PassReset::UserId).col(PassReset::Token))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(PassReset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PassReset {
    Table,
    UserId,
    Token,
    TokenExpiry,
}
