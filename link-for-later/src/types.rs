pub type Result<T> = std::result::Result<T, AppError>;

pub type AppState = crate::app::State;
pub type AppError = crate::app::Error;

#[derive(Debug)]
pub enum Database {
    MongoDb(mongodb::Database),
    InMemory,
}
