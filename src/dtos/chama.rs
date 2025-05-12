use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChamaDto {
    id:Option<i64>,             
    name:String,           
    contact_number:String,
    location:String,       
    size:i32,           
    contact_person:String,
    reg_number:Option<String>,       
}

#[derive(Debug, Deserialize)]
pub struct ChamaMemberDto {
    id:Option<i64>,                 
    user_id:i64,           
    chama_id:i64,           
    position:i64,      
    contribution_amount:f64,     

}

#[derive(Debug, Deserialize)]
pub struct ChamaPositionDto {
    id:Option<i64>,                 
    chama_position:String,           
    role_id:i64,               

}

#[derive(Debug, Deserialize)]
pub struct ChamaLoadApproverDto {
    id:Option<i64>,                 
    chama_id:i64,           
    approver_position_id:i64,               

}


#[derive(Debug, Deserialize)]
pub struct ChamaLoadLimitDto {
    id:Option<i64>,                 
    chama_id:i64,           
    amount:Option<f64>, 
    centage_member_savings:Option<f64>,               

}

#[derive(Debug, Deserialize)]
pub struct ChamaLoanRepaymentLimitDto {
    id:Option<i64>,                 
    chama_id:i64,           
    repayment_frequency:LoanRepaymentFrequecyEnum, 
    max_repayment_months:i32,  
}

#[derive(Debug, Deserialize)]
pub struct ChamaLoadQuaranteeSettingDto {
    id:Option<i64>,                 
    chama_id:i64,           
    centage_required:f64, 
}