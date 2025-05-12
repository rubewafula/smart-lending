use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AuthGroupPermissions {
  pub id:i64,
  pub group_id:i64,
  pub permission_id:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AuthGroup {
  pub id:i64,
  pub name:String
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AuthPermission {
  pub id:i64,
  pub name:String,
  pub content_type_id:i64,
  pub codename:String
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AuthUserGroup {
  pub id:i64,
  pub user_id:i64,
  pub group_id:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AuthUserUserPermissions {
  pub id:i64,
  pub user_id:i64,
  pub permission_id:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AuthUser {
  pub id:Option<i64>,
  pub password:String,
  pub last_login:Option<NaiveDateTime>,
  pub is_superuser:i8,
  pub username:String,
  pub first_name:String,
  pub last_name:String,
  pub email:Option<String>,
  pub is_staff:i8,
  pub is_active:i8,
  pub date_joined:NaiveDateTime
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AuthtokenToken {
  pub token:String,
  pub created:NaiveDateTime,
  pub user_id:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct VerificationCode {
    id:i64,          
    user_id :i64,    
    code :String,         
    action : String,       
    created_at:NaiveDateTime,   
    expiration_date:NaiveDateTime
} 


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Permissions {
  pub id:i64,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
} 

