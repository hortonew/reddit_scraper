# Use the official Rust image as a base
FROM rust:1.79 AS builder
ARG APP_NAME

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY services/ services/
COPY models/ models/
COPY databases/ databases/

RUN cargo build --release -p ${APP_NAME}

# Use a minimal base image for the build
FROM debian:bookworm-slim
ARG APP_NAME
ENV APP_NAME=${APP_NAME}
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    libssl-dev \
    curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}
COPY --from=builder /usr/src/app/databases /usr/src/app/databases

RUN chmod +x /usr/local/bin/${APP_NAME} && chmod 777 /usr/src/app
CMD ["/bin/bash", "-c", "/usr/local/bin/${APP_NAME}"]
