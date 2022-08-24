use bcrypt::BcryptError;
use derive_more::Display;
use diesel::result;

#[derive(Debug, Display)]
pub enum ApplicationError {
    #[display(fmt = "{ }", _0)]
    PasswordNotMatch(String),
    #[display(fmt = "{ }", _0)]
    WrongPassword(String),
    #[display(fmt = "{ }", _0)]
    DBError(result::Error),
    #[display(fmt = "{ }", _0)]
    HashError(BcryptError),
}

// From BcryptError to ApplicationError
impl From<BcryptError> for ApplicationError {
    fn from(error: bcrypt::BcryptError) -> Self {
        ApplicationError::HashError(error)
    }
}

// From diesel::result::Error to ApplicationError
impl From<result::Error> for ApplicationError {
    fn from(error: result::Error) -> Self {
        ApplicationError::DBError(error)
    }
}
