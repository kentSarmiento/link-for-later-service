pub use self::errors::App as AppError;
pub use self::state::App as AppState;
pub use self::{repository::DynLinks as DynLinksRepo, service::DynLinks as DynLinksService};

pub mod errors;
pub mod links;
pub mod repository;
pub mod request;
pub mod response;
pub mod service;
pub mod state;

pub type Result<T> = std::result::Result<T, AppError>;
