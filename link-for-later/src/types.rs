pub use self::errors::App as AppError;
pub use self::request::{
    Login as LoginRequest, PostLink as PostLinkRequest, Register as RegisterRequest,
};
pub use self::state::App as AppState;

pub mod entity;
pub mod errors;
pub mod request;
pub mod response;
pub mod state;

pub type Result<T> = std::result::Result<T, AppError>;

pub enum Database {
    MongoDb(mongodb::Database),
    None,
}
