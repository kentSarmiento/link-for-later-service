use std::{error, fmt};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Server {
    ItemNotFound,
    InternalServerError,
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ItemNotFound => write!(f, "item not found"),
            Self::InternalServerError => write!(f, "internal server error"),
        }
    }
}

impl error::Error for Server {}
