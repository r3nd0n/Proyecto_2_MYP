FROM rust:1.87-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libgtk-4-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/code
COPY code /app/code
RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libgtk-4-1 \
        libglib2.0-0 \
        libpango-1.0-0 \
        libgdk-pixbuf-2.0-0 \
        libgraphene-1.0-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/code/target/release/code /usr/local/bin/proyecto2

ENV MUSIC_SCAN_DIR=/home/ahalgana
ENV DB_PATH=/data/music.db

VOLUME ["/music", "/data"]

CMD ["/usr/local/bin/proyecto2"]
