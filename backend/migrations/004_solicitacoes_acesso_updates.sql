-- Migration 004_solicitacoes_acesso_updates.sql: Add UNIQUE constraint and update tracking (Idempotent)

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'unique_email_solicitacao'
    ) THEN
        ALTER TABLE solicitacoes_acesso ADD CONSTRAINT unique_email_solicitacao UNIQUE (email);
    END IF;
END $$;

ALTER TABLE solicitacoes_acesso
ADD COLUMN IF NOT EXISTS atualizacao_em TIMESTAMP WITH TIME ZONE;
