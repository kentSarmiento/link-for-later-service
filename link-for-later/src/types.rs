pub type Result<T> = std::result::Result<T, AppError>;

pub use link_for_later_types::dto::{
    LinkItemRequest, LinkQuery, LinkQueryBuilder, LoginResponse, UserInfoRequest, UserQuery,
    UserQueryBuilder,
};
pub use link_for_later_types::entity::{LinkItem, LinkItemBuilder, UserInfo, UserInfoBuilder};

pub use crate::auth::{Claims, Token};

pub type AppState = crate::app::State;
pub type AppError = crate::app::Error;

#[derive(Debug)]
pub enum Database {
    MongoDb(mongodb::Database),
    InMemory,
}
