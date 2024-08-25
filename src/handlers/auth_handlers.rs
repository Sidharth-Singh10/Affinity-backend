use axum::{http::{HeaderMap, StatusCode}, response::IntoResponse, Extension, Json};
use chrono::Utc;
use cookie::Cookie;
use entity::user;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set,ActiveModelTrait,ColumnTrait};
use uuid::Uuid;

use crate::{bcrypts::{hash_password, verify_password}, model::{Claims, LoginInfo, SignUpInfo}};

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
        return Err(StatusCode::CONFLICT);
    }

    let username = signup_info.username;
    let email = signup_info.email;
    let first_name = signup_info.first_name;
    let last_name = signup_info.last_name;
    let hashed_password = match hash_password(&signup_info.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password could not be hashed -> {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
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

            if let Some(_) = created_user {
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




pub fn decode_jwt(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret("696969".as_ref()),
                    &Validation::default(),
                ) {
                    Ok(token_data) => {
                        let email = token_data.claims.sub;
                        return Ok(Json(email));
                    }
                    Err(e) => {
                        eprintln!("Error decoding token {} !!!", e);
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}