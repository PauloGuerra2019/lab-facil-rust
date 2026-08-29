-- Migration 003_solicitacoes_acesso.sql: Solicitações de acesso para aprovação

CREATE TABLE IF NOT EXISTS solicitacoes_acesso (
    id              BIGSERIAL PRIMARY KEY,
    nome            TEXT    NOT NULL,
    email           TEXT    NOT NULL UNIQUE,
    empresa         TEXT,
    telefone        TEXT,
    mensagem        TEXT,
    status          TEXT    NOT NULL DEFAULT 'pendente',
    criado_em       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    atualizacao_em  TIMESTAMP WITH TIME ZONE
);

CREATE INDEX IF NOT EXISTS idx_solicitacoes_acesso_email
    ON solicitacoes_acesso(email);

CREATE INDEX IF NOT EXISTS idx_solicitacoes_acesso_status
    ON solicitacoes_acesso(status);
