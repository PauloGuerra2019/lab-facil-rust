-- Migration 002_nfse.sql: Adiciona suporte a NFS-e MEI nas Ordens de Serviço

ALTER TABLE ordens_servico ADD COLUMN IF NOT EXISTS nfse_numero TEXT;
ALTER TABLE ordens_servico ADD COLUMN IF NOT EXISTS nfse_status TEXT NOT NULL DEFAULT 'nao_emitida';
ALTER TABLE ordens_servico ADD COLUMN IF NOT EXISTS nfse_chave TEXT;
ALTER TABLE ordens_servico ADD COLUMN IF NOT EXISTS nfse_data_emissao TEXT;
ALTER TABLE ordens_servico ADD COLUMN IF NOT EXISTS nfse_mensagem TEXT;
