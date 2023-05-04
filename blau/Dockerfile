FROM rust:1-alpine as builder

RUN apk add --no-cache musl-dev build-base protobuf-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine

COPY --from=builder /app/target/release/blau /usr/local/bin/blau

ENTRYPOINT ["/usr/local/bin/blau"]
