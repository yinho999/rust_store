use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum UserError {
    #[display(fmt = "{ }", _0)]
    NotFound(String),
    #[display(fmt = "{ }", _0)]
    BadRequest(String),
    #[display(fmt = "{ }", _0)]
    InternalServerError(String),
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            UserError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            UserError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            UserError::InternalServerError(msg) => HttpResponse::InternalServerError().json(msg),
        }
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            UserError::NotFound(_) => StatusCode::NOT_FOUND,
            UserError::BadRequest(_) => StatusCode::BAD_REQUEST,
            UserError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
