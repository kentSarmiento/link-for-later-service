# Link for Later Service as a Standalone axum server

## Development

The standalone server is using a MongoDb repository.

Set the MongoDB server and database name. Then use `cargo run` to run the Server.

```sh
export MONGODB_URI="mongodb://localhost:23288"
export MONGODB_DATABASE_NAME="test"
cargo run --bin link-for-later-axum
```

You will be able to send requests to the server using port 8080.
