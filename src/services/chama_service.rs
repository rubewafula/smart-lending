use chrono::{Duration, NaiveDateTime, Utc};
use sqlx::MySqlPool;
use tracing::{info, error};
use crate::models::chama;
use crate::dtos::chama::ChamaDto;
use crate::repositories::crud_repository_trait::CrudRepositoryTrait;
use crate::repositories::data_repository;


pub async fn create_new_chama(pool:&MySqlPool, user_id:&str, payload:&ChamaDto) -> i64{

    let chama_repository = data_repository::DataRepository::<chama::Chama> {
        pool,
        table_name: "chama",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };
    let now_eat: NaiveDateTime = (Utc::now() + Duration::hours(3)).naive_utc();
    let chama = chama::Chama {
        id:None,
        name:payload.name.clone(),
        contact_number:payload.contact_number.clone(),
        location:payload.location.clone(),
        size:payload.size.clone(),
        contact_person:payload.contact_person.clone(),
        reg_number:payload.reg_number.clone().unwrap(),
        created_at:now_eat,
        updated_at:now_eat,
        created_by:user_id.parse::<i64>().unwrap()
    };

    let result:i64 = match chama_repository.record_exists(&"name", &payload.name).await {
        Ok(exists) => { if exists { -1 } else { 1 } },
        Err(_) => 0,
    };
    if result == -1 || result == 0{
        error!("Failed to create new chama: {:?}", result);
        return result;
    }
    let result = chama_repository.insert(&chama).await;
    if result.is_err() {
        error!("Failed to create new chama: {:?}", result);
        return 0;
    }

    result.unwrap()

}

pub async fn update_chama(pool:&MySqlPool, user_id:&str, payload:&ChamaDto) -> i64{

    let chama_repository = data_repository::DataRepository::<chama::Chama> {
        pool,
        table_name: "chama",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };
    let now_eat: NaiveDateTime = (Utc::now() + Duration::hours(3)).naive_utc();

    let result:i64 = match chama_repository.record_exists(&"name", &payload.name).await {
        Ok(exists) => { if exists { 1 } else { -1 } },
        Err(_) => 0,
    };
    if result == -1 || result == 0{
        error!("Failed to create new chama: {:?}", result);
        return result;
    }

    let chama = chama::Chama {
        id:payload.id.clone(),
        name:payload.name.clone(),
        contact_number:payload.contact_number.clone(),
        location:payload.location.clone(),
        size:payload.size.clone(),
        contact_person:payload.contact_person.clone(),
        reg_number:payload.reg_number.clone().unwrap(),
        created_at:now_eat,
        updated_at:now_eat,
        created_by:user_id.parse::<i64>().unwrap()
    };

    
   
    let result = chama_repository.update_by_id(&payload.id.unwrap(), &chama).await;
    if result.is_err() {
        error!("Failed to create new chama: {:?}", result);
        return 0;
    }

    result.unwrap() as i64

}

pub async fn add_member(pool:&MySqlPool, user_id:&str, payload:&ChamaDto) -> i64{
    let chama_member_repository = data_repository::DataRepository::<chama::Chama> {
        pool,
        table_name: "chama",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

}