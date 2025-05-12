use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum LoanRepaymentFrequecyEnum {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Deserialize)]
pub struct ChamaDto {
    pub id:Option<i64>,             
    pub name:String,           
    pub contact_number:String,
    pub location:String,       
    pub size:i32,           
    pub contact_person:String,
    pub reg_number:Option<String>,       
}

#[derive(Debug, Deserialize)]
pub struct ChamaMemberDto {
    pub id:Option<i64>,                 
    pub user_id:i64,           
    pub chama_id:i64,           
    pub position:i64,      
    pub contribution_amount:f64,     

}

#[derive(Debug, Deserialize)]
pub struct ChamaPositionDto {
    pub id:Option<i64>,                 
    pub chama_position:String,           
    pub role_id:i64,               

}

#[derive(Debug, Deserialize)]
pub struct ChamaLoadApproverDto {
    pub id:Option<i64>,                 
    pub chama_id:i64,           
    pub approver_position_id:i64,               

}


#[derive(Debug, Deserialize)]
pub struct ChamaLoadLimitDto {
    pub id:Option<i64>,                 
    pub chama_id:i64,           
    pub amount:Option<f64>, 
    pub centage_member_savings:Option<f64>,               

}

#[derive(Debug, Deserialize)]
pub struct ChamaLoanRepaymentLimitDto {
    pub id:Option<i64>,                 
    pub chama_id:i64,           
    pub repayment_frequency:LoanRepaymentFrequecyEnum, 
    pub max_repayment_months:i32,  
}

#[derive(Debug, Deserialize)]
pub struct ChamaLoadQuaranteeSettingDto {
    pub id:Option<i64>,                 
    pub chama_id:i64,           
    pub centage_required:f64, 
}