use std::env;

use chrono::{Duration, NaiveDateTime};
use sqlx::MySql;
use sqlx::MySqlPool;
use sqlx::Transaction;
use tracing::{info, error};
use crate::dtos::chama::ChamaLoadLimitDto;
use crate::dtos::chama::ChamaLoanQuaranteeSettingDto;
use crate::dtos::chama::ChamaLoanApproverDto;
use crate::dtos::chama::ChamaLoanRepaymentLimitDto;
use crate::dtos::chama::ChamaMemberDetailDto;
use crate::dtos::chama::ChamaPositionDetailDto;
use crate::dtos::chama::ChamaPositionDto;
use crate::models::chama;
use crate::dtos::chama::{ChamaDto, ChamaMemberApproveDto};
use crate::repositories::crud_repository_trait::CrudRepositoryTrait;
use crate::repositories::data_repository;
use crate::utils;
use sqlx::Row;


pub async fn create_new_chama(pool:&MySqlPool, user_id:&str, payload:&ChamaDto) -> i64{

    let mut tx: Transaction<'_, MySql> = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to start db transaction: {}", e);
            return 0;
        }
    };

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
    
    let result = chama_repository.insert_trx(&mut tx, &chama).await;
    if result.is_err() {
        error!("Failed to create new chama: {:?}", result);
        return 0;
    }
    let chama_member_repository = data_repository::DataRepository::<chama::ChamaMember> {
        pool,
        table_name: "chama_member",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };
    let chama_id = result.unwrap();

    let chama_member =  chama::ChamaMember {
        id:None,                 
        user_id:user_id.parse::<i64>().unwrap(),         
        chama_id:chama_id.clone(),
        position:1,
        contribution_amount:0.0,           
        created_at: now_eat,
        updated_at:now_eat,
        created_by:user_id.parse::<i64>().unwrap(),
        is_active:1
    };

    let member = chama_member_repository.insert_trx(&mut tx, &chama_member).await;
    if member.is_err() {
        error!("Failed to create new chama member: {:?}", member);
        return 0;
    }
    info!("Chama member created: {:?}", chama);

    if tx.commit().await.is_ok() {
        return  chama_id;
    } else {
        return 0
    }

    

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


