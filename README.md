# Link for Later Service

[![development](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml) [![release](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/release.yml)

Link for Later Service using Rust

## Development

### Required Downloads:

- Docker
- Visual Studio Code
- [Devcontainer extension for vscode](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Setup:

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

### Running the Service:

The project is structured into a separate [library](./link-for-later/) and multiple binaries depending on the option to run the service.

Running as [Lambda](./link-for-later-lambda/src/main.rs):

1. [`cargo lambda`](https://www.cargo-lambda.info/) can be used for development of this service and it is pre-installed as part of the devcontainer. Use [`cargo lambda watch`](https://www.cargo-lambda.info/commands/watch.html) to hotcompile your changes:

   ```sh
   cargo lambda watch
   ```

Running as [Axum Server](./link-for-later-axum/src/main.rs)

1. Use `cargo run`:

   ```sh
   cargo run --bin link-for-later-axum
   ```

### Testing:

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
