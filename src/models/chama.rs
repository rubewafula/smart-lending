

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

use crate::enums::LoanRepaymentFrequecyEnum;

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct Chama {
    pub id:Option<i64>,             
    pub name:String,           
    pub  contact_number:String,
    pub  location:String,       
    pub  size:i32,           
    pub  contact_person:String,
    pub  reg_number:String,     
    pub created_at:NaiveDateTime,     
    pub  updated_at :NaiveDateTime,
    pub created_by:i64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaMember {
    pub id:Option<i64>,                 
    pub user_id:i64,           
    pub chama_id:i64,           
    pub position:i64,      
    pub contribution_amount:f64,
    pub created_at: NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub created_by:i64,
    pub is_active:i8,  

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaPosition {
    pub id:Option<i64>,                 
    pub chama_position:String,           
    pub role_id:i64,           
    pub created_at: NaiveDateTime,
    pub updated_at:NaiveDateTime      

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoanApprover {
    pub id:Option<i64>,                 
    pub chama_id:i64,  
    pub created_by:i64,         
    pub approver_position_id:i64,           
    pub created_at: NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub is_active:i8  

}


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoanLimit {
    pub id:Option<i64>,                 
    pub chama_id:i64,           
    pub amount:f64, 
    pub centage_member_savings:f64,          
    pub created_at: NaiveDateTime,
    pub updated_at:NaiveDateTime,  
    pub is_active:i8   

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoanRepaymentLimit {
    pub id:Option<i64>,                 
    pub chama_id:i64,           
    pub repayment_frequency:String, 
    pub max_repayment_months:i32,          
    pub created_at: NaiveDateTime,
    pub updated_at:NaiveDateTime      

}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaLoanQuaranteeSetting {
    pub id:Option<i64>,                 
    pub chama_id:i64,           
    pub centage_required:f64, 
    pub is_active:i8,        
    pub created_at: NaiveDateTime,
    pub updated_at:NaiveDateTime      

}


#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct ChamaInvite {
    pub id:Option<i64>,                 
    pub chama_id:i64,  
    pub expiry_date:NaiveDateTime,
    pub invite_hash:String,    
    pub invited_by:i64,     
    pub created_at: NaiveDateTime,
    pub updated_at:NaiveDateTime      

}