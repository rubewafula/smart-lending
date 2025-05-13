use std::env;

use chrono::{Duration, NaiveDateTime, Utc};
use sqlx::MySqlPool;
use tracing::{info, error};
use crate::models::chama;
use crate::models::auth;
use crate::dtos::chama::{ChamaDto, ChamaMemberDto};
use crate::repositories::crud_repository_trait::CrudRepositoryTrait;
use crate::repositories::data_repository;
use crate::utils;


pub async fn create_new_chama(pool:&MySqlPool, user_id:&str, payload:&ChamaDto) -> i64{

    let chama_repository = data_repository::DataRepository::<chama::Chama> {
        pool,
        table_name: "chama",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };
    let now_eat = utils::now_eat();

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
    let now_eat: NaiveDateTime = utils::now_eat();

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

pub async fn add_member(pool:&MySqlPool, user_id:&str, payload:&ChamaMemberDto) -> i64{
    let chama_member_repository = data_repository::DataRepository::<chama::ChamaMember> {
        pool,
        table_name: "chama_member",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let chama_repository = data_repository::DataRepository::<chama::Chama> {
        pool,
        table_name: "chama",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };


    let user_repository = data_repository::DataRepository::<auth::AuthUser> {
        pool,
        table_name: "auth_user",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let result:i64 = match chama_repository.record_exists(&"id", &payload.chama_id.to_string()).await {
        Ok(exists) => { if exists { 1 } else { -1 } },
        Err(_) => 0,
    };

    if result == -1 || result == 0{
        error!("Failed to create new chama: {:?}", result);
        return result;
    }

    let result:i64 = match user_repository.record_exists(&"id", &payload.user_id.to_string()).await {
        Ok(exists) => { if exists { 1 } else { -1 } },
        Err(_) => 0,
    };

    if result == -1 || result == 0{
        error!("Failed to create new chama: {:?}", result);
        return result;
    }

    let now_eat: NaiveDateTime = utils::now_eat();
    let chama_member =  chama::ChamaMember {
        id:None,                 
        user_id:payload.user_id.clone(),           
        chama_id:payload.chama_id.clone(),
        position:payload.position.clone(),
        contribution_amount:payload.contribution_amount.clone(),           
        created_at: now_eat,
        updated_at:now_eat,
        created_by:user_id.parse::<i64>().unwrap(),
        is_active:1
    };
    let result = chama_member_repository.insert(&chama_member).await;
    if result.is_err() {
        error!("Failed to create new chama: {:?}", result);
        return 0;
    }

    result.unwrap()


}

pub async fn get_invite(pool:&MySqlPool, user_id:&str, chama_id:&i64) -> String{

    let chama_repository = data_repository::DataRepository::<chama::Chama> {
        pool,
        table_name: "chama",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let result:i64 = match chama_repository.record_exists(&"id", &chama_id.to_string()).await {
        Ok(exists) => { if exists { 1 } else { -1 } },
        Err(_) => 0,
    };

    if result == -1 || result == 0{
        error!("Failed to create new chama: {:?}", result);
        return result.to_string()
    }

    let chama_invite_repository = data_repository::DataRepository::<chama::ChamaInvite> {
        pool,
        table_name: "chama_invite",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let hash_string = utils::generate_invite_hash_64();

    let chama_invite =  chama::ChamaInvite {
        id:None,
        chama_id:chama_id.clone(),
        invited_by:user_id.parse::<i64>().unwrap(),
        expiry_date: now_eat + Duration::days(7),
        invite_hash:hash_string.clone(),
        created_at: now_eat,
        updated_at:now_eat,
       
    };
    let result = chama_invite_repository.insert(&chama_invite).await;
    if result.is_err() {
        error!("Failed to create new chama: {:?}", result);
        return "0".to_string();
    }

    let vurl:String = env::var("CHAMA_INVITE_URL").unwrap();
    format!("{}/invite/{}", vurl, hash_string)
}