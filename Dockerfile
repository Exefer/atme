FROM oven/bun:canary-alpine AS frontend-builder
WORKDIR /usr/src/app

COPY ./frontend/. .
RUN bun install --frozen-lockfile
ENV NODE_ENV=production
RUN bun run --bun build

FROM rust:alpine AS builder
WORKDIR /usr/src/app

RUN apk update && apk add --no-cache \
  build-base \
  musl-dev \
  openssl \
  openssl-dev \
  pkgconf \
  ca-certificates

COPY ./backend/. .
COPY --from=frontend-builder /usr/src/app/build ./static
RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/atme /usr/local/bin/atme

EXPOSE 3000

CMD ["atme"]