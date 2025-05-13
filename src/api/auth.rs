
use axum::http::{response, HeaderMap};
use axum::{
    routing::post,
    Router, Json, response::IntoResponse,
    http::StatusCode,
    Extension,
    extract::Path
};
use sqlx::MySqlPool;
use axum::debug_handler;
use tracing::info;

use crate::services::authentication_service;
use crate::services::account_service;
use crate::dtos::auth as auth_dtos;
use crate::utils::{ApiResponse, is_valid_phone, is_valid_email};



#[debug_handler]
pub async fn login( 
    Extension(pool): Extension<MySqlPool>, Json(payload): Json<auth_dtos::LoginInfo>) -> impl IntoResponse {
   
    let mut username: String = payload.username.clone();

    if let Some(phone)= is_valid_phone(&username){
        username = phone;
    } else {
        return ApiResponse::<auth_dtos::LoginResponse>::error(&format!("Username not valid, phone number expected"), StatusCode::BAD_REQUEST.as_u16()) 
    }
    
    if let Some(user)  = authentication_service::get_auth_user(&pool, &username).await {
        info!("Found user from db");
        if authentication_service::verify_password(&payload.password, &user.password) {

            let id_str = match user.id {
                Some(id) => id.to_string(),
                None => "0".to_string(),
            };

            let roles = match authentication_service::get_user_roles(&pool, &user.id.unwrap()).await {
                Ok(roles) => roles,
                Err(e) => return ApiResponse::<auth_dtos::LoginResponse>::error(&format!("Failed to fetch roles: {}", e), 500),
            };

            let token = match authentication_service::generate_jwt(&id_str, &roles) {
                Ok(token) => token,
                Err(e) =>  return ApiResponse::<auth_dtos::LoginResponse>::error(&format!("Failed to generate token: {}", e), 500),
            };

            let user_id = user.id.unwrap_or(0);
            let (balance, cbalance) = account_service::get_user_balance(&pool,&user_id).await.unwrap();

            let login_response =  auth_dtos::LoginResponse {
                token: token,
                balance: balance.unwrap_or(0.0),
                credit_balance: cbalance.unwrap_or(0.0)
            };
            return ApiResponse::<auth_dtos::LoginResponse>::success(Some(login_response))
          

        } else {
            return ApiResponse::<auth_dtos::LoginResponse>::error(&format!("Invalid Password"), StatusCode::UNAUTHORIZED.as_u16())
        }

        
    } 
    info!("Let Some did not find any users");
    ApiResponse::<auth_dtos::LoginResponse>::error(&format!("User not found"), StatusCode::UNAUTHORIZED.as_u16())
    

}

#[debug_handler]
pub async fn signup(Extension(pool): Extension<MySqlPool>, Json(payload): Json<auth_dtos::SignupInfo>) -> impl IntoResponse {
    let mut payload = payload;

    if let Some(phone)= is_valid_phone(&payload.username.clone()){
        payload.username = phone;
    } else {
        return ApiResponse::<&str>::error(&format!("Username not valid, phone number expected"), StatusCode::BAD_REQUEST.as_u16()) 
    }
    
    if let Some(email) = &payload.email.clone() {
        if !is_valid_email(&email) {
            return ApiResponse::<&str>::error(&format!("Email not valid"), StatusCode::BAD_REQUEST.as_u16()) 
        }
    }

    let last_insert_id = authentication_service::create_new_account(&pool, &payload).await;
     
    if last_insert_id == -1  {
        ApiResponse::<&str>::error(&format!("Duplicate user"), StatusCode::IM_USED.as_u16())
    } else if last_insert_id  != 0 { 
        ApiResponse::success(Some("User created"))
    } else {
        ApiResponse::<&str>::error(&format!("Could not created user"), StatusCode::EXPECTATION_FAILED.as_u16())
    }
}

#[debug_handler]
pub async fn verify_account(Extension(pool): Extension<MySqlPool>, Path(token): Path<String>) -> impl IntoResponse {
    if let Some(token)  = authentication_service::get_auth_token(&pool, &token).await {
        
        if let Some(_) = authentication_service::activate_user_account(&pool, &token.user_id).await{

            return ApiResponse::success(Some("Account Verification Success"))
        }
        return ApiResponse::<&str>::error(&format!("Could not verify account"), StatusCode::INTERNAL_SERVER_ERROR.as_u16())

    } else {
        return ApiResponse::<&str>::error(&format!("Token not found"), StatusCode::NOT_FOUND.as_u16())

    }

}

#[debug_handler]
pub async fn forgot_password(Extension(pool): Extension<MySqlPool>, Json(payload): Json<auth_dtos::ForgotPasswordDto>) -> impl IntoResponse {
   
    let email= &payload.email;

    if !is_valid_email(&email) {
        return ApiResponse::<&str>::error(&format!("Email not valid"), StatusCode::BAD_REQUEST.as_u16()) 
    }
    
    if let Some(user) = authentication_service::get_auth_user_by_email(&pool, &payload.email).await {
        info!("Found user from db: {:?}", user); 
        if let Some(_) = authentication_service::send_password_reset(&pool, &user.id.unwrap(), &user.first_name, &payload.email).await {
            return ApiResponse::success(Some("Password reset email sent"))  
        } else {
            return ApiResponse::<&str>::error(&format!("Failed to generate token"), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
        }
    } else {
        return ApiResponse::<&str>::error(&format!("User not found"), StatusCode::NOT_FOUND.as_u16())
    }


}

#[debug_handler]
pub async fn reset_password(
    Extension(pool): Extension<MySqlPool>, 
    Path(token): Path<String>, 
    Json(payload): Json<auth_dtos::ResetPasswordDto>) -> impl IntoResponse {


   if let Some(token)  = authentication_service::get_auth_token(&pool, &token).await {
        
        if authentication_service::change_password(&pool, &token.user_id, &payload.username, &payload.password).await{

            return ApiResponse::success(Some("Password change Success"))
        }
        return ApiResponse::<&str>::error(&format!("Could not verify account"), StatusCode::INTERNAL_SERVER_ERROR.as_u16())

    } else {
        return ApiResponse::<&str>::error(&format!("Token not found"), StatusCode::NOT_FOUND.as_u16())

    }

}

#[debug_handler]
pub async fn change_password(
    Extension(pool): Extension<MySqlPool>, 
    headers: HeaderMap, 
    Json(payload): Json<auth_dtos::PasswordChangeRequestDto>
) -> impl IntoResponse {
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(str::to_string);

    let Some(token) = token else {
        return ApiResponse::<&str>::error(&format!("Token not found"), StatusCode::UNAUTHORIZED.as_u16())
    };

    let Some(id_str) = authentication_service::decode_token(&token).await else {
        return ApiResponse::<&str>::error(&format!("Invalid token"), StatusCode::UNAUTHORIZED.as_u16())
        
    };

    let Some(user) = authentication_service::get_auth_user_by_id(&pool, &id_str).await else {
        return ApiResponse::<&str>::error(&format!("User not found"), StatusCode::UNAUTHORIZED.as_u16())
        
    };

    let password_changed= authentication_service::change_password(&pool, &user.id.unwrap(), &user.username, &payload.new_password).await ;
    
    if !password_changed {
        return ApiResponse::<&str>::error(&format!("Could not change password"), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
    } 
    return ApiResponse::success(Some("Password chnage Success"))
    
}


pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/verify/:token", post(verify_account))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password/:token", post(reset_password))
        .route("/change-password", post(change_password))

}


