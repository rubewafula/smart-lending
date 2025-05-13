

use axum::routing::{post,get};
use sqlx::MySqlPool;
use axum::debug_handler;
use tracing::info;
use axum::http::{response, HeaderMap};
use axum::{
    Router, Json, response::IntoResponse,
    http::StatusCode,
    Extension,
    extract::Path,
    middleware
};
use crate::dtos::chama::{ChamaDto, ChamaMemberDto};
use crate::utils::{ApiResponse, is_valid_phone, is_valid_email};
use crate::middleware::auth::require_auth;
use crate::dtos::auth::Claims;
use crate::services::chama_service;


#[debug_handler]
pub async fn create_new_chama(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Json(mut payload): Json<ChamaDto>) -> impl IntoResponse {

    let mut contact_number: String = payload.contact_number.clone();
    let user_id = claims.sub;
    
    if let Some(phone)= is_valid_phone(&contact_number){
        contact_number = phone;
    } else {
        return ApiResponse::<&str>::error(&format!("Contact number not valid, phone number expected"), StatusCode::BAD_REQUEST.as_u16()) 
    }

    //let mut payload = payload;
    payload.contact_number = contact_number;

    let last_insert_id = chama_service::create_new_chama(&pool, &user_id, &payload).await;
     
    if last_insert_id == -1  {
        return ApiResponse::<&str>::error(&format!("Chama with such name exists"), StatusCode::IM_USED.as_u16())
    } else if last_insert_id  != 0 { 
        return ApiResponse::success(Some("Chama created"))
    } else {
        return ApiResponse::<&str>::error(&format!("Could not created user"), StatusCode::EXPECTATION_FAILED.as_u16())
    }

}

pub async fn update_chama(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Json(mut payload): Json<ChamaDto>) -> impl IntoResponse {

        let mut contact_number: String = payload.contact_number.clone();
        let user_id = claims.sub;
        
        if let Some(phone)= is_valid_phone(&contact_number){
            contact_number = phone;
        } else {
            return ApiResponse::<&str>::error(&format!("Contact number not valid, phone number expected"), StatusCode::BAD_REQUEST.as_u16()) 
        }
    
        //let mut payload = payload;
        payload.contact_number = contact_number;
    
        let last_insert_id = chama_service::update_chama(&pool, &user_id, &payload).await;
         
        if last_insert_id == -1  {
            return ApiResponse::<&str>::error(&format!("Chama with such name exists"), StatusCode::IM_USED.as_u16())
        } else if last_insert_id  != 0 { 
            return ApiResponse::success(Some("Chama created"))
        } else {
            return ApiResponse::<&str>::error(&format!("Could not created user"), StatusCode::EXPECTATION_FAILED.as_u16())
        }
}


pub async fn join_chama(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Path(invite_hash): Path<String>) -> impl IntoResponse {

        let user_id = claims.sub;

        let last_insert_id = chama_service::join_chama(&pool, &user_id, &invite_hash).await;
         
        if last_insert_id == -1  {
            return ApiResponse::<&str>::error(&format!("No such chama "), StatusCode::IM_USED.as_u16())
        } else if last_insert_id  != 0 { 
            return ApiResponse::success(Some("User added to Chama"))
        } else {
            return ApiResponse::<&str>::error(&format!("Could not add user to chama"), StatusCode::EXPECTATION_FAILED.as_u16())
        }
}

pub async fn get_invite(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Path(chama_id): Path<i64>, ) -> impl IntoResponse {

        let user_id = claims.sub;

        let invite_url = chama_service::get_invite(&pool, &user_id, &chama_id).await;
         
        if invite_url == "-1".to_string()  {
            return ApiResponse::<String>::error(&format!("Chama with such name exists"), StatusCode::IM_USED.as_u16())
        } else if invite_url  != "0".to_string() { 
            return ApiResponse::<String>::success(Some(String::from(invite_url)))
        } else {
            return ApiResponse::<String>::error(&format!("Could not created user"), StatusCode::EXPECTATION_FAILED.as_u16())
        }
}

pub async fn create_new_position(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Json(mut payload): Json<ChamaDto>) -> impl IntoResponse {
    let mut position: String = payload.position.clone();
    let user_id = claims.sub;
    let last_insert_id = chama_service::create_new_position(&pool, &user_id, &payload).await;
    if last_insert_id == -1  {
        return ApiResponse::<&str>::error(&format!("Position with such name exists"), StatusCode::IM_USED.as_u16())
    } else if last_insert_id  != 0 { 
        return ApiResponse::success(Some("Chama position created"))
    } else {
        return ApiResponse::<&str>::error(&format!("Could not create position"), StatusCode::EXPECTATION_FAILED.as_u16())
    }
}


pub fn routes() -> Router {
    Router::new()
        .route("/chama/create", post(create_new_chama))
        .route("/chama/update", post(update_chama))
        .route("/chama/invite/:chama_id", get(get_invite))
        .route("/chama/join/:invite_hash", get(join_chama))
        .route("/chama/add-position", post(create_new_position))
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
        .layer(middleware::from_fn(require_auth))


}