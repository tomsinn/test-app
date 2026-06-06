FROM ubuntu:latest
LABEL authors="tommy"

ENTRYPOINT ["top", "-b"]

FROM rust:latest AS builder
WORKDIR /app
COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN mkdir -p text-files
COPY --from=builder /app/target/release/test-app /app/app
CMD ["./app"]