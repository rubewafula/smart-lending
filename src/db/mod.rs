use std::env;
use sqlx::MySqlPool;


pub async fn connect() -> MySqlPool {

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    MySqlPool::connect(&db_url).await.expect("Failed to connect to DB")
}