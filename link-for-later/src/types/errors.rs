use std::{error, fmt};

#[derive(Debug)]
pub enum App {
    ItemNotFound,
    NoDatabaseSetup,
    DatabaseError,
    NotSupported,
    #[cfg(test)]
    TestError,
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ItemNotFound => write!(f, "item not found"),
            Self::NoDatabaseSetup => write!(f, "no database setup"),
            Self::DatabaseError => write!(f, "database error setup"),
            Self::NotSupported => write!(f, "operation not supported"),
            #[cfg(test)]
            Self::TestError => write!(f, "test error"),
        }
    }
}

impl error::Error for App {}
