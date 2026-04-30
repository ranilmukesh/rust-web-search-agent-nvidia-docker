FROM rust:latest AS builder
WORKDIR /app

RUN rustup update

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM rust:latest

COPY --from=builder /app/target/release/rig-ddg-agent /usr/local/bin/rig-ddg-agent

ENTRYPOINT ["rig-ddg-agent"]
