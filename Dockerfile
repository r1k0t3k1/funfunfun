FROM rust:1.97.1-slim-trixie AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY ./f3-server/* .
RUN apt update && apt install -y libssl-dev pkg-config wget curl libnss3-tools
RUN cargo build --release

FROM debian:trixie-slim
WORKDIR /app

RUN apt update && apt install -y curl
RUN useradd --create-home --shell /bin/bash builder && chown -R builder /app
USER builder

COPY --from=builder --chown=builder:builder /app/certificate ./certificate
COPY --from=builder /app/target/release/f3-server ./target/release/f3-server

ENV PORT 443
EXPOSE $PORT
ENTRYPOINT ["./target/release/f3-server"]
