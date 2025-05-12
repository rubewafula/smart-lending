use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Transaction {
  pub id:i64,
  pub user_id:i64,
  pub amount:f64,
  pub transaction_type:String,
  pub trx_time:NaiveDateTime,
  pub cr_dr:i8,
  pub reference:String,
  pub status:String,
  pub narration:String,
  pub pre_balance:f64,
  pub balance:f64,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Deposit {
  pub id:i64,
  pub user_id:i64,
  pub paying_number:String,
  pub bill_no:String,
  pub account_no:String,
  pub vendor_reference:String,
  pub bill_date:NaiveDateTime,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Withdrawal {
  pub id:i64,         
  pub user_id:i64,    
  pub amount:f64,     
  pub msisdn:String,     
  pub reference:String,  
  pub narration:String,  
  pub status:String,     
  pub created_at: NaiveDateTime,
  pub updated_at :NaiveDateTime
}
