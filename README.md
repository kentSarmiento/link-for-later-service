# Link for Later Service

[![development](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/development.yml) [![codecov](https://codecov.io/gh/kentSarmiento/link-for-later-service/branch/main/graph/badge.svg)](https://codecov.io/gh/kentSarmiento/link-for-later-service) [![DeepSource](https://app.deepsource.com/gh/kentSarmiento/link-for-later-service.svg/?label=active+issues&token=WjmbW1QTMQOXFFMU5h1-BEmM)](https://app.deepsource.com/gh/kentSarmiento/link-for-later-service/) [![shuttle](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/shuttle.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/shuttle.yml) [![lambda](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/lambda.yml/badge.svg?branch=main)](https://github.com/kentSarmiento/link-for-later-service/actions/workflows/lambda.yml)

Link for Later Service provides an API to save links in your personal library for future reading.

## User Features

- [x] User registration/login for a personal library
- [x] Saving of links to library
- [ ] Analysis of saved links in library (to add information such as category, summary, estimated time to finish reading)

## Development Features

- axum...
- shuttle...
- devcontainer
- 

## Project Structure/Deployments

A workspace...
The project is structured into the [Service Library](./link-for-later/) and multiple binaries depending on the option to run/deploy the service

- [Shuttle](./link-for-later-shuttle/)
- [AWS Lambda Function](./link-for-later-lambda/)
- [Standalone server using axum](./link-for-later-axum/) (for local development)
