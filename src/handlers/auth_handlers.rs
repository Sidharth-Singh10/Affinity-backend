// auth_handler.rs
use std::collections::HashMap;

use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use chrono::Utc;
use cookie::Cookie;
use entity::{avatar, pass_reset, user_details, users};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use reqwest::header::AUTHORIZATION;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

use crate::{
    bcrypts::{hash_password, verify_password},
    configs::totp_config::totp,
    model::{Claims, LoginInfo, LoginResponse, SignUpInfo},
    utils::{
        constants::{JWT_SECRET, PASS_RESET_LINK},
        hash_token::{generate_secure_token, verify_token},
        pass_reset::PassReset,
        verify_email::EmailOTP,
    },
};

pub async fn signup_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(signup_info): Json<SignUpInfo>,
) -> impl IntoResponse {
    // Check if users already exists
    let email_exists = users::Entity::find()
        .filter(users::Column::Email.contains(&signup_info.email))
        .one(&db)
        .await;

    if let Ok(Some(_)) = email_exists {
        eprintln!("users with email {} already exists!", signup_info.email);
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
    let gender = signup_info.gender;

    // Create user model according to our database schema
    let user_model = users::ActiveModel {
        username: Set(username.clone()),
        first_name: Set(first_name),
        last_name: Set(last_name),
        age: Set(age),
        email: Set(email),
        password: Set(hashed_password),
        gender: Set(gender),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    // Starting a transaction
    let txn = match db.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Insert the user
    let inserted_user = match user_model.insert(&txn).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Failed to insert user into the database: {}", e);
            let _ = txn.rollback().await;
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Create user details model
    let user_details_model = user_details::ActiveModel {
        user_id: Set(inserted_user.id), // Reference to the user ID
        location: Set(signup_info.location.unwrap_or_default()),
        openness: Set(signup_info.openness.unwrap_or_default()),
        interests: Set(signup_info.interests.unwrap_or_default()),
        exp_qual: Set(signup_info.exp_qual.unwrap_or_default()),
        relation_type: Set(signup_info.relation_type.unwrap_or_default()),
        social_habits: Set(signup_info.social_habits.unwrap_or_default()),
        past_relations: Set(signup_info.past_relations.unwrap_or_default()),
        values: Set(signup_info.values.unwrap_or_default()),
        style: Set(signup_info.style.unwrap_or_default()),
        traits: Set(signup_info.traits.unwrap_or_default()),
        commitment: Set(signup_info.commitment.unwrap_or_default()),
        resolution: Set(signup_info.resolution.unwrap_or_default()),
        image_url: Set(signup_info.image_url.clone().unwrap_or_default()),
        bio: Set(signup_info.bio.clone().unwrap_or_default()),
        score: Set(signup_info.score),
    };

    // Insert user details
    if let Err(e) = user_details_model.insert(&txn).await {
        eprintln!("Failed to insert user details into the database: {}", e);
        let _ = txn.rollback().await;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Create avatar if image_url is provided
    if let Some(image_url) = signup_info.image_url.clone() {
        if !image_url.is_empty() {
            let avatar_model = avatar::ActiveModel {
                user_id: Set(inserted_user.id), // Reference to the user ID
                object_key: Set(image_url),
                ..Default::default()
            };

            if let Err(e) = avatar_model.insert(&txn).await {
                eprintln!("Failed to insert avatar into the database: {}", e);
                let _ = txn.rollback().await;
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    // Commit the transaction
    match txn.commit().await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("Failed to commit transaction: {}", e);
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

    // Find user by email
    let user_result = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(&db)
        .await;

    // Handle database errors
    let user = match user_result {
        Ok(maybe_user) => {
            if let Some(user) = maybe_user {
                user
            } else {
                return Err(StatusCode::NOT_FOUND);
            }
        }
        Err(e) => {
            eprintln!("Database error when finding user: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !verify_password(password, &user.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Fetch user details to include in response
    let user_details = user_details::Entity::find_by_id(user.id)
        .one(&db)
        .await
        .unwrap_or(None);

    // Create JWT token
    let claims = Claims {
        sub: user.id.to_string(), // Using user ID instead of email for better security
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    ) {
        Ok(tok) => tok,
        Err(e) => {
            eprintln!("Error generating token: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Create response with token in cookie and header
    let bearer_token = format!("Bearer {}", token);
    let mut cookie = Cookie::new("token", bearer_token.clone());
    cookie.set_http_only(true);
    cookie.set_path("/");
    // For production, also set cookie.set_secure(true) and SameSite

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", cookie.to_string().parse().unwrap());
    // Also add token to Authorization header for API usage
    headers.insert(AUTHORIZATION, bearer_token.parse().unwrap());

    // Create response with user data and details
    let response = LoginResponse {
        user,
        user_details,
        token,
    };

    Ok((headers, Json(response)).into_response())
}

// Modified send_pass_reset_handler
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

    if let Some(users) = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(&db)
        .await
        .unwrap()
    {
        let (token, hmac) = generate_secure_token();

        let token_expiry = Utc::now() + chrono::Duration::hours(1);
        let token_expiry_timestamp = token_expiry.timestamp();

        let username = users.username;
        let reset_link = format!("{}?token={}", *PASS_RESET_LINK, token);
        let sent_email = PassReset::new(username.to_string(), reset_link, email.to_string());

        let _ = sent_email.send_pass_reset().await;

        let pass_reset_model = pass_reset::ActiveModel {
            user_id: Set(users.id),
            token: Set(hmac), // Store the HMAC instead of the plain token
            token_expiry: Set(token_expiry_timestamp),
        };

        match pass_reset_model.insert(&db).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to insert pass_reset into the database: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    Ok(Json(format!("Password reset link sent to {}", email)))
}

pub async fn new_password_handler(
    Extension(db): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<String>, StatusCode> {
    if let (Some(reset_token), Some(new_password)) = (params.get("token"), params.get("password")) {
        let txn = db
            .begin()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Fetch all password reset entries
        let all_resets = pass_reset::Entity::find()
            .all(&txn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Find the matching token
        let matched_reset = all_resets
            .into_iter()
            .find(|reset| verify_token(reset_token, &reset.token));

        if let Some(reset) = matched_reset {
            // Check token expiry
            let current_time = Utc::now().timestamp();
            if reset.token_expiry < current_time {
                txn.rollback()
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                return Err(StatusCode::BAD_REQUEST); // Token has expired
            }

            // Delete all tokens for the users
            pass_reset::Entity::delete_many()
                .filter(pass_reset::Column::UserId.eq(reset.user_id))
                .exec(&txn)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Update the users's password
            let user_model = users::Entity::find_by_id(reset.user_id)
                .one(&txn)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::NOT_FOUND)?;

            let mut users: users::ActiveModel = user_model.into();

            // Hash the new password before storing
            let hashed_password =
                hash_password(new_password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            users.password = Set(hashed_password);
            users
                .update(&txn)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Commit the transaction
            txn.commit()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Json("Password updated successfully".to_string()))
        } else {
            // Token not found or invalid
            txn.rollback()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Err(StatusCode::BAD_REQUEST)
        }
    } else {
        // One or both parameters are missing
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

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}
