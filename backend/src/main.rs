// src/main.rs
//
// Lab Fácil — Backend em Rust (Axum + SQLx + JWT/Argon2)
//
// Comandos rápidos:
//   cargo run                 → dev (sem otimização)
//   cargo run --release       → produção
//   DATABASE_URL=<url-postgres> cargo sqlx migrate run       → aplica migrations manualmente

mod auth;
mod config;
mod error;
mod models;
mod routes;

use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, PgPool};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;

// ── Estado compartilhado entre todos os handlers ──────────────────────────────

#[derive(Clone)]
pub struct AppState {
    pub db:     PgPool,
    pub config: Config,
}

// ── Entrypoint ────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Carrega variáveis de ambiente do arquivo .env (se existir)
    dotenvy::dotenv().ok();

    // Log estruturado via tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "lab_facil=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    // Pool PostgreSQL — o Supabase gerencia persistência e concorrência
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(
            config.database_url.parse::<PgConnectOptions>()?
                .statement_cache_capacity(0),
        )
        .await?;

    // Roda as migrations automaticamente no startup
    sqlx::migrate!("./migrations").run(&db).await?;

    // Seed do admin (só executa se o usuário não existir)
    seed_admin(&db, &config).await?;

    let state = Arc::new(AppState { db, config: config.clone() });

    // CORS: permite o frontend React e acessos na rede local (celular / IP local)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Health check
        .route("/", get(|| async { r#"{"status":"ok","servico":"Lab Fácil API"}"# }))

        // Auth
        .route("/auth/login", post(routes::auth::login))
        .route("/auth/me",    get(routes::auth::me))
        .route("/auth/solicitar-acesso", post(routes::auth::solicitar_acesso))

        // Clientes
        .route("/clientes",     get(routes::clientes::listar).post(routes::clientes::criar))
        .route("/clientes/:id", get(routes::clientes::obter)
                                .put(routes::clientes::atualizar)
                                .delete(routes::clientes::desativar))

        // Tipos de Serviço
        .route("/tipos-servico",     get(routes::tipos_servico::listar).post(routes::tipos_servico::criar))
        .route("/tipos-servico/:id", put(routes::tipos_servico::atualizar)
                                     .delete(routes::tipos_servico::desativar))

        // Ordens de Serviço
        .route("/ordens-servico",
            get(routes::ordens_servico::listar).post(routes::ordens_servico::criar))
        .route("/ordens-servico/:id",
            get(routes::ordens_servico::obter)
            .put(routes::ordens_servico::atualizar)
            .delete(routes::ordens_servico::cancelar))
        .route("/ordens-servico/:id/nfse", post(routes::ordens_servico::emitir_nfse))
        .route("/ordens-servico/:id/recibo", get(routes::ordens_servico::gerar_recibo))

        // Dashboard
        .route("/dashboard", get(routes::dashboard::estatisticas))

        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".into());
    let addr = format!("0.0.0.0:{port}");
    tracing::info!("Lab Fácil API rodando em http://{addr}");
    tracing::info!("Documentação: a API não tem Swagger built-in no Axum — use Bruno/Insomnia/httpie");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ── Seed ──────────────────────────────────────────────────────────────────────

async fn seed_admin(db: &PgPool, _config: &Config) -> anyhow::Result<()> {
    let hash = auth::hash_senha("admin123")?;

    let alterado = sqlx::query(
        "UPDATE usuarios SET senha_hash = $1 WHERE email = 'admin@laboratorio.com' AND (senha_hash = '$placeholder$' OR senha_hash LIKE '%placeholder%')",
    )
    .bind(hash.clone())
    .execute(db)
    .await?
    .rows_affected();

    let existe = sqlx::query_scalar::<_, i64>("SELECT COUNT(*)::BIGINT FROM usuarios WHERE email = 'admin@laboratorio.com'")
        .fetch_one(db)
        .await?;

    if existe == 0 {
              sqlx::query(
            "INSERT INTO usuarios (nome, email, senha_hash, role)
             VALUES ('Administrador', 'admin@laboratorio.com', $1, 'admin')",
        )
          .bind(hash)
        .execute(db)
        .await?;

        tracing::info!("Admin criado: admin@laboratorio.com / admin123");
    } else if alterado > 0 {
        tracing::info!("Senha de admin atualizada com sucesso para admin123");
    }

    Ok(())
}
