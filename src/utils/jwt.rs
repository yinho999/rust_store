use std::env;

use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::server_error::ServerError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String, // this is the email
    pub exp: usize,
    pub company: String,
}

pub struct SlimUser {
    pub email: String,
    pub company: String,
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> SlimUser {
        SlimUser {
            email: claims.sub,
            company: claims.company,
        }
    }
}

impl Claims {
    pub fn new(email: &str, company: &str) -> Claims {
        Claims {
            sub: email.to_string(),
            company: company.to_string(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}

pub fn create_token(email: &str, company: &str) -> Result<String, ServerError> {
    let claims = Claims::new(email, company);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&get_secret()),
    )
    .map_err(|err| {
        println!("{}", err);
        ServerError::InternalServerError(err.to_string())
    })
}

pub fn decode_token(token: &str) -> Result<SlimUser, ServerError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&get_secret()),
        &Validation::default(),
    )
    .map(|data| data.claims.into())
    .map_err(|err| {
        println!("{}", err);
        ServerError::InternalServerError(err.to_string())
    })
}

// get secret as bytes from .env file using dot env
fn get_secret() -> Vec<u8> {
    env::var("SECRET")
        .expect("SECRET must be set in .env file")
        .into_bytes()
}
