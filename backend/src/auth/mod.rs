// src/auth/mod.rs

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::{AppError, Result}, AppState};

// ── Claims JWT ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:  i64,       // user id
    pub role: String,
    pub exp:  usize,     // unix timestamp de expiração
}

pub fn criar_token(user_id: i64, role: &str, secret: &str, horas: u64) -> Result<String> {
    let expiracao = (Utc::now() + Duration::hours(horas as i64)).timestamp() as usize;
    let claims = Claims { sub: user_id, role: role.to_string(), exp: expiracao };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Anyhow(e.into()))
}

pub fn verificar_token(token: &str, secret: &str) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}

// ── Extractor: injeta Claims diretamente nos handlers ─────────────────────────
//
// Uso: `async fn meu_handler(claims: Claims, ...) { ... }`
// Axum chama from_request_parts automaticamente.

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync + AsRef<AppState>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let app_state: &AppState = state.as_ref();
        let token = extrair_bearer(&parts.headers).ok_or(AppError::Unauthorized)?;
        verificar_token(token, &app_state.config.jwt_secret)
    }
}

fn extrair_bearer(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}

// ── Senha ─────────────────────────────────────────────────────────────────────

/// Gera hash Argon2id da senha. Argon2id é o padrão recomendado pela OWASP
/// e mais resistente a ataques de GPU do que bcrypt.
pub fn hash_senha(senha: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(senha.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Anyhow(anyhow::anyhow!("{e}")))
}

pub fn verificar_senha(senha: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .ok()
        .map(|h| Argon2::default().verify_password(senha.as_bytes(), &h).is_ok())
        .unwrap_or(false)
}
