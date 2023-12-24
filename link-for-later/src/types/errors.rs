use std::{error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum App {
    ServerError,
    DatabaseError,
    LinkNotFound,
    UserAlreadyExists,
    UserNotFound,
    IncorrectPassword,
    AuthorizationError,
    InvalidEmail,
    InvalidUrl,
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ServerError => write!(f, "server error"),
            Self::DatabaseError => write!(f, "database error"),
            Self::LinkNotFound => write!(f, "link item not found"),
            Self::UserAlreadyExists => write!(f, "user already regisered"),
            Self::UserNotFound => write!(f, "user not found"),
            Self::IncorrectPassword => write!(f, "incorrect password for user"),
            Self::AuthorizationError => write!(f, "invalid authorization token"),
            Self::InvalidEmail => write!(f, "invalid email"),
            Self::InvalidUrl => write!(f, "invalid url"),
        }
    }
}

impl error::Error for App {}
