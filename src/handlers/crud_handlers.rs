use crate::model::{ContestInfo, FriendListInfo, Matched};
use crate::model::{CharacterDetails, GetUserInfo, GirlBoyInfo, GirlBoyInfoById, UpadateScoreInfo};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{friend_list, matched};
use entity::user;
use migration::Expr;
use sea_orm::{QueryFilter, QueryOrder};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{DatabaseConnection, Set};

use sea_orm::ColumnTrait;

pub async fn get_boys_handler(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    let boys = user::Entity::find()
        .filter(user::Column::Gender.contains("Male"))
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
    let boys = user::Entity::find()
        .filter(user::Column::Gender.contains("Female"))
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
    Json(update_score_info): Json<UpadateScoreInfo>,
) -> impl IntoResponse {
    let email = update_score_info.email.clone();

    match user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(&db)
        .await
    {
        Ok(Some(user)) => {
            let mut active_user: user::ActiveModel = user.into();
            active_user.score = Set(update_score_info.score);
            match active_user.update(&db).await {
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
        Ok(_) => (StatusCode::NOT_FOUND, Json("User not found")).into_response(),
        Err(e) => {
            eprintln!("Failed to retrieve user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to retrieve user"),
            )
                .into_response()
        }
    }
}

pub async fn get_user_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(get_user_info): Json<GetUserInfo>,
) -> impl IntoResponse {
    let email = get_user_info.email;
    let user = user::Entity::find()
        .filter(user::Column::Email.contains(email))
        .one(&db)
        .await;

    match user {
        Ok(user) => {
            // Return the list of boys in JSON format
            Json(user).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_user_by_id_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(get_user_info): Json<GirlBoyInfoById>,
) -> impl IntoResponse {
    let id: i32 = get_user_info.id.parse().unwrap();
    let user = user::Entity::find()
        .filter(user::Column::Id.eq(id))
        .one(&db)
        .await;

    match user {
        Ok(user) => {
            // Return the list of boys in JSON format
            Json(user).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_all_users_handler(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let users = user::Entity::find().all(&db).await;

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
    let email = character_details.email;

    let user = user::Entity::find()
        .filter(user::Column::Email.contains(email))
        .one(&db)
        .await;

    match user {
        Ok(user) => {
            let mut user: user::ActiveModel = user.unwrap().into();
            // user.score = Set(character_details.score.to_owned());
            user.values = Set(character_details.values);
            user.style = Set(character_details.style);
            user.traits = Set(character_details.traits);
            user.commitment = Set(character_details.commitment);
            user.resolution = Set(character_details.resolution);
            user.interests = Set(character_details.interests);

            let user = user.update(&db).await;

            match user {
                Ok(_) => StatusCode::ACCEPTED.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_matched_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(info): Json<GirlBoyInfo>,
) -> impl IntoResponse {
    let email = info.email;

    let user = matched::Entity::find()
        .filter(
            matched::Column::BoyEmailId
                .contains(&email)
                .or(matched::Column::GirlEmailId.contains(email)),
        )
        .all(&db)
        .await;

    match user {
        Ok(user) => Json(user).into_response(),
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn add_friend_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(friend_list_info): Json<FriendListInfo>,
) -> impl IntoResponse {
    let girl_email = friend_list_info.girl_email;
    let boy_email = friend_list_info.boy_email;

    let friend_list_model = friend_list::ActiveModel {
        girl_email_id: Set(girl_email),
        boy_email_id: Set(boy_email),
        flag: Set("0".to_string()),
        contest_score: Set("0".to_string()),
        ..Default::default()
    };

    match friend_list_model.insert(&db).await {
        Ok(_) => (StatusCode::ACCEPTED, Json("Friend added successfully")).into_response(),
        Err(e) => {
            eprintln!("Failed to insert user into the database: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed add friend")).into_response()
        }
    }
}

pub async fn get_girl_request_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(boy_info): Json<GirlBoyInfo>,
) -> impl IntoResponse {
    let boy_email = boy_info.email;

    let girl_list = friend_list::Entity::find()
        .filter(friend_list::Column::BoyEmailId.contains(boy_email))
        .filter(friend_list::Column::Flag.contains("0"))
        .all(&db)
        .await;

    match girl_list {
        Ok(girl_list) => {
            // Return the list of boys in JSON format
            Json(girl_list).into_response()
        }

        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_accepted_boys_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(boy_info): Json<GirlBoyInfo>,
) -> impl IntoResponse {
    let girl_email = boy_info.email;

    let boy_list = friend_list::Entity::find()
        .filter(friend_list::Column::GirlEmailId.contains(girl_email))
        .filter(friend_list::Column::Flag.contains("1"))
        .order_by_desc(friend_list::Column::ContestScore)
        .all(&db)
        .await;

    match boy_list {
        Ok(boy_list) => {
            // Return the list of boys in JSON format
            Json(boy_list).into_response()
        }

        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn change_flag_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(boy_info): Json<GirlBoyInfo>,
) -> impl IntoResponse {
    let email: i32 = boy_info.email.parse().unwrap();

    let user = friend_list::Entity::find()
        .filter(friend_list::Column::Id.eq(email))
        .one(&db)
        .await;

    match user {
        Ok(user) => {
            let mut user: friend_list::ActiveModel = user.unwrap().into();
            // user.score = Set(character_details.score.to_owned());

            user.flag = Set("1".to_string());

            let user = user.update(&db).await;

            match user {
                Ok(_) => (StatusCode::ACCEPTED, Json("Flag Updated successfully")).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to update flag"),
                )
                    .into_response(),
            }
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to update flag"),
            )
                .into_response()
        }
    }
}

pub async fn create_matched_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(matched): Json<Matched>,
) -> impl IntoResponse {
    let boy_email = matched.boy_email;
    let girl_email = matched.girl_email;

    let list = matched::ActiveModel {
        boy_email_id: Set(boy_email.clone()),
        girl_email_id: Set(girl_email.clone()),
        ..Default::default()
    };

    if let Err(e) = list.insert(&db).await {
        println!("{}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to add friend"),
        )
            .into_response();
    }

    // Update flag to 0 for all matching boys where the flag is 1
    let flag_update_result = friend_list::Entity::update_many()
        .filter(friend_list::Column::GirlEmailId.eq(girl_email.clone()))
        .filter(friend_list::Column::Flag.eq("1".to_string()))
        .col_expr(friend_list::Column::Flag, Expr::value("0".to_string()))
        .exec(&db)
        .await;

    // Handle the result of updating the flags
    if let Err(_) = flag_update_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to update flags to 0"),
        )
            .into_response();
    }

    // Find the user from the friend_list
    let user_result = friend_list::Entity::find()
        .filter(friend_list::Column::BoyEmailId.eq(boy_email.clone()))
        .filter(friend_list::Column::GirlEmailId.eq(girl_email.clone()))
        .one(&db)
        .await;

    // Handle the result of finding the user
    match user_result {
        Ok(Some(user)) => {
            let mut user: friend_list::ActiveModel = user.into();
            user.flag = Set("2".to_string());

            if let Err(_) = user.update(&db).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to update flag to 2"),
                )
                    .into_response();
            }

            // Success response after both insert and update
            (
                StatusCode::ACCEPTED,
                Json("Friend added and flag updated successfully"),
            )
                .into_response()
        }
        Ok(_) => {
            // User not found
            (
                StatusCode::NOT_FOUND,
                Json("User not found for the provided emails"),
            )
                .into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to retrieve user from the database"),
            )
                .into_response()
        }
    }
}




