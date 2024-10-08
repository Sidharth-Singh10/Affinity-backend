use std::collections::HashMap;

use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use chrono::Utc;
use cookie::Cookie;
use entity::{pass_reset, user};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{
    bcrypts::{hash_password, verify_password},
    configs::totp_config::totp,
    model::{Claims, LoginInfo, SignUpInfo},
    utils::{
        constants::{JWT_SECRET, PASS_RESET_LINK},
        pass_reset::PassReset,
        verify_email::EmailOTP,
    },
};

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

            if created_user.is_some() {
                Ok(StatusCode::ACCEPTED)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(e) => {
            eprintln!("Failed to insert user into the database: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
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
            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
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

pub async fn send_pass_reset_handler(
    Extension(db): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<String>, StatusCode> {
    let email = match params.get("email") {
        Some(email) => email,
        None => {
            eprintln!("No email provided in the request");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    if let Some(user) = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(&db)
        .await
        .unwrap()
    {
        let token: String =
            rand::Rng::sample_iter(rand::thread_rng(), &rand::distributions::Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();

        // let hashed_token = match hash_password(token.as_str()) {
        //     Ok(hash) => hash,
        //     Err(e) => {
        //         eprintln!("Password could not be hashed -> {}", e);
        //         return Err(StatusCode::INTERNAL_SERVER_ERROR);
        //     }
        // };

        let token_expiry = Utc::now() + chrono::Duration::hours(1); // Adds 1 hour to the current time
        let token_expiry_timestamp = token_expiry.timestamp(); // Converts to i64 (seconds since Unix epoch)

        let username = user.user_name;
        let reset_link = format!("{}?token={}", *PASS_RESET_LINK, token);
        let sent_email = PassReset::new(username.to_string(), reset_link, email.to_string());

        let _ = sent_email.send_pass_reset().await;

        let pass_reset_model = pass_reset::ActiveModel {
            user_id: Set(user.id),
            token: Set(token),
            token_expiry: Set(token_expiry_timestamp)
        };

        match pass_reset_model.insert(&db).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to insert pass_reset into the database: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    // Temporarily return a success message
    Ok(Json(format!("Password reset link sent to {}", email)))
}

pub async fn new_password_handler(
    Extension(db): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<String>, StatusCode> {
    if let (Some(reset_token), Some(hashed_password)) =
        (params.get("token"), params.get("password"))
    {
        // let hashed_reset_token = match hash_password(reset_token) {
        //     Ok(hashed) => hashed,                                    // Successfully hashed
        //     Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR), // Handle error
        // };

        // println!("{}", hashed_reset_token);

        let user = pass_reset::Entity::find()
            .filter(pass_reset::Column::Token.contains(reset_token))
            .one(&db)
            .await
            .unwrap();

        let user_id = match user {
            Some(entity) => entity.user_id,
            None => return Err(StatusCode::NOT_FOUND),
        };

        let txn = db.begin().await.unwrap();

        let tokens = pass_reset::Entity::find()
            .filter(pass_reset::Column::UserId.eq(user_id))
            .all(&txn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let matched_token = tokens.into_iter().find(|row| row.token == *reset_token);

        if let Some(matched_token) = matched_token {
            // Check token expiry
            let current_time = Utc::now().timestamp();
            if matched_token.token_expiry < current_time {
                txn.rollback().await.unwrap();
                return Err(StatusCode::BAD_REQUEST); // Token has expired
            }

            // Delete all tokens for the user
            pass_reset::Entity::delete_many()
                .filter(pass_reset::Column::UserId.eq(user_id))
                .exec(&txn)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Update the user's password
            let user_model = user::Entity::find_by_id(user_id).one(&txn).await.unwrap();

            let mut user: user::ActiveModel = user_model.unwrap().into();

            user.password = Set(hashed_password.to_owned());
            user.update(&txn)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Commit the transaction
            txn.commit()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Json("Password updated successfully".to_string()))
        } else {
            // Token not found
            txn.rollback().await.unwrap();
            Err(StatusCode::BAD_REQUEST)
        }
    } else {
        // One or both are missing
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn otp_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<String>, StatusCode> {
    let otp_result2 = match totp() {
        Ok(otp) => otp,
        Err(err) => {
            eprintln!("Error generating TOTP: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let otp = params.get("otp");

    match otp {
        Some(otp) => {
            let check = otp_result2.check_current(otp).unwrap();
            if check {
                // (StatusCode::OK, Json("otp verified successfully"))
                Ok(Json("Otp verified".to_string()))
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        None => {
            // Generate the current TOTP
            match otp_result2.generate_current() {
                Ok(otp) => {
                    // Return the OTP as a JSON response with status 200 OK
                    let username = params.get("username").unwrap();
                    let to = params.get("email").unwrap();

                    let sent_email =
                        EmailOTP::new(username.to_string(), otp.to_string(), to.to_string());

                    let _ = sent_email.send_otp().await;

                    Ok(Json("Otp sent successfully".to_string()))
                }
                Err(_) => {
                    // If there's an error generating the OTP, return 500 Internal Server Error
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    }
}

pub fn _decode_jwt(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(JWT_SECRET.as_ref()),
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
