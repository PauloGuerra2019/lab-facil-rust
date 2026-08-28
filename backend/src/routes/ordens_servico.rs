// src/routes/ordens_servico.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Local;
use std::sync::Arc;

use crate::{
    auth::Claims,
    error::{AppError, Result},
    models::{
        Cliente, ItemOS, ItemOSOut, ItemOSPayload, OrdemFiltro, OrdemServico, OrdemServicoCreate,
        OrdemServicoOut, OrdemServicoUpdate, TipoServico,
    },
    AppState,
};

/// GET /ordens-servico
pub async fn listar(
    State(state): State<Arc<AppState>>,
    Query(filtro): Query<OrdemFiltro>,
    _claims: Claims,
) -> Result<Json<Vec<OrdemServicoOut>>> {
    let hoje = Local::now().format("%Y-%m-%d").to_string();
    let atrasadas_num: i64 = if filtro.atrasadas.unwrap_or(false) { 1 } else { 0 };
    let busca = filtro.busca.as_deref().map(|b| format!("%{b}%"));

    let ordens = sqlx::query_as!(
        OrdemServico,
        r#"
        SELECT os.id as "id!", os.numero as "numero!", os.cliente_id as "cliente_id!", os.paciente_nome, os.cor_dente,
               os.data_entrada as "data_entrada!", os.data_prevista, os.data_entrega,
               os.status as "status!", os.status_pagamento as "status_pagamento!", os.valor_pago as "valor_pago!", os.observacoes,
               os.nfse_numero, os.nfse_status as "nfse_status!", os.nfse_chave, os.nfse_data_emissao, os.nfse_mensagem
        FROM ordens_servico os
        JOIN clientes c ON c.id = os.cliente_id
                WHERE ($1 IS NULL OR os.status = $2)
                    AND ($3 IS NULL OR os.cliente_id = $4)
                    AND ($5 IS NULL OR os.paciente_nome LIKE $6 OR c.nome LIKE $7)
                    AND ($8 = 0 OR (os.data_prevista < $9 AND os.status NOT IN ('entregue','cancelado')))
        ORDER BY os.numero DESC
        "#,
        filtro.status, filtro.status,
        filtro.cliente_id, filtro.cliente_id,
        busca, busca, busca,
        atrasadas_num, hoje
    )
    .fetch_all(&state.db)
    .await?;

    let mut resultado = Vec::with_capacity(ordens.len());
    for os in ordens {
        resultado.push(enriquecer_ordem(&state, os).await?);
    }
    Ok(Json(resultado))
}

/// POST /ordens-servico
pub async fn criar(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Json(payload): Json<OrdemServicoCreate>,
) -> Result<(StatusCode, Json<OrdemServicoOut>)> {
    let numero = proximo_numero(&state).await?;

    let id = sqlx::query!(
        r#"
        INSERT INTO ordens_servico
            (numero, cliente_id, paciente_nome, cor_dente, data_entrada, data_prevista, observacoes)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
        numero,
        payload.cliente_id,
        payload.paciente_nome,
        payload.cor_dente,
        payload.data_entrada,
        payload.data_prevista,
        payload.observacoes,
    )
    .execute(&state.db)
    .await?
    .fetch_one(&state.db)
    .await?
    .id;

    inserir_itens(&state, id, &payload.itens).await?;

    let os = buscar_ordem(&state, id).await?;
    Ok((StatusCode::CREATED, Json(os)))
}

