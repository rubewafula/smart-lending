use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::dtos::auth::Claims;

pub async fn require_auth(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Extract headers
    let headers = req.headers();

    // Get the Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Ensure the header starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract the token
    let token = auth_header.trim_start_matches("Bearer ").trim();

    // Validate the token
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    // Handle token validation result
    match token_data {
        Ok(token_data) => {
            // Insert claims into request extensions
            req.extensions_mut().insert(token_data.claims);

            // Pass the request to the next middleware or handler
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}