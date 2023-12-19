# Link for Later Service

[![development](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml) [![shuttle](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/shuttle.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/shuttle.yml) [![lambda](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/lambda.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/lambda.yml)

Link for Later Service provides an API to save links in your personal library for future reading.

## Features

- User login/registration for personal library
- Saving of links
- Analysis of saved links (to add information such as label, category, description, estimated time)

## Deployments

The project is structured into the [Service Library](./link-for-later/) and multiple binaries depending on the option to run/deploy the service

- [Standalone axum server](./link-for-later-axum/) (for local development)
- [Shuttle](./link-for-later-shuttle/)
- [AWS Lambda Function](./link-for-later-lambda/)

## Getting Started

### Required Downloads

- Docker
- Visual Studio Code
- [Devcontainer extension for vscode](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Setup

1. Clone this repository.
1. Open Visual Studio Code within the repo.
1. Wait for the notification in the bottom right corner asking to `"Reopen in container"`.
