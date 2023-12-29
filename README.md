# Link for Later Service

[![development](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml) [![codecov](https://codecov.io/gh/kentSarmiento/link-for-later-service/branch/main/graph/badge.svg)](https://codecov.io/gh/kentSarmiento/link-for-later-service) [![DeepSource](https://app.deepsource.com/gh/kentSarmiento/link-for-later-service.svg/?label=active+issues&token=WjmbW1QTMQOXFFMU5h1-BEmM)](https://app.deepsource.com/gh/kentSarmiento/link-for-later-service/) [![shuttle](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/shuttle.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/shuttle.yml) [![lambda](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/lambda.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/lambda.yml)

Link for Later Service provides an API to save links in your personal library for future reading.

## User Features

- [x] User registration/login for a personal library
- [x] Saving of links to library
- [ ] Analysis of saved links in library
  - [x] estimated time to finish reading
  - [ ] summary
  - [ ] category

## Development Features

- [`Axum`](https://github.com/tokio-rs/axum) as web application framework
- Multiple deployment options:
  - [Shuttle](https://github.com/shuttle-hq/shuttle) application. Refer [here](./link-for-later-shuttle/) for details.
  - [Cargo Lambda](https://www.cargo-lambda.info/) to deploy the service as an AWS Lambda Function. Refer [here](./link-for-later-lambda/) for details.
  - Standalone server using axum for local development. Refer [here](./link-for-later/src/bin/) for details.
- Multiple repository options:
  - MongoDB
  - InMemory database
  - and more coming soon...
- Route authorization using [`jsonwebtoken`](https://github.com/Keats/jsonwebtoken)
- Password hashing using [`argon2`](https://github.com/RustCrypto/password-hashes/tree/master/argon2)
- Mock objects for testing using [`mockall`](https://github.com/asomers/mockall)
- Fixture-based test framework using [`rstest`](https://github.com/la10736/rstest)

## Development Tooling

- [Devcontainer](https://code.visualstudio.com/docs/devcontainers/containers) for development in VSCode
- [Github Actions](https://github.com/dependabot) for CI/CD
- [Github Dependabot](https://github.com/actions) for regular dependency updates
- [Clippy](https://github.com/rust-lang/rust-clippy) for linting/static analysis
- [Codecov](https://about.codecov.io/) for coverage metrics
- [DeepSource](https://deepsource.com/) for static analysis/coverage metrics management
