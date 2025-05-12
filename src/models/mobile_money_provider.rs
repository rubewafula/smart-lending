use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
 pub enum AccountTypeEnum {
  SMPP,
  HTTP,
  PARLAYX

}

#[derive(Serialize, Deserialize)]
pub struct MobileMoneyprovider {
  pub id:i32,
  pub name:String,
  pub username:String,
  pub description:String,
  pub account_type:AccountTypeEnum,
  pub password:String,
  pub status:String,
  pub api_username:String,
  pub api_password:String,
  pub smsc_id:i32,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
} 
