use std::{error, fmt};

#[derive(Debug)]
pub enum App {
    NotSupported,
    ServerError,
    DatabaseError,
    ItemNotFound,
    UserAlreadyExists,
    UserNotFound,
    InvalidPassword,
    AuthorizationError,
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "operation not supported"),
            Self::ServerError => write!(f, "server error"),
            Self::DatabaseError => write!(f, "database error"),
            Self::ItemNotFound => write!(f, "item not found"),
            Self::UserAlreadyExists => write!(f, "user already exist"),
            Self::UserNotFound => write!(f, "user not found"),
            Self::InvalidPassword => write!(f, "incorrect password for user"),
            Self::AuthorizationError => write!(f, "invalid authorization token"),
        }
    }
}

impl error::Error for App {}
