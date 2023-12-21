use std::{error, fmt};

#[derive(Debug)]
pub enum App {
    NotSupported,

    DatabaseError,
    ItemNotFound,
    UserAlreadyExists,
    UserNotFound,

    #[cfg(test)]
    TestError,
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "operation not supported"),
            Self::DatabaseError => write!(f, "database error"),
            Self::ItemNotFound => write!(f, "item not found"),
            Self::UserAlreadyExists => write!(f, "user already exist"),
            Self::UserNotFound => write!(f, "user not found"),
            #[cfg(test)]
            Self::TestError => write!(f, "test error"),
        }
    }
}

impl error::Error for App {}
