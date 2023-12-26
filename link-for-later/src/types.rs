use crate::app;

pub type Result<T> = std::result::Result<T, AppError>;

pub type AppState = app::State;
pub type AppError = app::Error;

pub enum Database {
    MongoDb(mongodb::Database),
    InMemory,
}
