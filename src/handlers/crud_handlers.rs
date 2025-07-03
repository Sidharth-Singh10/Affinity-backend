use std::collections::HashMap;
use std::time::Duration;

use crate::model::{
    BoyScoreInfo, CharacterDetails, GameSession, GetUserInfo, GirlBoyInfoById, MatchListInfo,
    UpdateScoreInfo,
};
use aws_config::Region;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use axum::extract::Query;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use entity::prelude::GameSessions;
use entity::{avatar, game_sessions, matches, user_details, users};
use migration::Expr;
use sea_orm::{ActiveModelTrait, EntityTrait, TransactionTrait};
use sea_orm::{Condition, QueryFilter};
use sea_orm::{DatabaseConnection, Set};

use sea_orm::ColumnTrait;

pub async fn get_boys_handler(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    let boys = users::Entity::find()
        .filter(users::Column::Gender.contains("Male"))
        .all(&db)
        .await;

    match boys {
        Ok(boys) => {
            // Return the list of boys in JSON format
            Json(boys).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get boys from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_girls_handler(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    let boys = users::Entity::find()
        .filter(users::Column::Gender.contains("Female"))
        .all(&db)
        .await;

    match boys {
        Ok(boys) => {
            // Return the list of boys in JSON format
            Json(boys).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get boys from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_score_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(update_score_info): Json<UpdateScoreInfo>,
) -> impl IntoResponse {
    let user_id = update_score_info.user_id;

    // Find user details directly by user_id
    match user_details::Entity::find_by_id(user_id).one(&db).await {
        Ok(Some(details)) => {
            // Update the score in user_details
            let mut active_details: user_details::ActiveModel = details.into();
            active_details.score = Set(update_score_info.score);

            match active_details.update(&db).await {
                Ok(_) => (StatusCode::ACCEPTED, Json("Score updated successfully")).into_response(),
                Err(e) => {
                    eprintln!("Failed to update user score: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("Failed to update score"),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => {
            // User details don't exist yet, create a new record
            let new_details = user_details::ActiveModel {
                user_id: Set(user_id),
                score: Set(update_score_info.score),
                ..Default::default()
            };

            match new_details.insert(&db).await {
                Ok(_) => (StatusCode::CREATED, Json("Score created successfully")).into_response(),
                Err(e) => {
                    eprintln!("Failed to create user details with score: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("Failed to create user details with score"),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to retrieve user details: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to retrieve user details"),
            )
                .into_response()
        }
    }
}

pub async fn get_user_handler(
    Extension(db): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, i32>>,
) -> impl IntoResponse {
    // Extract the user ID from the query parameters
    let id = match params.get("id") {
        Some(id) => *id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json("Missing user ID in query parameters"),
            )
                .into_response();
        }
    };
    let users = users::Entity::find()
        .filter(users::Column::Id.eq(id))
        .one(&db)
        .await;

    match users {
        Ok(users) => {
            // Return the list of boys in JSON format
            Json(users).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get users from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_user_by_id_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(get_user_info): Json<GirlBoyInfoById>,
) -> impl IntoResponse {
    // Attempt to parse the ID and handle potential errors
    let id = match get_user_info.id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json("Invalid ID format: Expected a valid integer"),
            )
                .into_response();
        }
    };

    // Handle potential database connection issues
    let users = users::Entity::find()
        .filter(users::Column::Id.eq(id))
        .one(&db)
        .await;

    match users {
        Ok(Some(users)) => {
            // Return users details in JSON format
            Json(users).into_response()
        }
        // `None` is part of the Rust `Option` enum and is not a variable.
        // It's a keyword used to represent the absence of a value, so the snake_case lint warning can be safely ignored here.
        Ok(None) => {
            // Handle the case where no users is found
            (
                StatusCode::NOT_FOUND,
                Json(format!("No users found with ID: {}", id)),
            )
                .into_response()
        }
        Err(e) => {
            // Handle database query errors
            eprintln!("Failed to retrieve users from the database: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Database query failed"),
            )
                .into_response()
        }
    }
}

pub async fn get_all_users_handler(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let users = users::Entity::find().all(&db).await;

    match users {
        Ok(users) => Json(users).into_response(),
        Err(e) => {
            eprintln!("Failed to get users from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_user_character_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(character_details): Json<CharacterDetails>,
) -> impl IntoResponse {
    let user_id = character_details.user_id;

    // Directly find the user_details record by user_id
    let user_details_result = user_details::Entity::find_by_id(user_id).one(&db).await;

    match user_details_result {
        Ok(Some(details)) => {
            // Update existing record
            let mut model: user_details::ActiveModel = details.into();

            // Update fields
            model.values = Set(character_details.values.unwrap_or_default());
            model.style = Set(character_details.style.unwrap_or_default());
            model.traits = Set(character_details.traits.unwrap_or_default());
            model.commitment = Set(character_details.commitment.unwrap_or_default());
            model.resolution = Set(character_details.resolution.unwrap_or_default());
            model.interests = Set(character_details.interests.unwrap_or_default());

            // Save the changes
            match model.update(&db).await {
                Ok(_) => (
                    StatusCode::ACCEPTED,
                    Json("Character details updated successfully"),
                )
                    .into_response(),
                Err(e) => {
                    eprintln!("Failed to update character details: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("Failed to update character details"),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => {
            // Return 404 instead of creating new details
            (StatusCode::NOT_FOUND, Json("User details not found")).into_response()
        }
        Err(e) => {
            eprintln!("Failed to retrieve user details: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to retrieve user details"),
            )
                .into_response()
        }
    }
}

pub async fn create_match(
    Extension(db): Extension<DatabaseConnection>,
    Json(friend_request): Json<MatchListInfo>,
) -> impl IntoResponse {
    // Begin transaction
    let transaction = db.begin().await;
    if let Err(e) = transaction {
        eprintln!("Failed to begin transaction: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Database transaction error"),
        )
            .into_response();
    }
    let transaction = transaction.unwrap();

    // Create match with pending status using provided IDs
    let match_model = matches::ActiveModel {
        male_id: Set(friend_request.male_id),
        female_id: Set(friend_request.female_id),
        status: Set("PENDING".to_string()),
        ..Default::default()
    };

    // Insert the match and get the newly created match
    let match_result = match_model.insert(&transaction).await;
    if let Err(e) = match_result {
        // Rollback transaction on error
        if let Err(rollback_err) = transaction.rollback().await {
            eprintln!("Failed to rollback transaction: {}", rollback_err);
        }
        eprintln!("Failed to create match in database: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to send friend request"),
        )
            .into_response();
    }

    let created_match = match_result.unwrap();
    let match_id = created_match.id;

    // Create game session with the match ID
    let game_session_model = game_sessions::ActiveModel {
        male_id: Set(friend_request.male_id),
        female_id: Set(friend_request.female_id),
        score: Set(0.0),         // Default score of 0
        game_id: Set(match_id),  // Set game_id to match_id
        match_id: Set(match_id), // Set match_id reference
        ..Default::default()
    };

    // Insert the game session
    let game_session_result = game_session_model.insert(&transaction).await;
    if let Err(e) = game_session_result {
        // Rollback transaction on error
        if let Err(rollback_err) = transaction.rollback().await {
            eprintln!("Failed to rollback transaction: {}", rollback_err);
        }
        eprintln!("Failed to create game session in database: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to create game session"),
        )
            .into_response();
    }

    // Commit the transaction
    if let Err(e) = transaction.commit().await {
        eprintln!("Failed to commit transaction: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to complete match and game session creation"),
        )
            .into_response();
    }

    // Return success response
    (
        StatusCode::ACCEPTED,
        Json("Friend request sent and game session created"),
    )
        .into_response()
}

pub async fn get_girl_request(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_info): Json<GetUserInfo>,
) -> impl IntoResponse {
    let male_id = user_info.id;

    // Find all pending match requests for this male user
    let pending_matches = matches::Entity::find()
        .filter(matches::Column::MaleId.eq(male_id))
        .filter(matches::Column::Status.eq("PENDING"))
        .all(&db)
        .await;

    match pending_matches {
        Ok(matches) => {
            // Return the list of pending matches in JSON format
            Json(matches).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get pending matches from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_contest_matches(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_info): Json<GetUserInfo>,
) -> impl IntoResponse {
    let female_id = user_info.id;

    // Find all matches with CONTEST status for this female user
    // Join with GameSessions to get the score for ordering
    let contest_matches = matches::Entity::find()
        .filter(matches::Column::FemaleId.eq(female_id))
        .filter(matches::Column::Status.eq("CONTEST"))
        // .find_with_related(GameSessions)
        .all(&db)
        .await;

    match contest_matches {
        Ok(matches) => {
            // Return the list of contest matches in JSON format
            Json(matches).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get contest matches from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
pub async fn get_accepted_matches(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_info): Json<GetUserInfo>,
) -> impl IntoResponse {
    let user_id = user_info.id;

    // Find all matches with ACCEPTED status where the user is either male or female
    let accepted_matches = matches::Entity::find()
        .filter(
            Condition::any()
                .add(matches::Column::MaleId.eq(user_id))
                .add(matches::Column::FemaleId.eq(user_id)),
        )
        .filter(matches::Column::Status.eq("ACCEPTED"))
        .all(&db)
        .await;

    match accepted_matches {
        Ok(matches) => {
            // Return the list of accepted matches in JSON format
            Json(matches).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get accepted matches from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn change_status_to_contest(
    Extension(db): Extension<DatabaseConnection>,
    Json(boy_id): Json<GetUserInfo>,
) -> impl IntoResponse {
    // Find the match with the given ID
    let match_record = matches::Entity::find_by_id(boy_id.id).one(&db).await;

    match match_record {
        Ok(Some(record)) => {
            let mut match_model: matches::ActiveModel = record.into();

            // Update the status to "contest"
            match_model.status = Set("CONTEST".to_string());

            // Save the updated record
            let updated = match_model.update(&db).await;

            match updated {
                Ok(_) => (
                    StatusCode::ACCEPTED,
                    Json("Match status updated to contest successfully"),
                )
                    .into_response(),

                Err(e) => {
                    eprintln!("Failed to update match status: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("Failed to update match status"),
                    )
                        .into_response()
                }
            }
        }

        Ok(None) => (StatusCode::NOT_FOUND, Json("Match not found")).into_response(),

        Err(e) => {
            eprintln!("Failed to query match from the database: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to query match"),
            )
                .into_response()
        }
    }
}
pub async fn get_boys_game_scores_for_girl(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_info): Json<GetUserInfo>,
) -> impl IntoResponse {
    let female_id = user_info.id;

    // Find all game sessions where the specified user is the female participant
    let game_sessions = GameSessions::find()
        .filter(game_sessions::Column::FemaleId.eq(female_id))
        .all(&db)
        .await;

    match game_sessions {
        Ok(sessions) => {
            // Structure the response to include male IDs and their scores
            let boy_scores: Vec<BoyScoreInfo> = sessions
                .into_iter()
                .map(|session| BoyScoreInfo {
                    male_id: session.male_id,
                    game_id: session.game_id,
                    match_id: session.match_id,
                    score: session.score, // Default to 0 if score is null
                })
                .collect();

            // Return the scores in JSON format
            Json(boy_scores).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get game sessions from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn reject_match(
    Extension(db): Extension<DatabaseConnection>,
    Json(match_info): Json<GetUserInfo>,
) -> impl IntoResponse {
    // Update the match status to "rejected" where either user is involved
    let status_update_result = matches::Entity::update_many()
        .filter(
            sea_orm::Condition::any()
                .add(matches::Column::MaleId.eq(match_info.id))
                .add(matches::Column::FemaleId.eq(match_info.id)),
        )
        .filter(matches::Column::Status.eq("PENDING"))
        .col_expr(matches::Column::Status, Expr::value("REJECTED".to_string()))
        .exec(&db)
        .await;

    match status_update_result {
        Ok(_) => (StatusCode::OK, Json("Match successfully rejected")).into_response(),
        Err(e) => {
            eprintln!("Failed to update match status to rejected: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to reject match"),
            )
                .into_response()
        }
    }
}
pub async fn update_game_session_score(
    Extension(db): Extension<DatabaseConnection>,
    Json(score_info): Json<GameSession>,
) -> impl IntoResponse {
    let game_id = score_info.id;

    // Find the game session by game_id
    match GameSessions::find()
        .filter(game_sessions::Column::GameId.eq(game_id))
        .one(&db)
        .await
    {
        Ok(Some(session)) => {
            let mut session_model: game_sessions::ActiveModel = session.into();

            // Update the score
            session_model.score = Set(score_info.score);

            // Attempt to update the game session score
            match session_model.update(&db).await {
                Ok(_) => (
                    StatusCode::OK,
                    Json(format!(
                        "Game session score updated successfully for game ID: {}",
                        game_id
                    )),
                )
                    .into_response(),

                Err(e) => {
                    eprintln!("Failed to update game session score: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("Failed to update game session score"),
                    )
                        .into_response()
                }
            }
        }

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(format!("Game session with game ID {} not found", game_id)),
        )
            .into_response(),

        Err(e) => {
            eprintln!("Database query error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Database query failed"),
            )
                .into_response()
        }
    }
}

pub async fn get_user_avatar(
    Extension(db): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, i32>>,
) -> Result<Json<String>, StatusCode> {
    let user_id = match params.get("id") {
        Some(id) => *id,
        None => {
            eprintln!("id parameter is missing.");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match avatar::Entity::find()
        .filter(avatar::Column::UserId.eq(user_id))
        .one(&db)
        .await
    {
        Ok(Some(avatar)) => Ok(Json(avatar.object_key)),
        Ok(None) => {
            eprintln!("No avatar found for user_id: {}", user_id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user_avatar(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<String>, StatusCode> {
    let config = aws_config::from_env()
        .region(Region::new("ap-south-1"))
        .load()
        .await;
    let client = Client::new(&config);

    let bucket_name = "affinitys3";

    let username = match params.get("username") {
        Some(username) => username,
        None => {
            eprintln!("Username parameter is missing.");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let filename = match params.get("filename") {
        Some(username) => username,
        None => {
            eprintln!("filename parameter is missing.");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let avatar_object_key = format!("avatars/{}/{}", username, filename);

    // Generate the presigned URL
    let presigned_request = match client
        .put_object()
        .bucket(bucket_name)
        .key(avatar_object_key.clone())
        .presigned(PresigningConfig::expires_in(Duration::from_secs(6000)).unwrap())
        .await
    {
        Ok(request) => request,
        Err(e) => {
            eprintln!("Failed to create presigned URL: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("Object URI: {}", presigned_request.uri());

    Ok(Json(presigned_request.uri().to_string()))
}
