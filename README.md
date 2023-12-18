# Link for Later Service

[![development](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml) [![release](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/release.yml)

Link for Later Lambda Service using Rust

## Development

Required Downloads:

- Docker
- Visual Studio Code
- [Devcontainer extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

Setup:

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

Testing:

1. [cargo lambda](https://www.cargo-lambda.info/) is used for development of this service and it is pre-installed as part of the devcontainer. Use [cargo lambda watch](https://www.cargo-lambda.info/commands/watch.html) to hotcompile your changes:

   ```sh
   cargo lambda watch
   ```

1. [cargo clippy](https://github.com/rust-lang/rust-clippy) is used for linting to catch common errors. This is setup to run on saving changes in the devcontainer. You may also run it from bash using the following command:

   ```sh
   cargo clippy --all-targets --all-features -- -D warnings
   ```
