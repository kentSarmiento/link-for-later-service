pub type Result<T> = std::result::Result<T, AppError>;

pub type AppState = crate::app::State;
pub type AppError = crate::app::Error;

pub enum Database {
    MongoDb(mongodb::Database),
    InMemory,
}
