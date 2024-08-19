use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {        

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::UserName).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string())
                    .col(ColumnDef::new(User::LastName).string())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Age).integer().not_null())
                    .col(ColumnDef::new(User::Gender).string().not_null())
                    .col(ColumnDef::new(User::Location).string())
                    .col(ColumnDef::new(User::Openness).string())
                    .col(ColumnDef::new(User::FavActiv).string())
                    .col(ColumnDef::new(User::ExpQual).string())
                    .col(ColumnDef::new(User::RelationType).string())
                    .col(ColumnDef::new(User::SocialHabits).string())
                    .col(ColumnDef::new(User::CommMethod).string())
                    .col(ColumnDef::new(User::PastRelations).string())
                    .col(ColumnDef::new(User::ImageUrl).string().not_null())
                    .col(ColumnDef::new(User::Score).integer().not_null())
                    .col(ColumnDef::new(User::Uuid).uuid().unique_key().not_null())
                    .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserName,
    FirstName,
    LastName,
    Email,
    Password,
    Age,
    Gender,
    Location,
    Openness,
    FavActiv,
    ExpQual,
    RelationType,
    SocialHabits,
    CommMethod,
    PastRelations,
    ImageUrl,
    Score,
    Uuid,
    CreatedAt,
}
