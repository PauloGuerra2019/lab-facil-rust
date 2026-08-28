// src/routes/dashboard.rs

use axum::{extract::State, Json};
use chrono::Local;
use serde::Serialize;
use std::sync::Arc;

use crate::{auth::Claims, error::Result, AppState};

#[derive(Serialize)]
pub struct DashboardStats {
    pub total_os_abertas:             i64,
    pub total_os_atrasadas:           i64,
    pub faturamento_mes:              f64,
    pub a_receber:                    f64,
    pub os_por_status:                serde_json::Value,
    pub faturamento_ultimos_6_meses:  Vec<MesFaturamento>,
}

#[derive(Serialize)]
pub struct MesFaturamento {
    pub mes:   String,
    pub total: f64,
}

pub async fn estatisticas(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<Json<DashboardStats>> {
    let hoje = Local::now().format("%Y-%m-%d").to_string();
    let inicio_mes = Local::now().format("%Y-%m-01").to_string();

    // OS abertas e atrasadas
    let row = sqlx::query!(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE status NOT IN ('entregue','cancelado')) AS "abertas!: i64",
            COUNT(*) FILTER (WHERE status NOT IN ('entregue','cancelado')
                             AND data_prevista < $1)       AS "atrasadas!: i64"
        FROM ordens_servico
        "#,
        hoje
    )
    .fetch_one(&state.db)
    .await?;

    // Faturamento do mês (soma dos itens das OS do mês)
    let fat_mes = sqlx::query!(
        r#"
        SELECT COALESCE(SUM(i.quantidade * i.valor_unitario), 0) AS "total!: f64"
        FROM itens_os i
        JOIN ordens_servico os ON os.id = i.ordem_id
        WHERE os.data_entrada >= $1 AND os.status != 'cancelado'
        "#,
        inicio_mes
    )
    .fetch_one(&state.db)
    .await?
    .total;

    // A receber (valor_total - valor_pago de OS não pagas)
    let a_receber = sqlx::query!(
        r#"
        SELECT COALESCE(
            SUM(i.quantidade * i.valor_unitario) - SUM(os.valor_pago), 0
        ) AS "total!: f64"
        FROM itens_os i
        JOIN ordens_servico os ON os.id = i.ordem_id
        WHERE os.status_pagamento != 'pago' AND os.status != 'cancelado'
        "#
    )
    .fetch_one(&state.db)
    .await?
    .total;

    // OS por status
    let por_status = sqlx::query!(
        r#"SELECT status, COUNT(*) AS "n!: i64" FROM ordens_servico
           WHERE status != 'cancelado' GROUP BY status"#
    )
    .fetch_all(&state.db)
    .await?;

    let mut status_map = serde_json::Map::new();
    for row in por_status {
        status_map.insert(row.status, serde_json::json!(row.n));
    }

    // Faturamento dos últimos 6 meses
    let mut fat_meses = Vec::new();
    for i in (0i64..6).rev() {
        let mes_ref = Local::now()
            .date_naive()
            .checked_sub_months(chrono::Months::new(i as u32))
            .unwrap();
        let inicio = mes_ref.with_day(1).unwrap().format("%Y-%m-%d").to_string();
        let proximo = mes_ref
            .checked_add_months(chrono::Months::new(1))
            .unwrap()
            .with_day(1)
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();

        let total = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(i.quantidade * i.valor_unitario), 0) AS "total!: f64"
            FROM itens_os i
            JOIN ordens_servico os ON os.id = i.ordem_id
            WHERE os.data_entrada >= $1 AND os.data_entrada < $2 AND os.status != 'cancelado'
            "#,
            inicio, proximo
        )
        .fetch_one(&state.db)
        .await?
        .total;

        fat_meses.push(MesFaturamento {
            mes: mes_ref.format("%Y-%m").to_string(),
            total,
        });
    }

    Ok(Json(DashboardStats {
        total_os_abertas: row.abertas,
        total_os_atrasadas: row.atrasadas,
        faturamento_mes: fat_mes,
        a_receber,
        os_por_status: serde_json::Value::Object(status_map),
        faturamento_ultimos_6_meses: fat_meses,
    }))
}

// Workaround: chrono::NaiveDate::with_day — importar trait necessária
use chrono::Datelike;
