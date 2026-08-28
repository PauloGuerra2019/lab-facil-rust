// src/routes/tipos_servico.rs

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{
    auth::Claims,
    error::{AppError, Result},
    models::{TipoServico, TipoServicoPayload},
    AppState,
};

pub async fn listar(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<Json<Vec<TipoServico>>> {
    let tipos = sqlx::query_as!(
        TipoServico,
        r#"SELECT id as "id!", nome, categoria, valor_padrao as "valor_padrao!", prazo_dias as "prazo_dias!", ativo as "ativo!: bool"
           FROM tipos_servico WHERE ativo = TRUE ORDER BY categoria, nome"#
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(tipos))
}

pub async fn criar(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Json(p): Json<TipoServicoPayload>,
) -> Result<(StatusCode, Json<TipoServico>)> {
    let id = sqlx::query_as::<_, (i64,)>(
        "INSERT INTO tipos_servico (nome, categoria, valor_padrao, prazo_dias) VALUES ($1,$2,$3,$4) RETURNING id",
    )
    .bind(p.nome)
    .bind(p.categoria)
    .bind(p.valor_padrao)
    .bind(p.prazo_dias as i32)
    .fetch_one(&state.db)
    .await?
    .0;

    let tipo = buscar_tipo(&state, id).await?;
    Ok((StatusCode::CREATED, Json(tipo)))
}

pub async fn atualizar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
    Json(p): Json<TipoServicoPayload>,
) -> Result<Json<TipoServico>> {
    sqlx::query!(
        "UPDATE tipos_servico SET nome=$1, categoria=$2, valor_padrao=$3, prazo_dias=$4 WHERE id=$5",
        p.nome, p.categoria, p.valor_padrao, p.prazo_dias as i32, id
    )
    .execute(&state.db)
    .await?;
    Ok(Json(buscar_tipo(&state, id).await?))
}

pub async fn desativar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<StatusCode> {
    sqlx::query!("UPDATE tipos_servico SET ativo=FALSE WHERE id=$1", id)
        .execute(&state.db)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn buscar_tipo(state: &AppState, id: i64) -> Result<TipoServico> {
    sqlx::query_as!(
        TipoServico,
        r#"SELECT id as "id!", nome, categoria, valor_padrao as "valor_padrao!", prazo_dias as "prazo_dias!", ativo as "ativo!: bool"
           FROM tipos_servico WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Serviço {id} não encontrado")))
}
