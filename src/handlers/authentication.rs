use super::pg_pool_handler;
use crate::errors::server_error::ServerError;
use crate::models::user::AuthenticateUser;
use crate::utils::jwt::create_token;
use crate::{db_connection::PgPool, errors::application_error::ApplicationError};
use actix_identity::Identity;
use actix_web::{delete, post, web, HttpMessage, HttpRequest, HttpResponse};
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use std::sync::Mutex;

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    auth_user: web::Json<AuthenticateUser>,
    pool: web::Data<PgPool>,
    generator: web::Data<Mutex<AesGcmCsrfProtection>>,
) -> Result<HttpResponse, ServerError> {
    let generator = generator.lock().unwrap();
    // handle pool
    let pg_pool = pg_pool_handler(pool)?;
    // login user
    let user = auth_user.login(&pg_pool).map_err(|err| {
        // match server error
        match err {
            // if error is not found, return bad request
            ApplicationError::DBError(diesel::result::Error::NotFound) => {
                ServerError::BadRequest(format!("{}", err))
            }
            // if error is internal server error, return internal server error
            _ => ServerError::InternalServerError(format!("{}", err)),
        }
    })?;

    // create jwt token
    let token = create_token(&user.email, &user.company)?;
    Identity::login(&req.extensions(), token)
        .map_err(|err| ServerError::InternalServerError(format!("{}", err)))?;

    // Response has csrf token for security
    let (csrf_token, csrf_cookie) = generator
        .generate_token_pair(None, 300)
        .expect("Failed to generate token");
    let response = HttpResponse::Ok()
        .append_header(("x-csrf-token", csrf_token.b64_string()))
        .append_header(("x-csrf-token-cookie", csrf_cookie.b64_string()))
        .json(user);
    Ok(response)
}

#[delete("/logout")]
pub async fn logout(id: Identity) -> Result<HttpResponse, ServerError> {
    id.logout();
    Ok(HttpResponse::Ok().into())
}
