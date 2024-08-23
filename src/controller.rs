use std::path::PathBuf;

use crate::{
    bcrypts::{hash_password, verify_password},
    // db::create_user,
    model::{
        CharacterDetails, Claims, FriendListInfo, GetUserInfo, GirlBoyInfo, LoginInfo, Matched, SignUpInfo, UpadateScoreInfo
    },
    utils::scripts::{compare_with_answer_file, docker_run},
};
use axum::{
    extract::Multipart,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use entity::{friend_list, matched};

use chrono::Utc;
use cookie::Cookie;
use entity::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{DatabaseConnection, Set};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;

use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

pub async fn signup_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(signup_info): Json<SignUpInfo>,
) -> impl IntoResponse {
    // Check if user already exists

    let email_exists = user::Entity::find()
        .filter(user::Column::Email.contains(&signup_info.email))
        .one(&db)
        .await;

    if let Ok(Some(_)) = email_exists {
        eprintln!("User with email {} already exists!", signup_info.email);
        return Err((StatusCode::CONFLICT));
    }

    let username = signup_info.username;
    let email = signup_info.email;
    let first_name = signup_info.first_name;
    let last_name = signup_info.last_name;
    let hashed_password = match hash_password(&signup_info.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password could not be hashed -> {}", e);
            return (Err(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    let age = signup_info.age;

    // Handling other optional fields
    let gender = signup_info.gender;

    let location = signup_info
        .location
        .unwrap_or_else(|| "Unknown".to_string());

    let openness = signup_info
        .openness
        .unwrap_or_else(|| "Neutral".to_string());

    let interest = signup_info.interests.unwrap_or_else(|| "None".to_string());

    let exp_qual = signup_info.exp_qual.unwrap_or_else(|| "None".to_string());

    let relation_type = signup_info
        .relation_type
        .unwrap_or_else(|| "Unspecified".to_string());

    let social_habits = signup_info
        .social_habits
        .unwrap_or_else(|| "Unspecified".to_string());

    let past_relations = signup_info
        .past_relations
        .unwrap_or_else(|| "Unspecified".to_string());

    let image_url: String = signup_info.image_url.unwrap_or_else(|| "".to_string());
    let score = signup_info.score;

    let user_model = user::ActiveModel {
        user_name: Set(username),
        email: Set(email),
        first_name: Set(Some(first_name)),
        last_name: Set(Some(last_name)),
        password: Set(hashed_password),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        age: Set(age),
        gender: Set(gender),
        location: Set(Some(location)),
        openness: Set(Some(openness)),
        interests: Set(Some(interest)),
        exp_qual: Set(Some(exp_qual)),
        relation_type: Set(Some(relation_type)),
        social_habits: Set(Some(social_habits)),
        past_relations: Set(Some(past_relations)),
        values: Set(None),
        style: Set(None),
        traits: Set(None),
        commitment: Set(None),
        resolution: Set(None),
        image_url: Set(image_url),
        score: Set(score),
        ..Default::default()
    };

    match user_model.insert(&db).await {
        Ok(inserted_user) => {
            let created_user = user::Entity::find_by_id(inserted_user.id)
                .one(&db)
                .await
                .unwrap();

            if let Some(user) = created_user {
                return Ok(StatusCode::ACCEPTED);
            } else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        Err(e) => {
            eprintln!("Failed to insert user into the database: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn login_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(login_info): Json<LoginInfo>,
) -> impl IntoResponse {
    let email = &login_info.email;
    let password = &login_info.password;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(&db)
        .await
        .unwrap();

    let is_valid;
    if let Some(ref user) = user {
        is_valid = verify_password(password, &user.password)
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    println!("{}", is_valid);

    Ok(if is_valid {
        let claims = Claims {
            sub: email.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("696969".as_ref()),
        ) {
            Ok(tok) => tok,
            Err(e) => {
                eprintln!("Error generating token {} !!!", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let mut cookie = Cookie::new("token", token);
        cookie.set_http_only(true);

        let mut headers = HeaderMap::new();
        headers.insert("Set-Cookie", cookie.to_string().parse().unwrap());
        return Ok((headers, Json(user)).into_response());

        // Ok(Json(LoginResponse{token}));
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    })
}

// pub async fn decode_jwt(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
//     if let Some(auth_header) = header_map.get("Authorization") {
//         if let Ok(auth_header_str) = auth_header.to_str() {
//             if auth_header_str.starts_with("Bearer ") {
//                 let token = auth_header_str.trim_start_matches("Bearer ").to_string();

//                 match decode::<Claims>(
//                     &token,
//                     &DecodingKey::from_secret("696969".as_ref()),
//                     &Validation::default(),
//                 ) {
//                     Ok(token_data) => {
//                         let email = token_data.claims.sub;
//                         return Ok(Json(email));
//                     }
//                     Err(e) => {
//                         eprintln!("Error decoding token {} !!!", e);
//                         return Err(StatusCode::UNAUTHORIZED);
//                     }
//                 }
//             }
//         }
//     }

//     Err(StatusCode::UNAUTHORIZED)
// }

// pub async fn decode_jwt(cookies: TypedHeader<Cookie>) -> Result<Json<String>, StatusCode> {
//     // Retrieve the token from the cookies
//     if let Some(token_cookie) = cookies.get("token") {
//         let token = token_cookie.to_string();

//         match decode::<Claims>(
//             &token,
//             &DecodingKey::from_secret("696969".as_ref()),
//             &Validation::default(),
//         ) {
//             Ok(token_data) => {
//                 let email = token_data.claims.sub;
//                 return Ok(Json(email));
//             }
//             Err(e) => {
//                 eprintln!("Error decoding token {} !!!", e);
//                 return Err(StatusCode::UNAUTHORIZED);
//             }
//         }
//     }

//     Err(StatusCode::UNAUTHORIZED)
// }

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
        Ok(None) => (StatusCode::NOT_FOUND, Json("User not found")).into_response(),
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
        .filter(friend_list::Column::BoyEmailId.contains(girl_email))
        .filter(friend_list::Column::Flag.contains("1"))
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
    let email:i32 = boy_info.email.parse().unwrap();

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


pub async fn create_matched_handler(Extension(db): Extension<DatabaseConnection>, Json(matched):Json<Matched>) -> impl IntoResponse
{

    let boy_email = matched.boy_email;
    let girl_email = matched.girl_email;

    let list = matched::ActiveModel{
        boy_email_id: Set(boy_email),
        girl_email_id: Set(girl_email),
    ..Default::default()
    };

    match list.insert(&db).await {
        Ok(_) => (StatusCode::ACCEPTED, Json("Friend added successfully")).into_response(),
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed add friend")).into_response()
        }
    }

}

pub async fn get_matched_handler (Extension(db): Extension<DatabaseConnection>, Json(info):Json<GirlBoyInfo>) -> impl IntoResponse
{

    let email = info.email;

    let user = matched::Entity::find()
        .filter(
            matched::Column::BoyEmailId.contains(&email)
            .or(matched::Column::GirlEmailId.contains(email))
        )
        .all(&db)
        .await;

    match user {
        Ok(user) => {
            Json(user).into_response()
        }
        Err(e) => {
            // Log the error and return a 500 status code
            eprintln!("Failed to get user from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

}



pub async fn code_handler(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field
            .file_name()
            .unwrap_or("default_filename.txt")
            .to_string();
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());

        //defingn path
        let dir: PathBuf = "./uploads".into();

        if let Err(err) = create_dir_all(&dir).await {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
        }

        let filepath = dir.join(filename.clone());

        let mut file = File::create(&filepath)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        file.write_all(&data)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let add = format!("./uploads/{}", filename);

        let args = [add.as_str()];

        match docker_run(&args).await {
            Ok(stdout) => match compare_with_answer_file(&stdout, &filename).await {
                Ok(true) => {
                    println!("The output matches the answer file.");
                    return Ok("AC");
                }
                Ok(false) => {
                    println!("The output does NOT match the answer file.");
                    return Ok("WA");
                }
                Err(e) => {
                    eprintln!("Error comparing with answer file: {}", e);
                    return Ok("Error comparing with answer file");
                }
            },
            Err(e) => {
                eprintln!("Error running script: {}", e);
                return Ok("Error running docker");
            }
        }
    }

    Ok("File uploaded successfully")
    // Define the path where you want to save the file

    // Save the file

    // Ok("File uploaded successfully".to_string())
}
