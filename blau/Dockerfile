FROM rust as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/blau /usr/local/bin/blau

ENTRYPOINT ["/usr/local/bin/blau"]
