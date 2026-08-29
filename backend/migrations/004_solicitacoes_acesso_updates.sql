-- Migration 004_solicitacoes_acesso_updates.sql: Add UNIQUE constraint and update tracking

ALTER TABLE solicitacoes_acesso
ADD CONSTRAINT unique_email_solicitacao UNIQUE (email);

ALTER TABLE solicitacoes_acesso
ADD COLUMN atualizacao_em TIMESTAMP WITH TIME ZONE;
