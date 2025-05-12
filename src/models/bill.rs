use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum PaymentModeEnum{
  CREDIT,
  CHECHOUT
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum AccountTypeEnum {
  PAYBILL,
  TILL
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum BillFrequencyEnum {
   Adhoc,
   Daily,
   Weekly,
   BiWeekly,
   Monthy,
   Quarterly,
   SemiAnnually,
   Annually

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Bill {
  pub id:i64,
  pub biller_id:i64,
  pub status:String,
  pub user_id:i64,
  pub frequency:BillFrequencyEnum,
  pub amount:f64,
  pub due_date:NaiveDateTime,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 



#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct BillHandler {
  pub id:i64,
  pub biller_id:i64,
  pub end_point:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64
} 



#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct BillPayment {
  pub id:i64,
  pub bill_id:i64,
  pub amount:f64,
  pub pay_date:NaiveDateTime,
  pub payment_mode: PaymentModeEnum,
  pub aggregator_transaction_id:String,
  pub vendor_receipt_id:String,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct BillReminder {
  pub id:i64,
  pub bill_id:i64,
  pub remind_date:NaiveDateTime,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 




#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Biller {
  pub id:i64,
  pub name:String,
  pub nick_name:String,
  pub account_type: AccountTypeEnum,
  pub bill_number:String,
  pub status:String,
  pub account_number:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i64,
} 
