

use sqlx::MySqlPool;
use axum::debug_handler;
use tracing::info;
use axum::http::{response, HeaderMap};
use axum::{
    routing::{post,get}
    Router, Json, response::IntoResponse,
    http::StatusCode,
    Extension,
    extract::Path,
    middleware
};
use crate::models::chama;
use crate::dtos::chama::{ChamaDto}
use crate::utils::{ApiResponse, is_valid_phone, is_valid_email};
use crate::middleware::auth::require_auth;


#[debug_handler]
pub async fn create_new_chama(Extension(pool): Extension<MySqlPool>, Json(payload): Json<ChamaDto>) -> impl IntoResponse {
    let mut contact_number: String = payload.contact_number.clone();

    if let Some(phone)= is_valid_phone(&contact_number){
        contact_number = phone;
    } else {
        return ApiResponse::<&str>::error(&format!("Username not valid, phone number expected"), StatusCode::BAD_REQUEST.as_u16()) 
    }


    

}

pub fn routes() -> Router {
    Router::new()
        .route("/chama/create", post(create_new_chama))
        .route("/chama/update", post(create_new_chama))

        .route("/chama/add-member", post(create_new_chama))
        .route("/chama/members", post(create_new_chama))
        .route("/chama/remove-member", post(create_new_chama))

        .route("/chama/add-approver", post(create_new_chama))
        .route("/chama/approvers", post(create_new_chama))
        .route("/chama/remove-approver", post(create_new_chama))

        .route("/chama/create-position", post(create_new_chama))
        .route("/chama/positions", post(create_new_chama))
        .route("/chama/remove-position", post(create_new_chama))

        .route("/chama/add-guaranter-setting", post(create_new_chama))
        .route("/chama/remove-guaranter-setting", post(create_new_chama))
        .route("/chama/guaranter-setting", post(create_new_chama))
        //create or update
        .route("/chama/loan-limit", post(create_new_chama))
        //create or update
        .route("/chama/add-loan-repayment-limit", post(create_new_chama))
        .layer(middleware::from_fn(require_auth));


}