
use axum::{
    routing::get,
    Router, Json, response::IntoResponse,
};
use crate::models::auth;

pub async fn get_user() -> impl IntoResponse {
   

}

pub async fn add_user(Json(payload): Json<auth::AuthUser>) -> impl IntoResponse {
    
   
}

pub fn routes() -> Router {
    Router::new()
        .route("/user", get(get_user).post(add_user))
}