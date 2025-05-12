use sqlx::MySqlPool;
use tracing::{info, error};
use sqlx::Row;

pub async fn get_user_balance(pool:&MySqlPool, user_id:&i64) -> Result<(Option<f64>, Option<f64>), sqlx::Error>{

   
    let results = sqlx::query(
        "SELECT ac.balance as account_balance, cb.balance  as credit_balance FROM account_balance ac
         LEFT JOIN credit_balance cb on ac.user_id = cb.user_id WHERE ac.user_id = ?"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = results {
            info!("Found results from query");
            Ok((row.try_get("account_balance")?, row.try_get("credit_balance")?))
    } else {
        error!("Could not find account balance information");
        Ok((Some(0 as f64), Some(0 as f64)))
    }
        
}