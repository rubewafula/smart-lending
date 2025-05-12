use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct IncomeRange {
  pub id:i32,
  pub min_amount:f64,
  pub max_amount:f64,
  pub status:String,
  pub created_at:NaiveDateTime,
  pub updated_at:NaiveDateTime,
  pub created_by:i32,
} 
