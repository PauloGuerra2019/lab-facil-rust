// src/error.rs
//
// Um único enum cobre todos os erros da aplicação.
// axum::IntoResponse converte automaticamente em JSON com o status HTTP certo.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Não autenticado")]
    Unauthorized,

    #[error("Sem permissão")]
    Forbidden,

    #[error("Não encontrado: {0}")]
    NotFound(String),

    #[error("Dados inválidos: {0}")]
    BadRequest(String),

    #[error("Conflito: {0}")]
    Conflict(String),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Unauthorized            => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden               => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::NotFound(msg)           => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg)         => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg)           => (StatusCode::CONFLICT, msg.clone()),
            AppError::Sqlx(e)                 => {
                tracing::error!("SQLx error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro de banco de dados".into())
            }
            AppError::Anyhow(e)               => {
                tracing::error!("Internal error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno".into())
            }
        };

        (status, Json(json!({ "detail": message }))).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
