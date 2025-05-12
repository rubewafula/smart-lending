use std::ops::DerefMut;

use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value as JsonValue;
use sqlx::{mysql::MySqlRow, Error, FromRow, MySql, MySqlPool, Transaction};
use crate::repositories::crud_repository_trait::CrudRepositoryTrait;

pub struct DataRepository<'a, T> {
    pub pool: &'a MySqlPool,
    pub table_name: &'a str,
    pub pk_column: &'a str,
    pub phantom: std::marker::PhantomData<T>,
}

fn extract_fields(item: &impl Serialize) -> (Vec<String>, Vec<JsonValue>) {
    let map = serde_json::to_value(item)
        .expect("Serialize error")
        .as_object()
        .expect("Expected struct")
        .clone();

    let mut columns = Vec::new();
    let mut values = Vec::new();

    for (key, val) in map {
        if !val.is_null() {
            columns.push(key);
            values.push(val);
        }
    }

    (columns, values)
}


#[async_trait]
impl<'a, T> CrudRepositoryTrait<T> for DataRepository<'a, T>
where
    T: Serialize + for<'de> serde::Deserialize<'de> + for<'r> FromRow<'r, MySqlRow> + Send + Sync + Unpin,
{
    async fn insert(&self, item: &T) -> Result<i64, Error> {
        let (columns, values) = extract_fields(item);

        let placeholders: Vec<String> = (0..values.len()).map(|_| "?".to_string()).collect();

        let sql = format!(
            "INSERT INTO `{}` ({}) VALUES ({})",
            self.table_name,
            columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", "),
            placeholders.join(", ")
        );
        let mut query = sqlx::query(&sql);

        for val in values
        {
            match val {
                JsonValue::String(s) => query = query.bind(s),
                JsonValue::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        query = query.bind(i);
                    } else if let Some(f) = n.as_f64() {
                        query = query.bind(f);
                    }
                }
                JsonValue::Bool(b) => query = query.bind(b),
                _ => query = query.bind(None::<String>),
            }
        }

        let result = query.execute(self.pool).await?;
        Ok(result.last_insert_id().try_into().unwrap_or(0))
    }

    async fn insert_trx(&self, trx: &mut Transaction<'_, MySql>, item: &T) -> Result<i64, Error> {
        let (columns, values) = extract_fields(item);

        let placeholders: Vec<String> = (0..values.len()).map(|_| "?".to_string()).collect();

        let sql = format!(
            "INSERT INTO `{}` ({}) VALUES ({})",
            self.table_name,
            columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", "),
            placeholders.join(", ")
        );
        let mut query = sqlx::query(&sql);

        for val in values
        {
            match val {
                JsonValue::String(s) => query = query.bind(s),
                JsonValue::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        query = query.bind(i);
                    } else if let Some(f) = n.as_f64() {
                        query = query.bind(f);
                    }
                }
                JsonValue::Bool(b) => query = query.bind(b),
                _ => query = query.bind(None::<String>),
            }
        }
        
        
        let result = query.execute(trx.deref_mut()).await?;
        Ok(result.last_insert_id().try_into().unwrap_or(0))
        
    }


    
    async fn find_by_id(&self, id: &i64) -> Result<Option<T>, Error> {

        let sql = format!("SELECT * FROM `{}` WHERE id = ?", self.table_name);

        let row = sqlx::query_as::<_, T>(&sql)
            .bind(id)
            .fetch_optional(self.pool)
            .await?;

        Ok(row)
    }


    async fn update_by_id(&self, id: &i64, item: &T) -> Result<u64, Error> {

        let (columns, values) = extract_fields(item);
        let mut set_clauses = vec![];
        
    
        for col  in columns {
            set_clauses.push(format!("`{}` = ?", col));
        }

        let set_sql = set_clauses.join(", ");
        let sql = format!(
            "UPDATE `{}` SET {} WHERE id = ?",
            self.table_name, set_sql
        );

        let mut query = sqlx::query(&sql);

        for val in values {
            
            match val {
                JsonValue::String(s) => query = query.bind(s),
                JsonValue::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        query = query.bind(i);
                    } else if let Some(f) = n.as_f64() {
                        query = query.bind(f);
                    }
                }
                JsonValue::Bool(b) => query = query.bind(b),
                _ => query = query.bind(None::<String>),
            }
            
        }
        query = query.bind(id);
        let result = query.execute(self.pool).await?;
        Ok(result.rows_affected())
    }

    async fn delete_by_id(&self, id: &i64) -> Result<u64, Error> {
        let sql = format!("DELETE FROM `{}` WHERE id = ?", self.table_name);

        let result = sqlx::query(&sql)
        .bind(id)
        .execute(self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn find_by(&self, field: &str, value: &str) -> Result<Vec<T>, Error> {
        let query = format!("SELECT * FROM {} WHERE {} = ?", self.table_name, field);
    
        let rows = sqlx::query_as::<_, T>(&query)
            .bind(value)
            .fetch_all(self.pool)
            .await?;
    
        Ok(rows)
    }

    async fn record_exists(&self,field: &str, value: &str) -> Result<bool, sqlx::Error>
    {
        let sql = format!("SELECT EXISTS (SELECT 1 FROM {} WHERE {} = ?)", self.table_name, field);
        let exists: (i64,) = sqlx::query_as(&sql)
            .bind(value)
            .fetch_one(self.pool)
            .await?;
    
        Ok(exists.0 == 1)
    }
    
}


