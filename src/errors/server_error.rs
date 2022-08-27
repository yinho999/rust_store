use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServerError {
    #[display(fmt = "{ }", _0)]
    NotFound(String),
    #[display(fmt = "{ }", _0)]
    BadRequest(String),
    #[display(fmt = "{ }", _0)]
    InternalServerError(String),

    // unauthorized
    #[display(fmt = "{ }", _0)]
    Unauthorized(String),
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServerError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            ServerError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ServerError::InternalServerError(msg) => HttpResponse::InternalServerError().json(msg),
            ServerError::Unauthorized(msg) => HttpResponse::Unauthorized().json(msg),
        }
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ServerError::NotFound(_) => StatusCode::NOT_FOUND,
            ServerError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServerError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
