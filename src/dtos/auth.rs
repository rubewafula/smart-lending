use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginInfo {
   pub username: String,
   pub password: String

}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub balance: f64,
    pub credit_balance: f64
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

#[derive(Deserialize)]
pub struct SignupInfo {
    pub username: String,
    pub password: String,
    pub last_name: String,
    pub first_name: String,
    pub email:Option<String>

}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordDto {
    pub email: String,
}


#[derive(Debug, Deserialize)]
pub struct ResetPasswordDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordChangeRequestDto {
    pub old_password: String,
    pub new_password: String,
}


