use std::{error, fmt};

#[allow(dead_code)]
#[derive(Debug)]
pub enum App {
    ItemNotFound,
    InternalAppError,
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ItemNotFound => write!(f, "item not found"),
            Self::InternalAppError => write!(f, "internal server error"),
        }
    }
}

impl error::Error for App {}
