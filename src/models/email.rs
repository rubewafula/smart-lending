
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Deserialize)]

pub struct EmailRequest {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, FromRow)]
pub struct AcccountEmailAddress {
    pub id: i64,
    pub email: String,
    pub verified: i8,
    pub primary:i8,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize)]   
#[derive(Debug, FromRow)]                                            
pub struct AccountEmailconfirmation {                                           
    pub id:i64,                                                                       
    pub created:NaiveDateTime,                                                        
    pub sent:NaiveDateTime,                                                           
    pub key:String,                                                                   
    pub email_address_id:i64 
} 