use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct CreditBalance {
  pub id:i64,
  pub user_id:i64,
  pub balance:f64,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct CreditOrganization {
  pub id:i64,
  pub name:String,
  pub type_of_business:String,
  pub location:String,
  pub contact_person:String,
  pub contact_email:String,
  pub contact_phone:i64,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct CreditProfile {
  pub id:i64,
  pub name:String,
  pub narration:String,
  pub max_limit:f64,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct CreditScore {
  pub id:i64,
  pub user_id:i64,
  pub score:f64,
  pub expiry_date:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct CreditScorehistory {
  pub id:i64,
  pub user_id:i64,
  pub score_time:NaiveDateTime,
  pub pre_limit:f64,
  pub current_limit:f64,
  pub narration:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct CreditTransaction {
  pub id:i64,
  pub bill_id:i64,
  pub transaction_id:i64,
  pub bill_payment_id:i64,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct CreditOrganizationStaff {
  pub id:i64,
  pub credit_organization_id:i64,
  pub first_name:String,
  pub last_name:String,
  pub organization_identifier:String,
  pub msisdn: String,
  pub income:f64,
  pub guarantee_value:f64,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 
