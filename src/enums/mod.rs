use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug)]
pub enum LoanRepaymentFrequecyEnum{
    WEEKLY,
    BIWEEKLY,
    MONTHLY
  }