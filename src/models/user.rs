use crate::diesel::ExpressionMethods;
use crate::errors::application_error::ApplicationError;
use crate::schema::users;
use crate::schema::users::dsl::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Create a struct to represent a user.
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub email: String,
    pub company: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: NaiveDateTime,
}

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use diesel::PgConnection;
use diesel::RunQueryDsl;

impl User {
    pub fn hash_password(plain_password: &str) -> Result<String, ApplicationError> {
        hash(plain_password, DEFAULT_COST).map_err(|err| ApplicationError::HashError(err))
    }

    pub fn create(
        register_user: &RegisterUser,
        conn: &PgConnection,
    ) -> Result<User, ApplicationError> {
        let hashed_password = Self::hash_password(&register_user.password)?;
        let user = NewUser {
            email: register_user.email.to_string(),
            company: register_user.company.to_string(),
            password: hashed_password,
            created_at: Local::now().naive_local(),
        };
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .map_err(|_| ApplicationError::DBError(diesel::result::Error::NotFound))
    }
}

// Struct for inserting a new user into database
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

// Register user model
#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub password_confirmation: String,
}

impl RegisterUser {
    pub fn validate(self) -> Result<RegisterUser, ApplicationError> {
        if self.password != self.password_confirmation {
            return Err(ApplicationError::PasswordNotMatch(
                "Password and password confirmation do not match".to_string(),
            ));
        }
        Ok(self)
    }
}

// Authenticate user model
#[derive(Deserialize)]
pub struct AuthenticateUser {
    pub email: String,
    pub password: String,
}
use crate::schema::users::dsl::email;
use diesel::QueryDsl;

impl AuthenticateUser {
    // login
    pub fn login(&self, conn: &PgConnection) -> Result<User, ApplicationError> {
        // find records with the same email
        let mut records = users.filter(email.eq(&self.email)).load::<User>(conn)?;

        // Get the user from the records
        let user = records.pop().ok_or(ApplicationError::WrongPassword(
            "Email or password is incorrect".to_string(),
        ))?;
        // Verify the password
        let password_is_valid =
            verify(&self.password, &user.password).map_err(|e| ApplicationError::HashError(e))?;
        if password_is_valid {
            Ok(user)
        } else {
            Err(ApplicationError::WrongPassword(
                "Email or password is incorrect".to_string(),
            ))
        }
    }
}