pub async fn reject_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(matched): Json<Matched>,
) -> impl IntoResponse {
    let boy_email = matched.boy_email;
    let girl_email = matched.girl_email;

    let flag_update_result = friend_list::Entity::update_many()
        .filter(friend_list::Column::GirlEmailId.eq(girl_email.clone()))
        .filter(friend_list::Column::BoyEmailId.contains(boy_email))
        .filter(friend_list::Column::Flag.eq("0".to_string()))
        .col_expr(friend_list::Column::Flag, Expr::value("2".to_string()))
        .exec(&db)
        .await;

    if let Err(_) = flag_update_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to update flags to 0"),
        )
            .into_response();
    } else {
        return (StatusCode::OK, Json("SUCESS")).into_response();
    }
}

pub async fn update_contest_score_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(contest_info): Json<ContestInfo>,
) -> impl IntoResponse {
    let id: i32 = contest_info.id.parse().unwrap();

    let user = friend_list::Entity::find()
        .filter(friend_list::Column::Id.eq(id))
        .one(&db)
        .await;

    match user {
        Ok(user) => {
            let mut user: friend_list::ActiveModel = user.unwrap().into();

            user.contest_score = Set(contest_info.contestscore);
            let user = user.update(&db).await;

            match user {
                Ok(_) => StatusCode::OK.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }

        Err(e) => {
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


