// src/config.rs
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url:              String,
    pub jwt_secret:                String,
    pub jwt_expiration_hours:      u64,
    pub cors_origin:               String,
    pub aprovacao_email:           String,
    pub smtp_host:                 Option<String>,
    pub smtp_port:                 u16,
    pub smtp_username:             Option<String>,
    pub smtp_password:             Option<String>,
    pub smtp_from:                 Option<String>,

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
            smtp_host:              env::var("SMTP_HOST").ok(),
            smtp_port:              env::var("SMTP_PORT")
                                        .ok()
                                        .and_then(|v| v.parse().ok())
                                        .unwrap_or(587),
            smtp_username:          env::var("SMTP_USERNAME").ok(),
            smtp_password:          env::var("SMTP_PASSWORD").ok(),
            smtp_from:              env::var("SMTP_FROM").ok(),
            lab_nome:      env::var("LAB_NOME").unwrap_or_else(|_| "DADG - Laboratório de prótese dentária".into()),
            lab_cnpj:      env::var("LAB_CNPJ").unwrap_or_else(|_| "64.329.994/0001-77".into()),
            lab_endereco:  env::var("LAB_ENDERECO").unwrap_or_else(|_| "Rua Carlos Luvison, 376 - Parque Bela Vista - Votorantim/SP - CEP 18110-435".into()),
            lab_telefone:  env::var("LAB_TELEFONE").unwrap_or_else(|_| "(15) 99719-7692".into()),
        }
    }
}
