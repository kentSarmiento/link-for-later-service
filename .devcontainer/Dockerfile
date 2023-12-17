# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# Setup Rust toolchain
ARG RUST_VERSION
RUN rustup install ${RUST_VERSION} \
    && rustup override set ${RUST_VERSION} \
    && rustup component add rust-src rustfmt clippy

# Install cargo-lambda using python: https://www.cargo-lambda.info/guide/installation.html
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends  python3 python3-pip \
    && pip3 install cargo-lambda==1.0.0
