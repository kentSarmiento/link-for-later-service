use mongodb::{options::ClientOptions, Client};

const INMEMORY_DB_KEY: &str = "INMEMORY_DB";
const MONGODB_URI_KEY: &str = "MONGODB_URI";
const MONGODB_DATABASE_NAME_KEY: &str = "MONGODB_DATABASE_NAME";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let app = if std::env::var(INMEMORY_DB_KEY).is_ok() {
        tracing::info!("Using in-memory database");
        link_for_later::app::new(link_for_later::DatabaseType::InMemory)
    } else {
        tracing::info!("Using mongodb database");

        let uri = std::env::var(MONGODB_URI_KEY)?;
        let database_name = std::env::var(MONGODB_DATABASE_NAME_KEY)?;

        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&database_name);

        link_for_later::app::new(link_for_later::DatabaseType::MongoDb(db))
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
