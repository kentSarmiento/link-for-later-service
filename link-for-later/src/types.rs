pub use self::errors::App as AppError;
pub use self::request::{
    Login as LoginRequest, PostLink as PostLinkRequest, Register as RegisterRequest,
};
pub use self::response::Login as LoginResponse;

pub mod auth;
pub mod entity;
pub mod errors;
pub mod request;
pub mod response;

pub type Result<T> = std::result::Result<T, AppError>;

pub enum Database {
    MongoDb(mongodb::Database),
    None,
}
