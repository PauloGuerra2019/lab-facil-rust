// src/routes/auth.rs

use axum::{extract::State, Json};
use lettre::{
    message::{Mailbox, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
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

fn enviar_email_solicitacao(
    config: &crate::config::Config,
    nome: &str,
    email: &str,
    empresa: Option<&str>,
    telefone: Option<&str>,
    mensagem: Option<&str>,
) -> Result<()> {
    let Some(smtp_host) = config.smtp_host.as_deref() else {
        tracing::warn!(
            "SMTP não configurado; solicitação registrada apenas no banco para {}",
            email
        );
        return Ok(());
    };

    let from = config
        .smtp_from
        .as_deref()
        .unwrap_or(config.aprovacao_email.as_str())
        .parse::<Mailbox>()
        .map_err(|e| AppError::BadRequest(format!("E-mail de origem inválido: {e}")))?;

    let to = config.aprovacao_email.parse::<Mailbox>().map_err(|e| {
        AppError::BadRequest(format!("E-mail de aprovação inválido: {e}"))
    })?;

    let detalhes = [
        format!("Nome: {nome}"),
        format!("E-mail: {email}"),
        empresa.map(|v| format!("Empresa: {v}")).unwrap_or_default(),
        telefone.map(|v| format!("Telefone: {v}")).unwrap_or_default(),
        mensagem.map(|v| format!("Mensagem: {v}")).unwrap_or_default(),
    ]
    .into_iter()
    .filter(|v| !v.is_empty())
    .collect::<Vec<_>>()
    .join("\n");

    let email_body = format!(
        "Nova solicitação de acesso no sistema DADG.\n\n{detalhes}\n"
    );

    let message = Message::builder()
        .from(from)
        .to(to)
        .subject("Nova solicitação de acesso - DADG")
        .singlepart(SinglePart::plain(email_body))
        .map_err(|e| AppError::BadRequest(format!("Falha ao montar e-mail: {e}")))?;

    let creds = Credentials::new(
        config
            .smtp_username
            .clone()
            .unwrap_or_else(|| "".into()),
        config
            .smtp_password
            .clone()
            .unwrap_or_else(|| "".into()),
    );

    let mailer = SmtpTransport::relay(smtp_host)
        .map_err(|e| AppError::BadRequest(format!("SMTP inválido: {e}")))?
        .port(config.smtp_port)
        .credentials(creds)
        .build();

    mailer
        .send(&message)
        .map_err(|e| AppError::BadRequest(format!("Falha ao enviar e-mail: {e}")))?;

    Ok(())
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

    let empresa = payload.empresa.as_deref().map(str::trim).filter(|v| !v.is_empty());
    let telefone = payload.telefone.as_deref().map(str::trim).filter(|v| !v.is_empty());
    let mensagem = payload.mensagem.as_deref().map(str::trim).filter(|v| !v.is_empty());

    // Tenta fazer INSERT; se já existir (constraint), atualiza em vez de bloquear
    sqlx::query(
        "INSERT INTO solicitacoes_acesso (nome, email, empresa, telefone, mensagem, status, criado_em)
         VALUES ($1, $2, $3, $4, $5, 'pendente', NOW())
         ON CONFLICT (email) DO UPDATE SET
           nome = $1,
           empresa = $3,
           telefone = $4,
           mensagem = $5,
           atualizacao_em = NOW()",
    )
    .bind(nome)
    .bind(email)
    .bind(empresa)
    .bind(telefone)
    .bind(mensagem)
    .execute(&state.db)
    .await?;

    if let Err(err) = enviar_email_solicitacao(
        &state.config,
        nome,
        email,
        empresa,
        telefone,
        mensagem,
    ) {
        tracing::warn!("Solicitação salva, mas e-mail de aprovação não pôde ser enviado: {err}");
    }

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
