// src/routes/auth.rs

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    auth::{criar_token, verificar_senha, Claims},
    error::{AppError, Result},
    models::{SolicitacaoAcessoPayload, Usuario},
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
pub struct SolicitacaoAcessoResponse {
    pub sucesso:  bool,
    pub mensagem: String,
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
        "SELECT id, nome, email, senha_hash, role, ativo
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

/// POST /auth/solicitar-acesso
/// Registra uma solicitação de cadastro e aguarda aprovação manual.
pub async fn solicitar_acesso(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SolicitacaoAcessoPayload>,
) -> Result<Json<SolicitacaoAcessoResponse>> {
    let nome = payload.nome.trim();
    let email = payload.email.trim();

    if nome.is_empty() || email.is_empty() {
        return Err(AppError::BadRequest("Nome e e-mail são obrigatórios.".into()));
    }

    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::BadRequest("Informe um e-mail válido.".into()));
    }

    let existente = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM solicitacoes_acesso WHERE email = $1 AND status = 'pendente')",
    )
    .bind(email)
    .fetch_one(&state.db)
    .await?;

    if existente {
        return Err(AppError::Conflict("Já existe uma solicitação pendente para este e-mail.".into()));
    }

    let empresa = payload.empresa.as_deref().map(str::trim).filter(|v| !v.is_empty());
    let telefone = payload.telefone.as_deref().map(str::trim).filter(|v| !v.is_empty());
    let mensagem = payload.mensagem.as_deref().map(str::trim).filter(|v| !v.is_empty());

    sqlx::query(
        "INSERT INTO solicitacoes_acesso (nome, email, empresa, telefone, mensagem, status, criado_em)
         VALUES ($1, $2, $3, $4, $5, 'pendente', NOW())",
    )
    .bind(nome)
    .bind(email)
    .bind(empresa)
    .bind(telefone)
    .bind(mensagem)
    .execute(&state.db)
    .await?;

    tracing::info!(
        "Nova solicitação de acesso recebida para {} (aprovação por: {})",
        email,
        state.config.aprovacao_email
    );

    Ok(Json(SolicitacaoAcessoResponse {
        sucesso: true,
        mensagem: "Solicitação enviada com sucesso. A aprovação será realizada por e-mail.".into(),
    }))
}

/// GET /auth/me  — requer Bearer token válido
pub async fn me(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<UsuarioOut>> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT id, nome, email, senha_hash, role, ativo
         FROM usuarios WHERE id = $1",
    )
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Usuário não encontrado".into()))?;

    Ok(Json(usuario.into()))
}
