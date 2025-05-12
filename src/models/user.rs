use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct UserCreditorganization {
  pub id:i32,
  pub user_id:i32,
  pub organization_id:i32,
  pub income_range_id:i32,
  pub user_identity_id:String,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct UserGroups {
  pub id:i32,
  pub user_group:String,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Roles {
  pub id:i32,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
} 



#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct UserDetail {
  pub id:i32,
  pub user_id:i32,
  pub id_no:String,
  pub kra_pin:String,
  pub profession:String,
  pub dob:NaiveDateTime,
  pub married:i8,
  pub default_contact:String,
  pub phone_imsi:String,
  pub phone_imei:String,
  pub credit_profile_id:i32,
  pub credit_due_date:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
  pub status:String,
  pub gender:String,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct IncomeHistory {
  pub id:i64,
  pub user_id:i64,
  pub credit_organization_id:i64,
  pub prev_range_id:i64,
  pub current_range_id:i64,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AcccountBalance {
    pub id: i32,
    pub user_id: i32,
    pub balance: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}