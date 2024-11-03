use crate::m20220101_000001_create_table::Users;
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
                    .table(Avatar::Table)
                    .if_not_exists()
                    .col(pk_auto(Avatar::Id))
                    .col(string(Avatar::UserName).not_null())
                    .col(string(Avatar::ObjectKey))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-avatar-username")
                            .from(Avatar::Table, Avatar::UserName)
                            .to(Users::Table, Users::UserName)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Avatar::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Avatar {
    Table,
    Id,
    UserName,
    ObjectKey,
}
