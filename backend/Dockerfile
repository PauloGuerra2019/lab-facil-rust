# Stage 1: build
FROM rust:1-slim AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates postgresql postgresql-client \
	pkg-config libssl-dev \
	&& rm -rf /var/lib/apt/lists/*

# Cache das dependências primeiro (Docker layer cache)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release

# Build real
COPY src/ src/
COPY migrations/ migrations/

# SQLx valida as macros durante o build usando um banco descartável local.
RUN service postgresql start \
	&& su postgres -c "psql -c \"CREATE USER sqlx WITH PASSWORD 'sqlx' SUPERUSER;\"" \
	&& su postgres -c "createdb -O sqlx sqlx_build" \
	&& PGPASSWORD=sqlx psql -h 127.0.0.1 -U sqlx -d sqlx_build -f migrations/001_initial.sql \
	&& PGPASSWORD=sqlx psql -h 127.0.0.1 -U sqlx -d sqlx_build -f migrations/002_nfse.sql \
	&& PGPASSWORD=sqlx psql -h 127.0.0.1 -U sqlx -d sqlx_build -f migrations/003_solicitacoes_acesso.sql
ENV DATABASE_URL=postgresql://sqlx:sqlx@127.0.0.1:5432/sqlx_build
RUN touch src/main.rs && cargo build --release

# Stage 2: imagem final ultra-slim (~8 MB)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/migrations/ ./migrations/

EXPOSE 8000
CMD ["./server"]
