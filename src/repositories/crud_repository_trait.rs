use async_trait::async_trait;
use sqlx::{Error, MySql, Transaction};

#[async_trait]
pub trait CrudRepositoryTrait<T>
where
    T: serde::Serialize + Send + Sync + Sized,
{
    async fn insert(&self, item: &T) -> Result<i64, Error>;
    async fn insert_trx(&self, trx: &mut Transaction<'_, MySql>, item: &T) -> Result<i64, Error>;

    async fn find_by_id(&self, id: &i64) -> Result<Option<T>, Error>;

    async fn update_by_id(&self, id: &i64, item: &T) -> Result<u64, Error>;

    async fn delete_by_id(&self, id: &i64) -> Result<u64, Error>;
    
    async fn find_by(&self, field: &str, value: &str) -> Result<Vec<T>, Error>;

    async fn record_exists(&self,field: &str, value: &str) -> Result<bool, sqlx::Error>;
}