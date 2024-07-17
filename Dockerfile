# Rust Official Image
FROM rust:1.77.2

# Work directory in the container.
WORKDIR /api

# Copy file from the project to the containter.
COPY ./ ./

# Install Dependencies
RUN apt-get update
RUN apt-get install -y openssl 
RUN apt-get install pkg-config
RUN apt-get install libudev-dev
RUN apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install --path ./

RUN cargo build --target=x86_64-unknown-linux-musl --release

EXPOSE 9000

CMD [ "/api/target/release/axum-pokedex" ]