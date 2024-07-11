# Rust Official Image
FROM rust:1.77.2

# Work directory in the container.
WORKDIR /api

# Copy file from the project to the containter.
COPY ./ ./

# Set the architecture argument (arm64, i.e. aarch64 as default)
ARG ARCH=x86_64

# Install Dependencies
RUN apt-get update && \
    apt-get install openssl -y && \
    apt-get install pkg-config && \
    apt-get install libudev-dev && \
    apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install --path ./

RUN cargo build --target=x86_64-unknown-linux-musl --release

EXPOSE 9000

CMD [ "/api/target/release/axum-pokedex" ]