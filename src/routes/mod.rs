use crate::api::{user, auth, chama}; 

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(user::routes())
        .merge(auth::routes())
        .merge(chama::routes())
}