/// GET /ordens-servico/:id
pub async fn obter(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<Json<OrdemServicoOut>> {
    let os = buscar_ordem(&state, id).await?;
    Ok(Json(os))
}

/// POST /ordens-servico/:id/nfse
pub async fn emitir_nfse(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<Json<OrdemServicoOut>> {
    let atual = sqlx::query_as!(
        OrdemServico,
        r#"SELECT id as "id!", numero as "numero!", cliente_id as "cliente_id!", paciente_nome, cor_dente, data_entrada as "data_entrada!",
                  data_prevista, data_entrega, status as "status!", status_pagamento as "status_pagamento!", valor_pago as "valor_pago!", observacoes,
                  nfse_numero, nfse_status as "nfse_status!", nfse_chave, nfse_data_emissao, nfse_mensagem
           FROM ordens_servico WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("OS {id} não encontrada")))?;

    if atual.nfse_status == "emitida" {
        return Ok(Json(buscar_ordem(&state, id).await?));
    }

    let numero: i64 = sqlx::query_scalar!(
        "SELECT COALESCE(MAX(CAST(nfse_numero AS INTEGER)), 0) + 1 FROM ordens_servico"
    )
    .fetch_one(&state.db)
    .await?;
    let nfse_numero = format!("{numero:08}");
    let nfse_chave = format!("NFSE-{}-{:08}", Local::now().format("%Y%m%d"), atual.numero);
    let nfse_data_emissao = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query!(
        r#"UPDATE ordens_servico
           SET nfse_numero = $1, nfse_status = 'emitida', nfse_chave = $2,
               nfse_data_emissao = $3, nfse_mensagem = $4, atualizado_em = CURRENT_TIMESTAMP
           WHERE id = $5"#,
        nfse_numero,
        nfse_chave,
        nfse_data_emissao,
        "NFS-e preparada para envio ao provedor municipal.",
        id,
    )
    .execute(&state.db)
    .await?;

    Ok(Json(buscar_ordem(&state, id).await?))
}

/// PUT /ordens-servico/:id
pub async fn atualizar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
    Json(payload): Json<OrdemServicoUpdate>,
) -> Result<Json<OrdemServicoOut>> {
    // Busca a OS atual para merge dos campos opcionais
    let atual = sqlx::query_as!(
        OrdemServico,
        r#"SELECT id as "id!", numero as "numero!", cliente_id as "cliente_id!", paciente_nome, cor_dente, data_entrada as "data_entrada!",
                  data_prevista, data_entrega, status as "status!", status_pagamento as "status_pagamento!", valor_pago as "valor_pago!", observacoes,
                  nfse_numero, nfse_status as "nfse_status!", nfse_chave, nfse_data_emissao, nfse_mensagem
           FROM ordens_servico WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("OS {id} não encontrada")))?;

    // Auto-preenche data_entrega ao marcar como entregue
    let nova_entrega = if payload.status.as_deref() == Some("entregue") && atual.data_entrega.is_none() {
        Some(Local::now().format("%Y-%m-%d").to_string())
    } else {
        payload.data_entrega.or(atual.data_entrega)
    };

    let cliente_id = payload.cliente_id.unwrap_or(atual.cliente_id);
    let paciente_nome = payload.paciente_nome.or(atual.paciente_nome);
    let cor_dente = payload.cor_dente.or(atual.cor_dente);
    let data_prevista = payload.data_prevista.or(atual.data_prevista);
    let status = payload.status.unwrap_or(atual.status);
    let status_pagamento = payload.status_pagamento.unwrap_or(atual.status_pagamento);
    let valor_pago = payload.valor_pago.unwrap_or(atual.valor_pago);
    let observacoes = payload.observacoes.or(atual.observacoes);

    sqlx::query!(
        r#"
        UPDATE ordens_servico SET
            cliente_id       = $1,
            paciente_nome    = $2,
            cor_dente        = $3,
            data_prevista    = $4,
            data_entrega     = $5,
            status           = $6,
            status_pagamento = $7,
            valor_pago       = $8,
            observacoes      = $9,
            atualizado_em    = CURRENT_TIMESTAMP
        WHERE id = $10
        "#,
        cliente_id,
        paciente_nome,
        cor_dente,
        data_prevista,
        nova_entrega,
        status,
        status_pagamento,
        valor_pago,
        observacoes,
        id,
    )
    .execute(&state.db)
    .await?;

    if let Some(itens) = &payload.itens {
        sqlx::query!("DELETE FROM itens_os WHERE ordem_id = $1", id)
            .execute(&state.db)
            .await?;
        inserir_itens(&state, id, itens).await?;
    }

    let os = buscar_ordem(&state, id).await?;
    Ok(Json(os))
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct DeleteParams {
    pub permanente: Option<bool>,
}

