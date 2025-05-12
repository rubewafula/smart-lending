use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
#[derive(Serialize, Deserialize)]
pub enum LoadRequestStatus {
   PENDING,
   PARTIALYAPPROVED,
   APPROVED,
   REJECTED,
   DISBURSED,
   REPAID,
   DEFAULTED
}


#[derive(Serialize, Deserialize)]
pub struct LoanRequest {
  pub id:i64,
  pub user_id:i64,
  pub amount_requested:f64,
  pub amount_approved:f64,
  pub credit_profile_id:i64,
  pub status:LoadRequestStatus,
  pub frequency:BillFrequencyEnum,
  pub due_date:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
 
} 

#[derive(Serialize, Deserialize)]
pub struct LoanRequestGuarantee {
  pub id:i64,
  pub load_request_id:i64,
  pub loan_quaranter_id:i64,
  pub amount_quaranteed:f64,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
 
} 


#[derive(Serialize, Deserialize)]
pub struct LoanRepayment {
  pub id:i64,
  pub load_request_id:i64,
  pub amount:f64,
  pub loan_balance:f64,
  pub payment_mothod:String,
  paid_at:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
 
} 

#[derive(Serialize, Deserialize)]
pub struct LoanDafault {
  pub id:i64,
  pub load_request_id:i64,
  pub loan_balance:f64,
  pub status:String,
  pub default_at:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
 
} 

#[derive(Serialize, Deserialize)]
pub struct LoanDafaultReclaim {
  pub id:i64,
  pub quarantee_type:String,
  pub quaranter_id:i64,
  pub amount_quaranteed:f64,
  pub chama_id:Option<i64>,
  pub default_at:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
 
} 