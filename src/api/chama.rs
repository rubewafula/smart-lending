

use axum::routing::{post,get};
use sqlx::MySqlPool;
use axum::debug_handler;

use axum::{
    Router, Json, response::IntoResponse,
    http::StatusCode,
    Extension,
    extract::Path,
    middleware
};
use crate::dtos::chama::{
    ChamaDto, 
    ChamaLoadLimitDto, 
    ChamaLoanApproverDto, 
    ChamaLoanQuaranteeSettingDto, 
    ChamaLoanRepaymentLimitDto, 
    ChamaMemberApproveDto, 
    ChamaMemberDetailDto, 
    ChamaPositionDetailDto
};
use crate::utils::{ApiResponse, is_valid_phone};
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
        return ApiResponse::<&str>::error(&format!("Contact number not valid, phone number expected").to_string(), StatusCode::BAD_REQUEST.as_u16()) 
    }

    //let mut payload = payload;
    payload.contact_number = contact_number;

    let last_insert_id = chama_service::create_new_chama(&pool, &user_id, &payload).await;
     
    if last_insert_id == -1  {
        return ApiResponse::<&str>::error(&format!("Chama with such name exists").to_string(), StatusCode::IM_USED.as_u16())
    } else if last_insert_id  != 0 { 
        return ApiResponse::<&str>::success(Some("Chama created"))
    } else {
        return ApiResponse::<&str>::error(&format!("Could not created user").to_string(), StatusCode::EXPECTATION_FAILED.as_u16())
    }

}

