# Link for Later Service as a Standalone server using axum

## Development

For local development, a standalone server using axum can be used with either an In-memory database or a MongoDb repository.

* To use In-memory database, set `INMEMORY_DB` before running the server

    ```sh
    INMEMORY_DB=true cargo run --bin link-for-later
    ```

* To use MongoDb, set the MongoDB server and database name before running the server

    ```sh
    MONGODB_URI="mongodb://localhost:27017" MONGODB_DATABASE_NAME="test" cargo run --bin link-for-later
    ```

You will be able to send requests to the server using port 8080.
