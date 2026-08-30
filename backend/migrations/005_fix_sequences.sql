-- Migration 005_fix_sequences.sql: Sincroniza sequências dos IDs para evitar conflito de chave primária em INSERTs

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'clientes_id_seq') THEN
        PERFORM setval('clientes_id_seq', COALESCE((SELECT MAX(id) FROM clientes), 1));
    END IF;
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'tipos_servico_id_seq') THEN
        PERFORM setval('tipos_servico_id_seq', COALESCE((SELECT MAX(id) FROM tipos_servico), 1));
    END IF;
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'ordens_servico_id_seq') THEN
        PERFORM setval('ordens_servico_id_seq', COALESCE((SELECT MAX(id) FROM ordens_servico), 1));
    END IF;
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'usuarios_id_seq') THEN
        PERFORM setval('usuarios_id_seq', COALESCE((SELECT MAX(id) FROM usuarios), 1));
    END IF;
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'itens_os_id_seq') THEN
        PERFORM setval('itens_os_id_seq', COALESCE((SELECT MAX(id) FROM itens_os), 1));
    END IF;
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'solicitacoes_acesso_id_seq') THEN
        PERFORM setval('solicitacoes_acesso_id_seq', COALESCE((SELECT MAX(id) FROM solicitacoes_acesso), 1));
    END IF;
END $$;
