// src/routes/auth.rs

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    auth::{criar_token, verificar_senha, Claims},
    error::{AppError, Result},
    models::Usuario,
    AppState,
};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub senha: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type:   String,
    pub usuario:      UsuarioOut,
}

#[derive(Serialize)]
pub struct UsuarioOut {
    pub id:    i64,
    pub nome:  String,
    pub email: String,
    pub role:  String,
}

impl From<Usuario> for UsuarioOut {
    fn from(u: Usuario) -> Self {
        Self { id: u.id, nome: u.nome, email: u.email, role: u.role }
    }
}

/// POST /auth/login
/// Recebe email + senha em JSON (não em form-data como o FastAPI)
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<TokenResponse>> {
    let email = payload.email.trim();

    // 1. Busca o usuário pelo e-mail
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT id as \"id!\", nome, email, senha_hash, role, ativo as \"ativo!: bool\"
         FROM usuarios WHERE email = $1 AND ativo = TRUE",
    )
    .bind(email)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // 2. Verifica a senha com argon2
    if !verificar_senha(&payload.senha, &usuario.senha_hash) {
        return Err(AppError::Unauthorized);
    }

    // 3. Cria o JWT
    let token = criar_token(
        usuario.id,
        &usuario.role,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "bearer".into(),
        usuario: usuario.into(),
    }))
}

/// GET /auth/me  — requer Bearer token válido
pub async fn me(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<UsuarioOut>> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT id as \"id!\", nome, email, senha_hash, role, ativo as \"ativo!: bool\"
         FROM usuarios WHERE id = $1",
    )
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Usuário não encontrado".into()))?;

    Ok(Json(usuario.into()))
}
