

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum LoanRepaymentFrequecyEnum{
    WEEKLY,
    BIWEEKLY,
    MONTHLY
  }

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Chama {
    id:Option<i64>,             
    name:String,           
    contact_number:String,
    location:String,       
    size:i32,           
    contact_person:String,
    reg_number:String,     
    created_at:NaiveDateTime,     
    updated_at :NaiveDateTime    
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaMember {
    id:Option<i64>,                 
    user_id:i64,           
    chama_id:i64,           
    position:i64,      
    contribution_amount:f64,
    created_at: NaiveDateTime,
    updated_at:NaiveDateTime      

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaPosition {
    id:Option<i64>,                 
    chama_position:String,           
    role_id:i64,           
    created_at: NaiveDateTime,
    updated_at:NaiveDateTime      

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoadApprover {
    id:Option<i64>,                 
    chama_id:i64,           
    approver_position_id:i64,           
    created_at: NaiveDateTime,
    updated_at:NaiveDateTime      

}


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoadLimit {
    id:Option<i64>,                 
    chama_id:i64,           
    amount:f64, 
    centage_member_savings:f64,          
    created_at: NaiveDateTime,
    updated_at:NaiveDateTime      

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoanRepaymentLimit {
    id:Option<i64>,                 
    chama_id:i64,           
    repayment_frequency:LoanRepaymentFrequecyEnum, 
    max_repayment_months:i32,          
    created_at: NaiveDateTime,
    updated_at:NaiveDateTime      

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoadQuaranteeSetting {
    id:Option<i64>,                 
    chama_id:i64,           
    centage_required:f64, 
    max_repayment_months:i32,          
    created_at: NaiveDateTime,
    updated_at:NaiveDateTime      

}