pub async fn join_chama(pool:&MySqlPool, user_id:&str, invite_hash:&String) -> i64{

    let chama_invite_repository = data_repository::DataRepository::<chama::ChamaInvite> {
        pool,
        table_name: "chama_invite",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let result = match chama_invite_repository.find_by(&"invite_hash", &invite_hash).await {
        Ok(mut result) => result.pop(), 
        Err(_) => None,
    };

    if let Some(chama_invite) = result {

        let chama_member_repository = data_repository::DataRepository::<chama::ChamaMember> {
            pool,
            table_name: "chama_member",
            pk_column: "id",
            phantom: std::marker::PhantomData,
        };

        let now_eat: NaiveDateTime = utils::now_eat();
        let chama_member =  chama::ChamaMember {
            id:None,                 
            user_id:user_id.parse::<i64>().unwrap(),         
            chama_id:chama_invite.chama_id,
            position:0,
            contribution_amount:0.0,           
            created_at: now_eat,
            updated_at:now_eat,
            created_by:chama_invite.invited_by,
            is_active:0
        };
        let result = chama_member_repository.insert(&chama_member).await;
        if result.is_err() {
            error!("Failed to create new chama: {:?}", result);
            return 0;
        }
        return result.unwrap() as i64;
    }
    return 0;
}

pub async fn approve_member(pool:&MySqlPool, user_id:&str, payload:&ChamaMemberApproveDto) -> i64{

    let chama_member_repository = data_repository::DataRepository::<chama::ChamaMember> {
        pool,
        table_name: "chama_member",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    match chama_member_repository.find_by(&"id", &payload.user_id.to_string()).await {
        Ok(mut result) => {
            if let Some(mut chama_member) = result.pop() {
                chama_member.is_active = payload.is_active.clone();
                chama_member.position = payload.position.clone();
                chama_member.contribution_amount = payload.contribution_amount.clone();
                chama_member.updated_at = utils::now_eat();
                chama_member.created_by = user_id.parse::<i64>().unwrap();
                let result = chama_member_repository.update_by_id(&payload.user_id, &chama_member).await;
                if result.is_err() {
                    error!("Failed to approve member: {:?}", result);
                    return 0;
                }
                return result.unwrap() as i64;
            }
            0
        },
        Err(_) => {
            error!("Failed to find chama member by id");
            0
        },
    }

}

pub async fn remove_member(pool:&MySqlPool, member_id:&i64) -> i64{

    let chama_member_repository = data_repository::DataRepository::<chama::ChamaMember> {
        pool,
        table_name: "chama_member",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    match chama_member_repository.find_by(&"id", &member_id.to_string()).await {
        Ok(mut result) => {
            if let Some(mut chama_member) = result.pop() {
                chama_member.is_active = 0;
                chama_member.updated_at = utils::now_eat();
                let result = chama_member_repository.update_by_id(&member_id, &chama_member).await;
                if result.is_err() {
                    error!("Failed to approve member: {:?}", result);
                    return 0;
                }
                return result.unwrap() as i64;
            }
            0
        },
        Err(_) => {
            error!("Failed to find chama member by id");
            0
        },
    }

}

pub async fn get_chama_roles(pool:&MySqlPool, user_id:&str, chama_id:&str) -> Result<Vec<String>, sqlx::Error> {

    let results = sqlx::query(
        "select name from  chama_member cm 
        inner join chama_position cp on cm.position = cp.id 
        inner join auth_group ag on cp.role_id = ag.id 
        where cm.chama_id = ? and cm.user_id = ?"
    )
    .bind(chama_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    if !results.is_empty() {
        let mut roles = Vec::new();
        for row in results {
            roles.push(row.try_get::<String, _>("name")?);
        }
        info!("Found results from query");
        return Ok(roles);
    } else {
        error!("Could not find user roles");
        return Ok(Vec::new());
    }

}

pub async fn get_members(pool:&MySqlPool, user_id:&str, chama_id:&str) -> Result<Vec<ChamaMemberDetailDto>, sqlx::Error> {

    let results = sqlx::query(
        "select first_name, last_name, email, username,
            chama_position, cm.contribution_amount, cm.user_id, cm.chama_id, cm.is_active
        from chama_member cm 
        inner join auth_user au on au.id = cm.user_id 
        inner join chama_position cp on cp.id = cm.position 
        where cm.chama_id=? and 
        exists (select 1 from chama_member where user_id = ?)"
    )
    .bind(chama_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    if !results.is_empty() {
        let mut members: Vec<ChamaMemberDetailDto> = Vec::new();
        for row in results {
            let cmdto = ChamaMemberDetailDto {
                id: None,
                user_id: row.try_get::<i64, _>("user_id").unwrap(),
                chama_id: row.try_get::<i64, _>("chama_id").unwrap(),
                first_name: row.try_get::<String, _>("first_name").unwrap(),
                last_name: row.try_get::<String, _>("last_name").unwrap(),
                position: row.try_get::<String, _>("chama_position").unwrap(),
                contribution_amount: row.try_get::<f64, _>("contribution_amount").unwrap(),
                is_active: row.try_get::<i8, _>("is_active").unwrap(),
                phone_number: row.try_get::<String, _>("username").unwrap(),
                email: Some(row.try_get::<String, _>("email").unwrap()),
            };
            members.push(cmdto);
        }
        info!("Found results from query");
        return Ok(members);
    } else {
        error!("Could not find user roles");
        return Ok(Vec::new());
    }
}

pub async fn get_loan_approvers(pool:&MySqlPool, user_id:&str, chama_id:&str) -> Result<Vec<ChamaMemberDetailDto>, sqlx::Error> {

    let results = sqlx::query(
        "select first_name, last_name, email, username,
            chama_position, cm.contribution_amount, cm.user_id, cm.chama_id, cm.is_active
        from chama_member cm 
        inner join auth_user au on au.id = cm.user_id 
        inner join chama_position cp on cp.id = cm.position 
        inner join chama_loan_approver cla on cp.id = cla.approver_position_id
        where cm.chama_id=? and 
        exists (select 1 from chama_member where user_id = ?)"
    )
    .bind(chama_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    if !results.is_empty() {
        let mut members: Vec<ChamaMemberDetailDto> = Vec::new();
        for row in results {
            let cmdto = ChamaMemberDetailDto {
                id: None,
                user_id: row.try_get::<i64, _>("user_id").unwrap(),
                chama_id: row.try_get::<i64, _>("chama_id").unwrap(),
                first_name: row.try_get::<String, _>("first_name").unwrap(),
                last_name: row.try_get::<String, _>("last_name").unwrap(),
                position: row.try_get::<String, _>("chama_position").unwrap(),
                contribution_amount: row.try_get::<f64, _>("contribution_amount").unwrap(),
                is_active: row.try_get::<i8, _>("is_active").unwrap(),
                phone_number: row.try_get::<String, _>("username").unwrap(),
                email: Some(row.try_get::<String, _>("email").unwrap()),
            };
            members.push(cmdto);
        }
        info!("Found results from query");
        return Ok(members);
    } else {
        error!("Could not find user roles");
        return Ok(Vec::new());
    }
}

pub async fn get_chama_positions(pool:&MySqlPool) -> Result<Vec<ChamaPositionDetailDto>, sqlx::Error> {

    let results = sqlx::query(
        "select cp.id, cp.chama_position, au.name
        from chama_position cp
        inner join auth_group au on cp.role_id = ag.id"
    )
    .fetch_all(pool)
    .await?;

    if !results.is_empty() {
        let mut positions: Vec<ChamaPositionDetailDto> = Vec::new();
        for row in results {
            let pos = ChamaPositionDetailDto {
                id: Some(row.try_get::<i64, _>("id").unwrap()),
                position: row.try_get::<String, _>("chama_position").unwrap(),
                role: row.try_get::<String, _>("name").unwrap()
                
            };
            positions.push(pos);
        }
        info!("Found results from query");
        return Ok(positions);
    } else {
        error!("Could not find user roles");
        return Ok(Vec::new());
    }
}


pub async fn get_guaranter_settings(pool:&MySqlPool, chama_id:&i64) -> Result<Vec<ChamaLoanQuaranteeSettingDto>, sqlx::Error> {

    let results = sqlx::query(
        "select gs.id, gs.centage_required, chama_id
        from chama_loan_quarantee_setting gs where chama_id = ?"
    )
    .bind(chama_id)
    .fetch_all(pool)
    .await?;

    if !results.is_empty() {
        let mut settings: Vec<ChamaLoanQuaranteeSettingDto> = Vec::new();
        for row in results {
            let setting = ChamaLoanQuaranteeSettingDto {
                id: Some(row.try_get::<i64, _>("id").unwrap()),
                centage_required: row.try_get::<f64, _>("centage_required").unwrap(),
                chama_id: row.try_get::<i64, _>("chama_id").unwrap()
            };
            settings.push(setting);
        }
        info!("Found results from query");
        return Ok(settings);
    } else {
        error!("Could not find user roles");
        return Ok(Vec::new());
    }
}


pub async fn add_loan_approver(pool:&MySqlPool, user_id:&str, payload:&ChamaLoanApproverDto) -> Result<i64, sqlx::Error> {
    let chama_loan_approver_repository = data_repository::DataRepository::<chama::ChamaLoanApprover> {
        pool,
        table_name: "chama_loan_approver",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let chama_loan_approver =  chama::ChamaLoanApprover {
        id:None,                 
        chama_id:payload.chama_id.clone(),
        approver_position_id:payload.approver_position_id.clone(),           
        created_at: now_eat,
        updated_at:now_eat,
        created_by:user_id.parse::<i64>().unwrap(),
        is_active:1
    };
    let result = chama_loan_approver_repository.insert(&chama_loan_approver).await;
    if result.is_err() {
        error!("Failed to create new chama loan approver: {:?}", result);
        return Err(result.err().unwrap());
    }
    Ok(result.unwrap() as i64)
}


pub async fn add_guaranter_setting(pool:&MySqlPool, payload:&ChamaLoanQuaranteeSettingDto) -> Result<i64, sqlx::Error> {
    let chama_loan_guarantee_repository = data_repository::DataRepository::<chama::ChamaLoanQuaranteeSetting> {
        pool,
        table_name: "chama_loan_quarantee_setting",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let chama_loan_guarantee =  chama::ChamaLoanQuaranteeSetting {
        id:None,                 
        chama_id:payload.chama_id.clone(),
        centage_required:payload.centage_required.clone(),           
        created_at: now_eat,
        updated_at:now_eat,
        is_active:1
    };
    let result = chama_loan_guarantee_repository.insert(&chama_loan_guarantee).await;
    if result.is_err() {
        error!("Failed to create new chama loan approver: {:?}", result);
        return Err(result.err().unwrap());
    }
    Ok(result.unwrap() as i64)
}


pub async fn add_loan_limit(pool:&MySqlPool, payload:&ChamaLoadLimitDto) -> Result<i64, sqlx::Error> {
    let chama_loan_limit_repository = data_repository::DataRepository::<chama::ChamaLoanLimit> {
        pool,
        table_name: "chama_loan_limit",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let chama_loan_limit =  chama::ChamaLoanLimit {
        id:None,                 
        chama_id:payload.chama_id.clone(),
        amount:payload.amount.clone().unwrap(),           
        created_at: now_eat,
        updated_at:now_eat,
        centage_member_savings:payload.centage_member_savings.clone().unwrap(),
        is_active:1
    };
    let result = chama_loan_limit_repository.insert(&chama_loan_limit).await;
    if result.is_err() {
        error!("Failed to create new chama loan approver: {:?}", result);
        return Err(result.err().unwrap());
    }
    Ok(result.unwrap() as i64)
}

pub async fn add_loan_repayment_limit(pool:&MySqlPool, payload:&ChamaLoanRepaymentLimitDto) -> Result<i64, sqlx::Error> {
    let chama_loan_repayment_limit_repository = data_repository::DataRepository::<chama::ChamaLoanRepaymentLimit> {
        pool,
        table_name: "chama_loan_repayment_limit",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let chama_loan_repayment_limit =  chama::ChamaLoanRepaymentLimit {
        id:None,                 
        chama_id:payload.chama_id.clone(),
        max_repayment_months:payload.max_repayment_months.clone(),
        repayment_frequency:payload.repayment_frequency.clone(),          
        created_at: now_eat,
        updated_at:now_eat
        
    };  
    let result = chama_loan_repayment_limit_repository.insert(&chama_loan_repayment_limit).await;
    if result.is_err() {
        error!("Failed to create new chama loan approver: {:?}", result);
        return Err(result.err().unwrap());
    }
    Ok(result.unwrap() as i64)
}

pub async fn create_chama_position(pool:&MySqlPool, payload:&ChamaPositionDto) -> Result<i64, sqlx::Error> {
    let chama_position_repository = data_repository::DataRepository::<chama::ChamaPosition> {
        pool,
        table_name: "chama_position",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let chama_position =  chama::ChamaPosition {
        id:None,                 
        chama_position:payload.chama_position.clone(),
        role_id:payload.role_id.clone(),           
        created_at: now_eat,
        updated_at:now_eat
    };
    let result = chama_position_repository.insert(&chama_position).await;
    if result.is_err() {
        error!("Failed to create new chama loan approver: {:?}", result);
        return Err(result.err().unwrap());
    }
    Ok(result.unwrap() as i64)
}


pub async fn remove_from_loan_approver(pool:&MySqlPool, position_id:&i64) -> Result<i64, sqlx::Error> {
    let chama_loan_approver_repository = data_repository::DataRepository::<chama::ChamaLoanApprover> {
        pool,
        table_name: "chama_loan_approver",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let result = match chama_loan_approver_repository.find_by(&"id", &position_id.to_string()).await {
        Ok(mut result) => result.pop(), 
        Err(_) => None,
    };
    if result.is_none() {
        error!("Failed to find chama loan approver by id");
        return Ok(0);
    }

    let mut chama_loan_approver =  result.unwrap();
    chama_loan_approver.is_active = 0;
    chama_loan_approver.updated_at = now_eat;

    let result = chama_loan_approver_repository.update_by_id(&position_id, &chama_loan_approver).await;
    if result.is_err() {
        error!("Failed to create new chama loan approver: {:?}", result);
        return Err(result.err().unwrap());
    }
    Ok(result.unwrap() as i64)
}


pub async fn remove_guarantee_setting(pool:&MySqlPool, g_id:&i64) -> Result<i64, sqlx::Error> {
    let chama_loan_guarantee_repository = data_repository::DataRepository::<chama::ChamaLoanQuaranteeSetting> {
        pool,
        table_name: "chama_loan_quarantee_setting",
        pk_column: "id",
        phantom: std::marker::PhantomData,
    };

    let now_eat: NaiveDateTime = utils::now_eat();
    let result = match chama_loan_guarantee_repository.find_by(&"id", &g_id.to_string()).await {
        Ok(mut result) => result.pop(), 
        Err(_) => None,
    };
    if result.is_none() {
        error!("Failed to find chama loan approver by id");
        return Ok(0);
    }

    let mut g_setting =  result.unwrap();
    g_setting.is_active = 0;
    g_setting.updated_at = now_eat;

    let result = chama_loan_guarantee_repository.update_by_id(&g_id, &g_setting).await;
    if result.is_err() {
        error!("Failed to create new chama loan approver: {:?}", result);
        return Err(result.err().unwrap());
    }
    Ok(result.unwrap() as i64)
}