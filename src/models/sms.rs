use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Message {
  pub id:i32,
  pub user_id:i32,
  pub date:NaiveDateTime,
  pub organization:String,
  pub amount:f64,
  pub transaction_type:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct SmsReminder {
  pub id:i32,
  pub code:String,
  pub message_template:String,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Outbox {
  pub id:i32,
  pub user_id:i32,
  pub sender:String,
  pub network:String,
  pub msisdn:String,
  pub reference:String,
  pub date_sent:NaiveDateTime,
  pub text:String,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Inbox {
  pub id:i64,
  pub msisdn:String,
  pub short_code:String,
  pub message:String,
  pub status:String,
  pub reference:String,
  pub arrive_time:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
} 
