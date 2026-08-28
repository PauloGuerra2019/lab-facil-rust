use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ── Usuário ───────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize)]
pub struct Usuario {
    pub id:         i64,
    pub nome:       String,
    pub email:      String,
    #[serde(skip_serializing)]   // nunca expõe o hash na API
    pub senha_hash: String,
    pub role:       String,
    pub ativo:      bool,
}

// ── Cliente ───────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct Cliente {
    pub id:          i64,
    pub nome:        String,
    pub cpf_cnpj:    Option<String>,
    pub telefone:    Option<String>,
    pub email:       Option<String>,
    pub endereco:    Option<String>,
    pub observacoes: Option<String>,
    pub ativo:       bool,
}

#[derive(Debug, Deserialize)]
pub struct ClientePayload {
    pub nome:        String,
    pub cpf_cnpj:    Option<String>,
    pub telefone:    Option<String>,
    pub email:       Option<String>,
    pub endereco:    Option<String>,
    pub observacoes: Option<String>,
}

// ── Tipo de Serviço ───────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct TipoServico {
    pub id:           i64,
    pub nome:         String,
    pub categoria:    Option<String>,
    pub valor_padrao: f64,
    pub prazo_dias:   i64,
    pub ativo:        bool,
}

#[derive(Debug, Deserialize)]
pub struct TipoServicoPayload {
    pub nome:         String,
    pub categoria:    Option<String>,
    pub valor_padrao: f64,
    pub prazo_dias:   i64,
}

// ── Item da OS ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct ItemOS {
    pub id:              i64,
    pub ordem_id:        i64,
    pub tipo_servico_id: i64,
    pub dente_arcada:    Option<String>,
    pub quantidade:      i64,
    pub valor_unitario:  f64,
}

/// Versão "rich" do item que inclui os dados do tipo de serviço (para a resposta)
#[derive(Debug, Serialize)]
pub struct ItemOSOut {
    pub id:              i64,
    pub tipo_servico_id: i64,
    pub dente_arcada:    Option<String>,
    pub quantidade:      i64,
    pub valor_unitario:  f64,
    pub valor_total:     f64,
    pub tipo_servico:    TipoServico,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ItemOSPayload {
    pub tipo_servico_id: i64,
    pub dente_arcada:    Option<String>,
    pub quantidade:      i64,
    pub valor_unitario:  f64,
}

// ── Ordem de Serviço ──────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize)]
pub struct OrdemServico {
    pub id:               i64,
    pub numero:           i64,
    pub cliente_id:       i64,
    pub paciente_nome:    Option<String>,
    pub cor_dente:        Option<String>,
    pub data_entrada:     String,
    pub data_prevista:    Option<String>,
    pub data_entrega:     Option<String>,
    pub status:           String,
    pub status_pagamento: String,
    pub valor_pago:       f64,
    pub observacoes:      Option<String>,
    // NFS-e MEI
    pub nfse_numero:       Option<String>,
    pub nfse_status:       String,
    pub nfse_chave:        Option<String>,
    pub nfse_data_emissao: Option<String>,
    pub nfse_mensagem:     Option<String>,
}

/// Resposta completa com cliente e itens aninhados
#[derive(Debug, Serialize)]
pub struct OrdemServicoOut {
    pub id:               i64,
    pub numero:           i64,
    pub cliente_id:       i64,
    pub paciente_nome:    Option<String>,
    pub cor_dente:        Option<String>,
    pub data_entrada:     String,
    pub data_prevista:    Option<String>,
    pub data_entrega:     Option<String>,
    pub status:           String,
    pub status_pagamento: String,
    pub valor_pago:       f64,
    pub valor_total:      f64,   // calculado a partir dos itens
    pub observacoes:      Option<String>,
    // NFS-e MEI
    pub nfse_numero:       Option<String>,
    pub nfse_status:       String,
    pub nfse_chave:        Option<String>,
    pub nfse_data_emissao: Option<String>,
    pub nfse_mensagem:     Option<String>,
    pub cliente:          Cliente,
    pub itens:            Vec<ItemOSOut>,
}

#[derive(Debug, Deserialize)]
pub struct OrdemServicoCreate {
    pub cliente_id:    i64,
    pub paciente_nome: Option<String>,
    pub cor_dente:     Option<String>,
    pub data_entrada:  String,
    pub data_prevista: Option<String>,
    pub observacoes:   Option<String>,
    pub itens:         Vec<ItemOSPayload>,
}

#[derive(Debug, Deserialize, Default)]
pub struct OrdemServicoUpdate {
    pub cliente_id:       Option<i64>,
    pub paciente_nome:    Option<String>,
    pub cor_dente:        Option<String>,
    pub data_prevista:    Option<String>,
    pub data_entrega:     Option<String>,
    pub status:           Option<String>,
    pub status_pagamento: Option<String>,
    pub valor_pago:       Option<f64>,
    pub observacoes:      Option<String>,
    pub itens:            Option<Vec<ItemOSPayload>>,
}

// ── Filtros de listagem ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Default)]
pub struct OrdemFiltro {
    pub status:     Option<String>,
    pub cliente_id: Option<i64>,
    pub busca:      Option<String>,
    pub atrasadas:  Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ClienteFiltro {
    pub busca:        Option<String>,
    pub apenas_ativos: Option<bool>,
}