/// DELETE /ordens-servico/:id
pub async fn cancelar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Query(params): Query<DeleteParams>,
    _claims: Claims,
) -> Result<StatusCode> {
    if params.permanente.unwrap_or(false) {
        sqlx::query!("DELETE FROM itens_os WHERE ordem_id = $1", id)
            .execute(&state.db)
            .await?;

        let linhas = sqlx::query!("DELETE FROM ordens_servico WHERE id = $1", id)
            .execute(&state.db)
            .await?
            .rows_affected();

        if linhas == 0 {
            return Err(AppError::NotFound(format!("OS {id} não encontrada")));
        }
        return Ok(StatusCode::NO_CONTENT);
    }

    let linhas = sqlx::query!(
        "UPDATE ordens_servico SET status='cancelado', atualizado_em=CURRENT_TIMESTAMP WHERE id=$1",
        id
    )
    .execute(&state.db)
    .await?
    .rows_affected();

    if linhas == 0 {
        return Err(AppError::NotFound(format!("OS {id} não encontrada")));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ── helpers ───────────────────────────────────────────────────────────────────

async fn proximo_numero(state: &AppState) -> Result<i64> {
    let row = sqlx::query!("SELECT COALESCE(MAX(numero), 0) AS max FROM ordens_servico")
        .fetch_one(&state.db)
        .await?;
    Ok(row.max + 1)
}

async fn inserir_itens(state: &AppState, ordem_id: i64, itens: &[ItemOSPayload]) -> Result<()> {
    for item in itens {
        sqlx::query!(
            "INSERT INTO itens_os (ordem_id, tipo_servico_id, dente_arcada, quantidade, valor_unitario)
             VALUES ($1, $2, $3, $4, $5)",
            ordem_id, item.tipo_servico_id, item.dente_arcada, item.quantidade, item.valor_unitario
        )
        .execute(&state.db)
        .await?;
    }
    Ok(())
}

async fn buscar_ordem(state: &AppState, id: i64) -> Result<OrdemServicoOut> {
    let os = sqlx::query_as!(
        OrdemServico,
        r#"SELECT id as "id!", numero as "numero!", cliente_id as "cliente_id!", paciente_nome, cor_dente, data_entrada as "data_entrada!",
                  data_prevista, data_entrega, status as "status!", status_pagamento as "status_pagamento!", valor_pago as "valor_pago!", observacoes,
                  nfse_numero, nfse_status as "nfse_status!", nfse_chave, nfse_data_emissao, nfse_mensagem
           FROM ordens_servico WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("OS {id} não encontrada")))?;

    enriquecer_ordem(state, os).await
}

async fn enriquecer_ordem(state: &AppState, os: OrdemServico) -> Result<OrdemServicoOut> {
    let cliente = sqlx::query_as!(
        Cliente,
        r#"SELECT id as "id!", nome, cpf_cnpj, telefone, email, endereco, observacoes,
                  ativo as "ativo!: bool" FROM clientes WHERE id = $1"#,
        os.cliente_id
    )
    .fetch_one(&state.db)
    .await?;

    let itens_raw = sqlx::query_as!(
        ItemOS,
        r#"SELECT id as "id!", ordem_id as "ordem_id!", tipo_servico_id as "tipo_servico_id!", dente_arcada, quantidade as "quantidade!", valor_unitario as "valor_unitario!"
         FROM itens_os WHERE ordem_id = $1"#,
        os.id
    )
    .fetch_all(&state.db)
    .await?;

    let mut itens_out = Vec::with_capacity(itens_raw.len());
    let mut valor_total = 0.0_f64;

    for item in itens_raw {
        let tipo = sqlx::query_as!(
            TipoServico,
            r#"SELECT id as "id!", nome, categoria, valor_padrao as "valor_padrao!", prazo_dias as "prazo_dias!", ativo as "ativo!: bool"
               FROM tipos_servico WHERE id = $1"#,
            item.tipo_servico_id
        )
        .fetch_one(&state.db)
        .await?;

        let vt = item.quantidade as f64 * item.valor_unitario;
        valor_total += vt;

        itens_out.push(ItemOSOut {
            id: item.id,
            tipo_servico_id: item.tipo_servico_id,
            dente_arcada: item.dente_arcada,
            quantidade: item.quantidade,
            valor_unitario: item.valor_unitario,
            valor_total: vt,
            tipo_servico: tipo,
        });
    }

    Ok(OrdemServicoOut {
        id: os.id,
        numero: os.numero,
        cliente_id: os.cliente_id,
        paciente_nome: os.paciente_nome,
        cor_dente: os.cor_dente,
        data_entrada: os.data_entrada,
        data_prevista: os.data_prevista,
        data_entrega: os.data_entrega,
        status: os.status,
        status_pagamento: os.status_pagamento,
        valor_pago: os.valor_pago,
        valor_total,
        observacoes: os.observacoes,
        nfse_numero: os.nfse_numero,
        nfse_status: os.nfse_status,
        nfse_chave: os.nfse_chave,
        nfse_data_emissao: os.nfse_data_emissao,
        nfse_mensagem: os.nfse_mensagem,
        cliente,
        itens: itens_out,
    })
}

/// GET /ordens-servico/:id/recibo
pub async fn gerar_recibo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<Response> {
    let os = buscar_ordem(&state, id).await?;
    let pdf_bytes = renderizar_pdf_recibo(&state.config, &os)?;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/pdf"),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::HeaderValue::from_str(&format!(
            "inline; filename=\"recibo_os_{:05}.pdf\"",
            os.numero
        ))
        .unwrap_or_else(|_| axum::http::HeaderValue::from_static("inline; filename=\"recibo.pdf\"")),
    );

    Ok((headers, pdf_bytes).into_response())
}

