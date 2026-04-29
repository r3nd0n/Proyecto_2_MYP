FROM rust:1.92-trixie AS builder

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libgtk-4-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY data ./data

RUN cargo build --release

FROM debian:trixie-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libgtk-4-1 \
        libglib2.0-0 \
        libgraphene-1.0-0 \
        libcairo2 \
        libpango-1.0-0 \
        libgdk-pixbuf-2.0-0 \
        libx11-6 \
        libxkbcommon0 \
        libwayland-client0 \
        libwayland-cursor0 \
        libwayland-egl1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/code /app/code
COPY data ./data

ENV DB_PATH=/app/data/music.db

CMD ["/app/code"]