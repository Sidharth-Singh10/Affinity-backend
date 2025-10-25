use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(integer(Users::Id).not_null().auto_increment().primary_key())
                    .col(string(Users::Username).not_null().unique_key())
                    .col(string(Users::FirstName).not_null())
                    .col(string(Users::LastName).not_null())
                    .col(integer(Users::Age).not_null())
                    .col(string(Users::Email).not_null().unique_key())
                    .col(string(Users::Password).not_null())
                    .col(string(Users::Gender).not_null())
                    .col(
                        timestamp(Users::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create UserDetails table
        manager
            .create_table(
                Table::create()
                    .table(UserDetails::Table)
                    .if_not_exists()
                    .col(integer(UserDetails::UserId).not_null().primary_key())
                    .col(string(UserDetails::Location))
                    .col(string(UserDetails::Openness))
                    .col(string(UserDetails::Interests))
                    .col(string(UserDetails::ExpQual))
                    .col(string(UserDetails::RelationType))
                    .col(string(UserDetails::SocialHabits))
                    .col(string(UserDetails::PastRelations))
                    .col(string(UserDetails::Values))
                    .col(string(UserDetails::Style))
                    .col(string(UserDetails::Traits))
                    .col(string(UserDetails::Commitment))
                    .col(string(UserDetails::Resolution))
                    .col(string(UserDetails::ImageUrl))
                    .col(string(UserDetails::Bio))
                    .col(float(UserDetails::Score))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_details_user")
                            .from(UserDetails::Table, UserDetails::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Matches table
        manager
            .create_table(
                Table::create()
                    .table(Matches::Table)
                    .if_not_exists()
                    .col(
                        integer(Matches::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(integer(Matches::MaleId).not_null())
                    .col(integer(Matches::FemaleId).not_null())
                    .col(string(Matches::Status).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_matches_male")
                            .from(Matches::Table, Matches::MaleId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_matches_female")
                            .from(Matches::Table, Matches::FemaleId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create GameSessions table
        manager
            .create_table(
                Table::create()
                    .table(GameSessions::Table)
                    .if_not_exists()
                    .col(
                        integer(GameSessions::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(integer(GameSessions::MaleId).not_null())
                    .col(integer(GameSessions::FemaleId).not_null())
                    .col(float(GameSessions::Score))
                    .col(integer(GameSessions::GameId).not_null())
                    .col(integer(GameSessions::MatchId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_sessions_male")
                            .from(GameSessions::Table, GameSessions::MaleId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_sessions_female")
                            .from(GameSessions::Table, GameSessions::FemaleId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_sessions_match")
                            .from(GameSessions::Table, GameSessions::MatchId)
                            .to(Matches::Table, Matches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Avatar table
        manager
            .create_table(
                Table::create()
                    .table(Avatar::Table)
                    .if_not_exists()
                    .col(
                        integer(Avatar::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(integer(Avatar::UserId).not_null())
                    .col(string(Avatar::ObjectKey).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_avatar_user")
                            .from(Avatar::Table, Avatar::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create PassReset table
        manager
            .create_table(
                Table::create()
                    .table(PassReset::Table)
                    .if_not_exists()
                    .col(integer(PassReset::UserId).not_null())
                    .col(string(PassReset::Token).not_null())
                    .col(big_integer(PassReset::TokenExpiry).not_null())
                    // Define composite primary key
                    .primary_key(
                        Index::create()
                            .name("user_token")
                            .col(PassReset::UserId)
                            .col(PassReset::Token)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_passreset_user")
                            .from(PassReset::Table, PassReset::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order to avoid foreign key constraint violations
        manager
            .drop_table(Table::drop().table(PassReset::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Avatar::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GameSessions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Matches::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserDetails::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    FirstName,
    LastName,
    Age,
    Email,
    Password,
    Gender,
    CreatedAt,
}

#[derive(DeriveIden)]
enum UserDetails {
    Table,
    UserId,
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
    Bio,
}

#[derive(DeriveIden)]
enum Matches {
    Table,
    Id,
    MaleId,
    FemaleId,
    Status,
}

#[derive(DeriveIden)]
enum GameSessions {
    Table,
    Id,
    MaleId,
    FemaleId,
    Score,
    GameId,
    MatchId,
}

#[derive(DeriveIden)]
enum Avatar {
    Table,
    Id,
    UserId,
    ObjectKey,
}

#[derive(DeriveIden)]
enum PassReset {
    Table,
    UserId,
    Token,
    TokenExpiry,
}
