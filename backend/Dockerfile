# Stage 1: build
FROM rust:1-slim AS builder
WORKDIR /app

# SQLx precisa da URL para validar as queries durante a compilação.
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Cache das dependências primeiro (Docker layer cache)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release

# Build real
COPY src/ src/
COPY migrations/ migrations/
RUN touch src/main.rs && cargo build --release

# Stage 2: imagem final ultra-slim (~8 MB)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/migrations/ ./migrations/

EXPOSE 8000
CMD ["./server"]
