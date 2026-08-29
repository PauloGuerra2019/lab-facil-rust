// src/config.rs
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url:              String,
    pub jwt_secret:                String,
    pub jwt_expiration_hours:      u64,
    pub cors_origin:               String,
    pub aprovacao_email:           String,

    // Dados do laboratório (usados no PDF/recibo)
    pub lab_nome:      String,
    pub lab_cnpj:      String,
    pub lab_endereco:  String,
    pub lab_telefone:  String,
}

impl Config {
    /// Lê as variáveis de ambiente (com valores padrão seguros para dev).
    /// Em produção, sempre defina JWT_SECRET com uma string aleatória longa.
    pub fn from_env() -> Self {
        Self {
            database_url:           env::var("DATABASE_URL")
                                        .expect("DATABASE_URL deve estar definido"),
            jwt_secret:             env::var("JWT_SECRET")
                                        .expect("JWT_SECRET deve estar definido"),
            jwt_expiration_hours:   env::var("JWT_EXPIRATION_HOURS")
                                        .ok()
                                        .and_then(|v| v.parse().ok())
                                        .unwrap_or(12),
            cors_origin:            env::var("CORS_ORIGIN")
                                        .unwrap_or_else(|_| "http://localhost:5173".into()),
            aprovacao_email:        env::var("APROVACAO_EMAIL")
                                        .unwrap_or_else(|_| "contato@dadg.com.br".into()),
            lab_nome:      env::var("LAB_NOME").unwrap_or_else(|_| "Laboratório de Prótese".into()),
            lab_cnpj:      env::var("LAB_CNPJ").unwrap_or_else(|_| "00.000.000/0001-00".into()),
            lab_endereco:  env::var("LAB_ENDERECO").unwrap_or_else(|_| "Endereço do laboratório".into()),
            lab_telefone:  env::var("LAB_TELEFONE").unwrap_or_else(|_| "(00) 00000-0000".into()),
        }
    }
}
