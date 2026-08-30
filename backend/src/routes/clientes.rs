// src/routes/clientes.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{
    auth::Claims,
    error::{AppError, Result},
    models::{Cliente, ClienteFiltro, ClientePayload},
    AppState,
};

fn limpar_opt(s: Option<String>) -> Option<String> {
    s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

/// GET /clientes
pub async fn listar(
    State(state): State<Arc<AppState>>,
    Query(filtro): Query<ClienteFiltro>,
    _claims: Claims,
) -> Result<Json<Vec<Cliente>>> {
    let apenas_ativos_num: i64 = if filtro.apenas_ativos.unwrap_or(true) { 1 } else { 0 };
    let busca = filtro.busca.as_deref().map(|b| format!("%{b}%"));

    let clientes = sqlx::query_as::<_, Cliente>(
        r#"
        SELECT id as "id!", nome, cpf_cnpj, telefone, email, endereco, observacoes,
               ativo as "ativo!: bool"
        FROM clientes
        WHERE ($1 = 0 OR ativo = TRUE)
          AND ($2 IS NULL OR nome LIKE $3 OR cpf_cnpj LIKE $4)
        ORDER BY nome
        "#,
    )
    .bind(apenas_ativos_num)
    .bind(&busca)
    .bind(&busca)
    .bind(&busca)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(clientes))
}

/// POST /clientes
pub async fn criar(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Json(payload): Json<ClientePayload>,
) -> Result<(StatusCode, Json<Cliente>)> {
    let nome = payload.nome.trim().to_string();
    if nome.is_empty() {
        return Err(AppError::BadRequest("O nome do cliente é obrigatório.".into()));
    }

    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO clientes (nome, cpf_cnpj, telefone, email, endereco, observacoes)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
    )
    .bind(nome)
    .bind(limpar_opt(payload.cpf_cnpj))
    .bind(limpar_opt(payload.telefone))
    .bind(limpar_opt(payload.email))
    .bind(limpar_opt(payload.endereco))
    .bind(limpar_opt(payload.observacoes))
    .fetch_one(&state.db)
    .await?;

    let cliente = buscar_por_id(&state, id).await?;
    Ok((StatusCode::CREATED, Json(cliente)))
}

/// GET /clientes/:id
pub async fn obter(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<Json<Cliente>> {
    let cliente = buscar_por_id(&state, id).await?;
    Ok(Json(cliente))
}

/// PUT /clientes/:id
pub async fn atualizar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
    Json(payload): Json<ClientePayload>,
) -> Result<Json<Cliente>> {
    let nome = payload.nome.trim().to_string();
    if nome.is_empty() {
        return Err(AppError::BadRequest("O nome do cliente é obrigatório.".into()));
    }

    sqlx::query(
        "UPDATE clientes SET nome=$1, cpf_cnpj=$2, telefone=$3, email=$4, endereco=$5, observacoes=$6
         WHERE id=$7",
    )
    .bind(nome)
    .bind(limpar_opt(payload.cpf_cnpj))
    .bind(limpar_opt(payload.telefone))
    .bind(limpar_opt(payload.email))
    .bind(limpar_opt(payload.endereco))
    .bind(limpar_opt(payload.observacoes))
    .bind(id)
    .execute(&state.db)
    .await?;

    let cliente = buscar_por_id(&state, id).await?;
    Ok(Json(cliente))
}

/// DELETE /clientes/:id  (soft delete — mantém histórico de OS)
pub async fn desativar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<StatusCode> {
    let linhas = sqlx::query("UPDATE clientes SET ativo=FALSE WHERE id=$1")
        .bind(id)
        .execute(&state.db)
        .await?
        .rows_affected();

    if linhas == 0 {
        return Err(AppError::NotFound(format!("Cliente {id} não encontrado")));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ── helpers ───────────────────────────────────────────────────────────────────

async fn buscar_por_id(state: &AppState, id: i64) -> Result<Cliente> {
    sqlx::query_as::<_, Cliente>(
        r#"SELECT id as "id!", nome, cpf_cnpj, telefone, email, endereco, observacoes,
                  ativo as "ativo!: bool"
           FROM clientes WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Cliente {id} não encontrado")))
}
