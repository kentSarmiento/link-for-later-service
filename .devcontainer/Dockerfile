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

# Install cargo-shuttle from script fails, install from source instead
# RUN curl -sSfL https://www.shuttle.rs/install | bash
RUN cargo install cargo-shuttle
RUN sudo chown -R vscode /usr/local/cargo/

# Install Docker
RUN apt-get update \
    && apt-get install -y apt-transport-https ca-certificates curl gnupg2 lsb-release \
    && curl -fsSL https://download.docker.com/linux/$(lsb_release -is | tr '[:upper:]' '[:lower:]')/gpg | apt-key add - 2>/dev/null \
    && echo "deb [arch=$(dpkg --print-architecture)] https://download.docker.com/linux/$(lsb_release -is | tr '[:upper:]' '[:lower:]') $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list \
    && apt-get update \
    && apt-get install -y docker-ce-cli
