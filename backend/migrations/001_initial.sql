-- Lab Fácil — schema inicial
-- SQLx roda automaticamente os arquivos de migrations/ na ordem do prefixo numérico

CREATE TABLE IF NOT EXISTS usuarios (
    id         BIGSERIAL PRIMARY KEY,
    nome       TEXT    NOT NULL,
    email      TEXT    NOT NULL UNIQUE,
    senha_hash TEXT    NOT NULL,
    role       TEXT    NOT NULL DEFAULT 'operador',  -- 'admin' | 'operador'
    ativo      BOOLEAN NOT NULL DEFAULT TRUE,
    criado_em  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS clientes (
    id           BIGSERIAL PRIMARY KEY,
    nome         TEXT    NOT NULL,
    cpf_cnpj     TEXT,
    telefone     TEXT,
    email        TEXT,
    endereco     TEXT,
    observacoes  TEXT,
    ativo        BOOLEAN NOT NULL DEFAULT TRUE,
    criado_em    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tipos_servico (
    id           BIGSERIAL PRIMARY KEY,
    nome         TEXT    NOT NULL,
    categoria    TEXT,
    valor_padrao DOUBLE PRECISION NOT NULL DEFAULT 0,
    prazo_dias   INTEGER NOT NULL DEFAULT 5,
    ativo        BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS ordens_servico (
    id                BIGSERIAL PRIMARY KEY,
    numero            BIGINT NOT NULL UNIQUE,
    cliente_id        BIGINT NOT NULL REFERENCES clientes(id),
    paciente_nome     TEXT,
    cor_dente         TEXT,
    data_entrada      TEXT    NOT NULL,  -- ISO 8601 date (YYYY-MM-DD)
    data_prevista     TEXT,
    data_entrega      TEXT,
    status            TEXT    NOT NULL DEFAULT 'recebido',
    -- 'recebido' | 'em_producao' | 'controle_qualidade' | 'pronto' | 'entregue' | 'cancelado'
    status_pagamento  TEXT    NOT NULL DEFAULT 'pendente',
    -- 'pendente' | 'parcial' | 'pago'
    valor_pago        DOUBLE PRECISION NOT NULL DEFAULT 0,
    observacoes       TEXT,
    criado_em         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS itens_os (
    id               BIGSERIAL PRIMARY KEY,
    ordem_id         BIGINT NOT NULL REFERENCES ordens_servico(id) ON DELETE CASCADE,
    tipo_servico_id  BIGINT NOT NULL REFERENCES tipos_servico(id),
    dente_arcada     TEXT,
    quantidade       BIGINT NOT NULL DEFAULT 1,
    valor_unitario   DOUBLE PRECISION NOT NULL DEFAULT 0
);

-- Índices
CREATE INDEX IF NOT EXISTS idx_ordens_cliente   ON ordens_servico(cliente_id);
CREATE INDEX IF NOT EXISTS idx_ordens_status    ON ordens_servico(status);
CREATE INDEX IF NOT EXISTS idx_itens_ordem      ON itens_os(ordem_id);

INSERT INTO usuarios (nome, email, senha_hash, role)
VALUES ('Administrador', 'admin@laboratorio.com',
        -- Hash argon2id de "admin123" — gerado em runtime, este valor é placeholder
        -- O seed real acontece via código Rust no startup
    '$placeholder$', 'admin')
ON CONFLICT (email) DO NOTHING;

INSERT INTO tipos_servico (nome, categoria, valor_padrao, prazo_dias) VALUES
    ('Coroa de Porcelana',              'Prótese Fixa',       280.00, 7),
    ('Coroa de Zircônia',               'Prótese Fixa',       450.00, 10),
    ('Faceta de Porcelana',             'Estética',           380.00, 8),
    ('Prótese Parcial Removível (PPR)', 'Prótese Removível',  350.00, 12),
    ('Prótese Total',                   'Prótese Removível',  500.00, 15),
    ('Placa de Bruxismo',               'Ortodontia',         150.00, 5),
    ('Provisório em Resina',            'Prótese Fixa',        90.00, 2),
    ('Núcleo de Preenchimento',         'Prótese Fixa',       120.00, 4)
ON CONFLICT DO NOTHING;
