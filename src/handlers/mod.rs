use actix_identity::Identity;
use actix_utils::future::{ready, Ready};
use actix_web::{web, FromRequest};
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use data_encoding::BASE64;

use crate::{
    db_connection::{PgPool, PgPooledConnection},
    errors::server_error::ServerError,
    utils::jwt::{decode_token, SlimUser},
};

pub type LoggedUser = SlimUser;

pub mod authentication;
pub mod products;
pub mod register;

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, ServerError> {
    pool.get()
        .map_err(|e| ServerError::InternalServerError(e.to_string()))
}

impl FromRequest for LoggedUser {
    type Error = ServerError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();

        let generator = req.app_data::<AesGcmCsrfProtection>().ok_or(ready(Err(
            ServerError::InternalServerError("No CSRF protection".to_string()),
        )));
        let generator = match generator {
            Ok(g) => g,
            Err(e) => return e,
        };

        let crsf_token =
            req.headers()
                .get("x-csrf-token")
                .ok_or(ready(Err(ServerError::Unauthorized(
                    "No CSRF token".to_string(),
                ))));
        let crsf_token = match crsf_token {
            Ok(t) => t,
            Err(e) => return e,
        };

        let crsf_cookie =
            req.headers()
                .get("x-csrf-token-cookie")
                .ok_or(ready(Err(ServerError::Unauthorized(
                    "No CSRF token cookie".to_string(),
                ))));
        let crsf_cookie = match crsf_cookie {
            Ok(t) => t,
            Err(e) => return e,
        };

        // convert token into bytes
        let crsf_token_bytes = BASE64
            .decode(crsf_token.as_bytes())
            .map_err(|_| ServerError::Unauthorized("Invalid CSRF token".to_string()));
        let crsf_token_bytes = match crsf_token_bytes {
            Ok(t) => t,
            Err(e) => return ready(Err(e)),
        };

        // convert cookie into bytes
        let crsf_cookie_bytes = BASE64
            .decode(crsf_cookie.as_bytes())
            .map_err(|_| ServerError::Unauthorized("Invalid CSRF token cookie".to_string()));
        let crsf_cookie_bytes = match crsf_cookie_bytes {
            Ok(t) => t,
            Err(e) => return ready(Err(e)),
        };

        let parse_token = generator
            .parse_token(&crsf_token_bytes)
            .expect("Failed to parse token");

        let parse_cookie = generator
            .parse_cookie(&crsf_cookie_bytes)
            .expect("Failed to parse cookie");

        // verify token
        let result = generator.verify_token_pair(&parse_token, &parse_cookie);
        if !result {
            // return unauthorized if token is not valid
            return ready(Err(ServerError::Unauthorized(
                "Invalid CSRF token/cookie pair".to_string(),
            )));
        }

        // get user from token
        let user = Identity::from_request(&req, payload);
        let identity = match user.into_inner() {
            Ok(u) => u,
            Err(_) => return ready(Err(ServerError::Unauthorized("User not found".to_string()))),
        };
        // decode
        if let Ok(token) = identity.id() {
            let token = decode_token(&token);
            let token = match token {
                Ok(t) => t,
                Err(e) => return ready(Err(e)),
            };
            // return user if token is valid
            return ready(Ok(token));
        } else {
            return ready(Err(ServerError::Unauthorized("User not found".to_string())));
        }
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}