pub async fn update_chama(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Json(mut payload): Json<ChamaDto>) -> impl IntoResponse {

        let mut contact_number: String = payload.contact_number.clone();
        let user_id = claims.sub;
        let chama_id = payload.id.unwrap();
        let roles = chama_service::get_chama_roles(&pool, &user_id, &chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();
        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }
        
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
        let roles = chama_service::get_chama_roles(&pool, &user_id, &chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();
        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<String>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        let invite_url = chama_service::get_invite(&pool, &user_id, &chama_id).await;
         
        if invite_url == "-1".to_string()  {
            return ApiResponse::<String>::error(&format!("Chama with such name exists"), StatusCode::IM_USED.as_u16())
        } else if invite_url  != "0".to_string() { 
            return ApiResponse::<String>::success(Some(String::from(invite_url)))
        } else {
            return ApiResponse::<String>::error(&format!("Could not created user"), StatusCode::EXPECTATION_FAILED.as_u16())
        }
}

pub async fn approve_member(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Json(payload):Json<ChamaMemberApproveDto>) -> impl IntoResponse {

        let user_id = claims.sub;
        let chama_id = payload.chama_id;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        let approved = chama_service::approve_member(&pool, &user_id, &payload).await;
         
        if approved == -1 {
            return ApiResponse::<&str>::error(&format!("No such member"), StatusCode::IM_USED.as_u16())
        } else if approved  != 0 { 
            return ApiResponse::success(Some("Member approved"))
        } else {
            return ApiResponse::<&str>::error(&format!("Could not approve member"), StatusCode::EXPECTATION_FAILED.as_u16())
        }
}

pub async fn remove_member(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Path(chama_id):Path<i64>, Path(member_id):Path<i64>) -> impl IntoResponse {

        let user_id = claims.sub;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        let removed = chama_service::remove_member(&pool, &member_id).await;
         
        if removed == -1 {
            return ApiResponse::<&str>::error(&format!("No such member"), StatusCode::IM_USED.as_u16())
        } else if removed  != 0 { 
            return ApiResponse::success(Some("Member removed"))
        } else {
            return ApiResponse::<&str>::error(&format!("Could not remove member"), StatusCode::EXPECTATION_FAILED.as_u16())
        }
}

pub async fn members(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Path(chama_id):Path<i64>) -> impl IntoResponse {

        let user_id = claims.sub;
        match chama_service::get_members(&pool, &chama_id.to_string(), &user_id).await {
            Ok(members) => ApiResponse::<Vec<ChamaMemberDetailDto>>::success(Some(members)),
            Err(_) => ApiResponse::<Vec<ChamaMemberDetailDto>>::error(&format!("Could not get members"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}

pub async fn get_loan_approvers(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Path(chama_id):Path<i64>) -> impl IntoResponse {

        let user_id = claims.sub;
        match chama_service::get_loan_approvers(&pool, &chama_id.to_string(), &user_id).await {
            Ok(members) => ApiResponse::<Vec<ChamaMemberDetailDto>>::success(Some(members)),
            Err(_) => ApiResponse::<Vec<ChamaMemberDetailDto>>::error(&format!("Could not get members"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}


pub async fn get_chama_positions(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
        match chama_service::get_chama_positions(&pool).await {
            Ok(positions) => ApiResponse::<Vec<ChamaPositionDetailDto>>::success(Some(positions)),
            Err(_) => ApiResponse::<Vec<ChamaPositionDetailDto>>::error(&format!("Could not get positions"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}


pub async fn get_guaranter_setting(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Path(chama_id):Path<i64>) -> impl IntoResponse {
        match chama_service::get_guaranter_settings(&pool, &chama_id).await {
            Ok(settings) => ApiResponse::<Vec<ChamaLoanQuaranteeSettingDto>>::success(Some(settings)),
            Err(_) => ApiResponse::<Vec<ChamaLoanQuaranteeSettingDto>>::error(&format!("Could not get positions"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}



pub async fn add_loan_approver(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Json(payload):Json<ChamaLoanApproverDto>) -> impl IntoResponse {

        let user_id = claims.sub;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &payload.chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        match chama_service::add_loan_approver(&pool, &user_id, &payload).await {
            Ok(_) => ApiResponse::success(Some("Approver added")),
            Err(_) => ApiResponse::<&str>::error(&format!("Could not add approver"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}


pub async fn add_guaranter_setting(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Json(payload):Json<ChamaLoanQuaranteeSettingDto>) -> impl IntoResponse {

        let user_id = claims.sub;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &payload.chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        match chama_service::add_guaranter_setting(&pool, &payload).await {
            Ok(_) => ApiResponse::success(Some("Approver added")),
            Err(_) => ApiResponse::<&str>::error(&format!("Could not add guranter setting"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}


pub async fn add_chama_loan_limit(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Json(payload):Json<ChamaLoadLimitDto>) -> impl IntoResponse {

        let user_id = claims.sub;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &payload.chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        match chama_service::add_loan_limit(&pool, &payload).await {
            Ok(_) => ApiResponse::success(Some("Loan limit added")),
            Err(_) => ApiResponse::<&str>::error(&format!("Could not add loan limit"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}
pub async fn add_chama_loan_repayment_limit(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, Json(payload):Json<ChamaLoanRepaymentLimitDto>) -> impl IntoResponse {

        let user_id = claims.sub;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &payload.chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        match chama_service::add_loan_repayment_limit(&pool, &payload).await {
            Ok(_) => ApiResponse::success(Some("Loan repayment limit added")),
            Err(_) => ApiResponse::<&str>::error(&format!("Could not add loan repayment limit"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}

pub async fn remove_from_loan_approver(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Path(position_id):Path<i64>,
    Path(chama_id):Path<i64>) -> impl IntoResponse {

        let user_id = claims.sub;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        match chama_service::remove_from_loan_approver(&pool, &position_id).await {
            Ok(_) => ApiResponse::success(Some("Approver added")),
            Err(_) => ApiResponse::<&str>::error(&format!("Could not add approver"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}

pub async fn remove_guaranter_setting(
    Extension(claims): Extension<Claims>, 
    Extension(pool): Extension<MySqlPool>, 
    Path(g_id):Path<i64>, Path(chama_id):Path<i64>) -> impl IntoResponse {

        let user_id = claims.sub;
        let roles = chama_service::get_chama_roles(&pool, &user_id, &chama_id.to_string()).await;
        let roles = roles.unwrap_or_default();

        
        if !roles.contains(&String::from("chama-admin")) {
            return ApiResponse::<&str>::error(&format!("User not allowed to perform this action"), StatusCode::FORBIDDEN.as_u16()) 
        }

        match chama_service::remove_guarantee_setting(&pool, &g_id).await {
            Ok(_) => ApiResponse::success(Some("Guarantee removed")),
            Err(_) => ApiResponse::<&str>::error(&format!("Could not remove setting"), StatusCode::EXPECTATION_FAILED.as_u16()),
        }
        
}



pub fn routes() -> Router {
    Router::new()
        .route("/chama/create", post(create_new_chama))
        .route("/chama/update", post(update_chama))
        .route("/chama/invite/:chama_id", get(get_invite))
        .route("/chama/join/:invite_hash", get(join_chama))

        .route("/chama/approve-member", post(approve_member)) 

        .route("/chama/members/:chama_id", get(members))
        .route("/chama/remove-member/:chama_id/:member_id", get(remove_member))

        .route("/chama/add-approver", post(add_loan_approver))
        .route("/chama/approvers/:chama_id", get(get_loan_approvers))
        .route("/chama/remove-approver/:chama_id/:position_id", post(remove_from_loan_approver))

        .route("/chama/positions", get(get_chama_positions))
        
        .route("/chama/add-guaranter-setting", post(add_guaranter_setting))
        .route("/chama/remove-guaranter-setting/:chama_id/:g_id", post(remove_guaranter_setting))
        .route("/chama/guaranter-setting/:chama_id", post(get_guaranter_setting))
        //create or update
        .route("/chama/loan-limit", post(add_chama_loan_limit))
        //create or update
        .route("/chama/add-loan-repayment-limit", post(add_chama_loan_repayment_limit))
        .layer(middleware::from_fn(require_auth))


}
