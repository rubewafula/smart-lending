use crate::api::{user, auth}; 

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(user::routes())
        .merge(auth::routes())
}