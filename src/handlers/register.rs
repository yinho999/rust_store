use actix_web::{web, HttpResponse};

use crate::{
    db_connection::PgPool,
    errors::server_error::ServerError,
    models::user::{RegisterUser, User},
};

use super::products::pool_handler;

pub fn register(
    new_user: web::Json<RegisterUser>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let pool = pool_handler(pool)?;
    // validate user password and password confirmation
    let register_user = new_user
        .into_inner()
        .validate()
        .map_err(|err| ServerError::InternalServerError(err.to_string()))?;

    // create user
    User::create(&register_user, &pool)
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|err| ServerError::InternalServerError(err.to_string()))
}
