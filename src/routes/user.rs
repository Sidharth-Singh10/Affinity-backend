use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait};

pub fn user_routes() -> Router {
    Router::new().route("/getallusers", get(get_all_users_handler))
}

async fn get_all_users_handler(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    let users = user::Entity::find().all(&db).await;

    match users {
        Ok(users) => Json(users).into_response(),
        Err(e) => {
            eprintln!("Failed to get users from the database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
