# Link for Later Service

[![development](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml) [![release](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/release.yml)

Link for Later Service using Rust

## Deployments

### AWS Lambda

Soon!

### Shuttle

🚀 https://link-for-later.shuttleapp.rs/v1/links

## Development

### Required Downloads

- Docker
- Visual Studio Code
- [Devcontainer extension for vscode](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Setup

1. Clone repository

   ```sh
   git@github.com:kentSarmiento/link-for-later-service.git
   ```

1. Open Visual Studio Code within the repo

   ```sh
   cd link-for-later-service
   code .
   ```

1. Wait for notification in the bottom right corner asking to `"Reopen in container"`.

### Running the Service

The project is structured into a separate [library](./link-for-later/) and multiple binaries depending on the option to run the service.

Running as [Lambda](./link-for-later-lambda/src/main.rs):

1. [`cargo lambda`](https://www.cargo-lambda.info/) is one of the options for development of this service and it is pre-installed as part of the devcontainer. Use [`cargo lambda watch`](https://www.cargo-lambda.info/commands/watch.html) to run the app as lambda:

   ```sh
   cargo lambda watch
   ```

   You will be able to send requests to the lambda using port 9000.

Running as [Shuttle](./link-for-later-shuttle/src/main.rs)

1. [Shuttle](https://github.com/shuttle-hq/shuttle) is also one of the options for development of this service and it is pre-installed as part of the devcontainer. Use [`cargo shuttle run`](https://docs.shuttle.rs/getting-started/local-run) to run the app locally:

   ```sh
   cargo shuttle run
   ```

   You will be able to send requests to the server using port 8000.

Running as standalone [Axum Server](./link-for-later-axum/src/main.rs)

1. Use `cargo run`:

   ```sh
   cargo run --bin link-for-later-axum
   ```

   You will be able to send requests to the server using port 3000.

### Testing

Aside from sending HTTP requests to a running server, there are also several other tests that can be done locally:

1. `cargo test` is used to run unit/integration tests

   Unit Test:

   ```sh
   cargo test --lib
   ```

   Integration Test:

   ```sh
   cargo test --test '*'
   ```

1. [`cargo clippy`](https://github.com/rust-lang/rust-clippy) is used for linting to catch common errors. This is setup to run on saving changes in the devcontainer. You may also run it from bash using the following command:

   ```sh
   cargo clippy --all-targets --all-features -- -D warnings
   ```
