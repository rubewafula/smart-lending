

use std::env;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration, NaiveDateTime};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use sqlx::{MySql, MySqlPool, Transaction};
use tracing::{info, error};

use crate::dtos::auth::{Claims, SignupInfo};
use crate::repositories::data_repository;
use crate::repositories::crud_repository_trait::CrudRepositoryTrait;
use crate::models::auth;
use crate::utils;
use crate::services::email_service;



pub async fn get_auth_user(pool:&MySqlPool, username:&str) -> Option<auth::AuthUser>{

    let user_data_repository = data_repository::DataRepository::<auth::AuthUser> {
        pool: pool,
        table_name: "auth_user",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    match user_data_repository.find_by("username", username).await {
        Ok(mut results) => { 
            info!("Found results from query");
            return results.pop()
        }
        Err(e) => {

            error!("Found error during Query:{}",  e);
            return None
        },
    }

}

pub async fn get_auth_user_by_id(pool:&MySqlPool, user_id:&str) -> Option<auth::AuthUser>{

    let user_data_repository = data_repository::DataRepository::<auth::AuthUser> {
        pool: pool,
        table_name: "auth_user",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    match user_data_repository.find_by("id", user_id).await {
        Ok(mut results) => { 
            info!("Found results from query");
            return results.pop()
        }
        Err(e) => {

            error!("Found error during Query:{}",  e);
            return None
        },
    }

}

pub async fn get_auth_user_by_email(pool:&MySqlPool, email:&str) -> Option<auth::AuthUser>{

    let user_data_repository = data_repository::DataRepository::<auth::AuthUser> {
        pool: pool,
        table_name: "auth_user",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    match user_data_repository.find_by("email", email).await {
        Ok(mut results) => { 
            info!("Found results from query to user using email : {}", email);
            return results.pop()
        }
        Err(e) => {

            error!("Found error during Query:{}",  e);
            return None
        },
    }

}

pub async fn send_password_reset(pool: &MySqlPool, user_id: &i64, name:&str, email:&str) -> Option<String> {
    let token_data_repository = data_repository::DataRepository::<auth::AuthtokenToken> {
        pool: pool,
        table_name: "authtoken_token",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };
    let now_eat: NaiveDateTime = (Utc::now() + Duration::hours(3)).naive_utc();
    let user_token: String = utils::generate_token_128();

    let user_token_model = auth::AuthtokenToken {
        token:user_token.clone(),
        created:now_eat,
        user_id:*user_id
     };

     let token_id = match token_data_repository.insert(&user_token_model).await {
        Ok(results) => results,
        Err(e) => {
            error!("Failed to insert token: {}", e);
            return None
        },

     };
    
    if token_id == 0 {
        return None
    }

    info!("Caling send email function for reset email:{}", email);
    let email_subject:String = "Reset Password".to_string();
    let vurl:String = env::var("ORIGINATOR_EMAIL_VERIFICATION_URL").unwrap();
    let reset_password_url:String = format!("{}/reset-password/{}", vurl, user_token);

    let email_body  = format!(
        r#"<p>Hi {},</p>
        <p>Please reset your password by clicking the link below:</p>
        <p><a href="{}" style="color: #1a73e8;">Reset password</a></p>
        <p>Can't see link? use this url : {} </p>
        <p>Thanks,<br>YourApp Team</p>"#,
        name, reset_password_url, reset_password_url
    );
    let email_sent_message = email_service::send_email(
        email.to_string(), 
        email_subject, 
        email_body
    ).await;
    info!("Email send response: {}", email_sent_message);

    Some("Email dispatched".to_string())

   
}

pub async fn activate_user_account(pool:&MySqlPool, user_id:&i64) -> Option<auth::AuthUser>{

    let user_data_repository = data_repository::DataRepository::<auth::AuthUser> {
        pool: pool,
        table_name: "auth_user",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    match user_data_repository.find_by_id(&user_id).await {
        Ok(results) => { 
            info!("Found results from query");
            if let Some(mut user) = results {
                if user.is_active == 0 {
                    user.last_login = Some((Utc::now() + Duration::hours(3)).naive_utc());
                    user.is_active = 1;

                    match user_data_repository.update_by_id(&user_id, &user).await {
                        Ok(affected) => {
                            if affected > 0 {
                                info!("User account activated successfully, {}", affected);
                                return Some(user);
                            } else {
                                error!("Failed to update user: 0");
                                return None
                            }
                        },
                        Err(e) => {
                            error!("Failed to update user: {}", e);
                            return None
                        },
                    }
                } else {
                        error!("User already activated");
                        return None
                }
            }
            return None
        },
        Err(e) => {

            error!("Found error during Query:{}",  e);
            return None
        },
    }

}

pub async fn decode_token(token:&str) -> Option<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims.sub)
    .ok()
}


pub async fn get_auth_token(pool:&MySqlPool, token:&str) -> Option<auth::AuthtokenToken>{

    let user_data_repository = data_repository::DataRepository::<auth::AuthtokenToken> {
        pool: pool,
        table_name: "authtoken_token",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    match user_data_repository.find_by("token", token).await {
        Ok(mut results) => { 
            info!("Found results from query auth token query");
            return results.pop()
        }
        Err(e) => {

            error!("Found error during Query:{}",  e);
            return None
        },
    }

}

pub async fn change_password(pool:&MySqlPool, &user_id:&i64, username:&str, new_password:&str) -> bool {

    let user_data_repository = data_repository::DataRepository::<auth::AuthUser> {
        pool: pool,
        table_name: "auth_user",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let password = hash_password(&new_password);
    let now_eat: NaiveDateTime = (Utc::now() + Duration::hours(3)).naive_utc();

    if let Some(mut user) = user_data_repository.find_by_id(&user_id).await.unwrap() {
        if user.username != username {
            error!("User not found");
            return false
        }
        user.password = password;
        user.last_login = Some(now_eat);
        user.is_active = 1;
        match user_data_repository.update_by_id(&user_id, &user).await {
            Ok(affected) => {
                if affected > 0 {
                    info!("User password updated successfully, {}", affected);
                    return true;
                } else {
                    error!("Failed to update user password");
                    return false
                }
            },
            Err(e) => {
                error!("Failed to update user password: {}", e);
                return false
            },
        }
    } else {
        error!("User not found");
        return false
    }

}

pub async fn  create_new_account(pool:&MySqlPool, payload: &SignupInfo )-> i64{

    let user_data_repository = data_repository::DataRepository::<auth::AuthUser> {
        pool,
        table_name: "auth_user",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };
    let password = hash_password(&payload.password);
    let now_eat: NaiveDateTime = (Utc::now() + Duration::hours(3)).naive_utc();

    let user = auth::AuthUser {
        id:None,
        password:password,
        last_login:None,
        is_superuser:0,
        username:payload.username.clone(),
        first_name:payload.first_name.clone(),
        last_name:payload.last_name.clone(),
        email:payload.email.clone(),
        is_staff:0,
        is_active:0,
        date_joined:now_eat
    };
    let result:i64 = match user_data_repository.record_exists(&"username", &user.username).await {
        Ok(exists) => { if exists { -1 } else { 1 } },
        Err(_) => 0,
    };
    if result == -1 || result == 0{
        return result
    }

    let mut tx: Transaction<'_, MySql> = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to start db transaction: {}", e);
            return 0;
        }
    };



     let user_id = match user_data_repository.insert_trx(&mut tx, &user).await {
        Ok(results) => results,
        Err(e) => {
            error!("Failed to insert user: {}", e); 
            return 0
        }


     };
     if user_id == 0 {
        return 0
     }

     let user_token:String = utils::generate_token_128();
     let token_data_repository = data_repository::DataRepository::<auth::AuthtokenToken> {
        pool,
        table_name: "authtoken_token",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

     let user_token_model = auth::AuthtokenToken {
        token:user_token.clone(),
        created:now_eat,
        user_id:user_id
     };
    
     let token_id = match token_data_repository.insert_trx(&mut tx, &user_token_model).await {
        Ok(results) => results,
        Err(e) => {
            error!("Failed to insert token: {}", e);
            return 0
        },

     };
    
    info!("Token id is not 0 will proceed to send email:{}", token_id);

    
    info!("Caling send email function to email:{}", payload.email.clone().unwrap());
    let email_subject:String = "Account Confirmation".to_string();
    let vurl:String = env::var("ORIGINATOR_EMAIL_VERIFICATION_URL").unwrap();
    let verification_url:String = format!("{}/verify/{}", vurl, user_token);

    let email_body  = format!(
        r#"<p>Hi {},</p>
        <p>Please verify your email by clicking the link below:</p>
        <p><a href="{}" style="color: #1a73e8;">Activate your Account</a></p>
        <p>Can't see link? use this url : {} </p>
        <p>Thanks,<br>YourApp Team</p>"#,
        payload.first_name, verification_url, verification_url
    );
    if let Some(email) = payload.email.clone() {
        let email_sent_message = email_service::send_email(
            email, 
            email_subject, 
            email_body
        ).await;
        info!("Email send response: {}", email_sent_message);
    }
     
     
     if tx.commit().await.is_ok() {
            return  user_id;
     } else {
        return 0
     }
     
}

pub fn hash_password(password: &str) -> String{
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hashed_password:&str) -> bool{
    verify(password, &hashed_password).unwrap()
}


pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

