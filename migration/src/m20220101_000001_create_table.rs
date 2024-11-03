use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::UserName).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::FirstName).string())
                    .col(ColumnDef::new(Users::LastName).string())
                    .col(ColumnDef::new(Users::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::Age).integer().not_null())
                    .col(ColumnDef::new(Users::Gender).string().not_null())
                    .col(ColumnDef::new(Users::Location).string())
                    .col(ColumnDef::new(Users::Openness).string())
                    .col(ColumnDef::new(Users::Interests).string())
                    .col(ColumnDef::new(Users::ExpQual).string())
                    .col(ColumnDef::new(Users::RelationType).string())
                    .col(ColumnDef::new(Users::SocialHabits).string())
                    .col(ColumnDef::new(Users::PastRelations).string())
                    .col(ColumnDef::new(Users::Values).string())
                    .col(ColumnDef::new(Users::Style).string())
                    .col(ColumnDef::new(Users::Traits).string())
                    .col(ColumnDef::new(Users::Commitment).string())
                    .col(ColumnDef::new(Users::Resolution).string())
                    .col(ColumnDef::new(Users::ImageUrl).string().not_null())
                    .col(ColumnDef::new(Users::Score).integer().not_null())
                    .col(ColumnDef::new(Users::Uuid).uuid().unique_key().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
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
    Interests,
    ExpQual,
    RelationType,
    SocialHabits,
    PastRelations,
    Values,
    Style,
    Traits,
    Commitment,
    Resolution,
    ImageUrl,
    Score,
    Uuid,
    CreatedAt,
}
