use mongodb::Database;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::MongoDb] db: Database) -> shuttle_axum::ShuttleAxum {
    let app = link_for_later::app::new(link_for_later::DatabaseType::MongoDb(db));
    Ok(app.into())
}
