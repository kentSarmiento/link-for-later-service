pub use self::errors::App as AppError;

pub type Result<T> = std::result::Result<T, AppError>;

pub enum Database {
    MongoDb(mongodb::Database),
    None,
}

pub mod auth;
pub mod dto;
pub mod entity;
pub mod errors;
