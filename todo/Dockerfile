FROM lukemathwalker/cargo-chef:0.1.51-rust-1-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin todo

FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/todo /usr/local/bin
ENTRYPOINT ["/usr/local/bin/todo"]
