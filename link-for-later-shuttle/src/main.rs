use mongodb::Database;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::MongoDb] db: Database) -> shuttle_axum::ShuttleAxum {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();

    let app = link_for_later::app::new(link_for_later::DatabaseType::MongoDb(db));
    Ok(app.into())
}