fn renderizar_pdf_recibo(config: &crate::config::Config, os: &OrdemServicoOut) -> Result<Vec<u8>> {
    use printpdf::*;

    let (doc, page1, layer1) = PdfDocument::new(
        format!("Recibo OS #{:05}", os.numero),
        Mm(210.0),
        Mm(297.0),
        "Layer 1",
    );
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).map_err(|e| anyhow::anyhow!(e))?;
    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| anyhow::anyhow!(e))?;

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Cabeçalho
    current_layer.begin_text_section();
    current_layer.set_font(&font_bold, 20.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(270.0));
    current_layer.write_text(&config.lab_nome, &font_bold);
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font_regular, 10.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(262.0));
    current_layer.write_text(format!("CNPJ: {} | Tel: {}", config.lab_cnpj, config.lab_telefone), &font_regular);
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font_regular, 10.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(256.0));
    current_layer.write_text(&config.lab_endereco, &font_regular);
    current_layer.end_text_section();

    // Título
    current_layer.begin_text_section();
    current_layer.set_font(&font_bold, 16.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(240.0));
    current_layer.write_text(format!("ORDEM DE SERVICO #{:05}", os.numero), &font_bold);
    current_layer.end_text_section();

    // Dados da OS
    let mut y = 228.0;
    current_layer.begin_text_section();
    current_layer.set_font(&font_regular, 11.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(y));
    current_layer.write_text(format!("Cliente: {}", os.cliente.nome), &font_regular);
    current_layer.end_text_section();

    y -= 6.0;
    current_layer.begin_text_section();
    current_layer.set_font(&font_regular, 11.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(y));
    current_layer.write_text(format!("Paciente: {} | Cor/Escala: {}", os.paciente_nome.as_deref().unwrap_or("-"), os.cor_dente.as_deref().unwrap_or("-")), &font_regular);
    current_layer.end_text_section();

    y -= 6.0;
    current_layer.begin_text_section();
    current_layer.set_font(&font_regular, 11.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(y));
    current_layer.write_text(format!("Data Entrada: {} | Previsao: {}", os.data_entrada, os.data_prevista.as_deref().unwrap_or("-")), &font_regular);
    current_layer.end_text_section();

    // Itens
    y -= 15.0;
    current_layer.begin_text_section();
    current_layer.set_font(&font_bold, 12.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(y));
    current_layer.write_text("SERVICOS DISCRIMINADOS", &font_bold);
    current_layer.end_text_section();

    y -= 8.0;
    for item in &os.itens {
        current_layer.begin_text_section();
        current_layer.set_font(&font_regular, 10.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(y));
        let dente = if let Some(ref d) = item.dente_arcada { format!(" ({})", d) } else { "".into() };
        current_layer.write_text(format!("- {}{} | Qtd: {} | R$ {:.2} un | Total: R$ {:.2}", item.tipo_servico.nome, dente, item.quantidade, item.valor_unitario, item.valor_total), &font_regular);
        current_layer.end_text_section();
        y -= 6.0;
    }

    // Valoração
    y -= 10.0;
    current_layer.begin_text_section();
    current_layer.set_font(&font_bold, 12.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(y));
    current_layer.write_text(format!("VALOR TOTAL: R$ {:.2}", os.valor_total), &font_bold);
    current_layer.end_text_section();

    y -= 6.0;
    current_layer.begin_text_section();
    current_layer.set_font(&font_regular, 11.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(y));
    current_layer.write_text(format!("Valor Pago: R$ {:.2} | Status Pagamento: {}", os.valor_pago, os.status_pagamento.to_uppercase()), &font_regular);
    current_layer.end_text_section();

    let bytes = doc.save_to_bytes().map_err(|e| anyhow::anyhow!(e))?;
    Ok(bytes)
}
