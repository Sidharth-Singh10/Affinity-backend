use axum::{
    body::Body,
    extract::Request,
    http::{self, Response},
    middleware::Next,
};
use reqwest::StatusCode;

use crate::handlers::auth_handlers::decode_jwt;

pub async fn authorization_middleware(
    mut req: Request<Body>, 
    next: Next
) -> Result<Response<Body>, (StatusCode, &'static str)> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => match header.to_str() {
            Ok(header_str) => header_str,
            Err(_) => return Err((StatusCode::FORBIDDEN, "Empty header is not allowed")),
        },
        None => return Err((StatusCode::FORBIDDEN, "Please add the JWT token to the header")),
    };

    let mut header_parts = auth_header.split_whitespace();
    let (_bearer, token) = (header_parts.next(), header_parts.next());

    let token_data = match token {
        Some(token) => match decode_jwt(token.to_string()) {
            Ok(data) => data,
            Err(_) => return Err((StatusCode::UNAUTHORIZED, "Unable to decode token")),
        },
        None => return Err((StatusCode::FORBIDDEN, "Missing token")),
    };

    req.extensions_mut().insert(token_data.claims.sub.clone());

    Ok(next.run(req).await)
}