# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# Setup Rust toolchain
ARG RUST_VERSION
RUN rustup install ${RUST_VERSION} \
    && rustup override set ${RUST_VERSION} \
    && rustup component add rust-src rustfmt clippy

# Install Docker
RUN curl -fsSL https://get.docker.com | bash

# Install cargo-lambda using python: https://www.cargo-lambda.info/guide/installation.html
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends  python3 python3-pip \
    && pip3 install cargo-lambda==1.0.0

# Install cargo-shuttle from script fails, install from source instead
# RUN curl -sSfL https://www.shuttle.rs/install | bash
RUN cargo install cargo-shuttle

# Install cargo llvm-cov
RUN rustup component add llvm-tools-preview \
    && curl --proto '=https' --tlsv1.2 -fsSL https://github.com/taiki-e/cargo-llvm-cov/releases/latest/download/cargo-llvm-cov-$(rustc -Vv | grep host | sed 's/host: //').tar.gz | tar xzf - -C "/usr/local/cargo/bin"

# Install rust-covfix as temporary workaround for incorrect line coverage
# https://github.com/Kogia-sima/rust-covfix#how-is-the-incorrect-line-coverage-detected
RUN curl --proto '=https' --tlsv1.2 -fsSL https://github.com/Kogia-sima/rust-covfix/releases/latest/download/rust-covfix-linux-x86_64.tar.xz | tar xJf - -C "/tmp" \
    && cp /tmp/rust-covfix-linux-x86_64/rust-covfix /usr/local/cargo/bin/

# Set ownership of cargo packages to vscode user
RUN sudo chown -R vscode /usr/local/cargo/
