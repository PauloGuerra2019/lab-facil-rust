import { FileDown, Plus, ReceiptText, Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import api from "../api.js";
import { STATUS_OPTIONS } from "../components/StatusBadge.jsx";

const formatoMoeda = (v) => new Intl.NumberFormat("pt-BR", { style: "currency", currency: "BRL" }).format(v || 0);

function novoItem(tipos) {
  return { tipo_servico_id: tipos[0]?.id || "", dente_arcada: "", quantidade: 1, valor_unitario: tipos[0]?.valor_padrao || 0 };
}

export default function OrdemDetalhe() {
  const { id } = useParams();
  const navigate = useNavigate();
  const editando = Boolean(id);

  const [clientes, setClientes] = useState([]);
  const [tipos, setTipos] = useState([]);
  const [ordem, setOrdem] = useState(null); // OS já salva (modo edição)

  const [form, setForm] = useState({
    cliente_id: "",
    paciente_nome: "",
    cor_dente: "",
    data_entrada: new Date().toISOString().slice(0, 10),
    data_prevista: "",
    observacoes: "",
    status: "recebido",
    status_pagamento: "pendente",
    valor_pago: 0,
  });
  const [itens, setItens] = useState([]);
  const [salvando, setSalvando] = useState(false);
  const [gerandoPdf, setGerandoPdf] = useState(false);
  const [nfseEmitindo, setNfseEmitindo] = useState(false);
  const [erro, setErro] = useState("");

  useEffect(() => {
    async function carregarBase() {
      const [{ data: clientesData }, { data: tiposData }] = await Promise.all([
        api.get("/clientes"),
        api.get("/tipos-servico"),
      ]);
      setClientes(clientesData);
      setTipos(tiposData);

      if (editando) {
        const { data: os } = await api.get(`/ordens-servico/${id}`);
        setOrdem(os);
        setForm({
          cliente_id: os.cliente_id,
          paciente_nome: os.paciente_nome || "",
          cor_dente: os.cor_dente || "",
          data_entrada: os.data_entrada,
          data_prevista: os.data_prevista || "",
          observacoes: os.observacoes || "",
          status: os.status,
          status_pagamento: os.status_pagamento,
          valor_pago: os.valor_pago,
        });
        setItens(
          os.itens.map((i) => ({
            tipo_servico_id: i.tipo_servico_id,
            dente_arcada: i.dente_arcada || "",
            quantidade: i.quantidade,
            valor_unitario: i.valor_unitario,
          }))
        );
      } else if (tiposData.length > 0) {
        setItens([novoItem(tiposData)]);
      }
    }
    carregarBase();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [id]);

  function atualizarItem(index, campo, valor) {
    setItens((prev) => {
      const copia = [...prev];
      copia[index] = { ...copia[index], [campo]: valor };
      if (campo === "tipo_servico_id") {
        const tipo = tipos.find((t) => t.id === Number(valor));
        if (tipo) copia[index].valor_unitario = tipo.valor_padrao;
      }
      return copia;
    });
  }

  function adicionarItem() {
    setItens((prev) => [...prev, novoItem(tipos)]);
  }

  function removerItem(index) {
    setItens((prev) => prev.filter((_, i) => i !== index));
  }

  const valorTotal = itens.reduce((soma, i) => soma + Number(i.quantidade || 0) * Number(i.valor_unitario || 0), 0);

  async function salvar(e) {
    e.preventDefault();
    setErro("");
    if (itens.length === 0) {
      setErro("Adicione ao menos um serviço à ordem.");
      return;
    }
    setSalvando(true);
    try {
      const payloadItens = itens.map((i) => ({
        tipo_servico_id: Number(i.tipo_servico_id),
        dente_arcada: i.dente_arcada,
        quantidade: Number(i.quantidade),
        valor_unitario: Number(i.valor_unitario),
      }));

      if (editando) {
        await api.put(`/ordens-servico/${id}`, {
          ...form,
          cliente_id: Number(form.cliente_id),
          data_prevista: form.data_prevista || null,
          valor_pago: Number(form.valor_pago),
          itens: payloadItens,
        });
      } else {
        await api.post("/ordens-servico", {
          cliente_id: Number(form.cliente_id),
          paciente_nome: form.paciente_nome,
          cor_dente: form.cor_dente,
          data_entrada: form.data_entrada,
          data_prevista: form.data_prevista || null,
          observacoes: form.observacoes,
          itens: payloadItens,
        });
      }
      navigate("/ordens");
    } catch (err) {
      setErro(err.response?.data?.detail || "Não foi possível salvar a ordem de serviço.");
    } finally {
      setSalvando(false);
    }
  }

  async function baixarRecibo() {
    setGerandoPdf(true);
    try {
      const { data } = await api.get(`/ordens-servico/${id}/recibo`, { responseType: "blob" });
      const url = window.URL.createObjectURL(new Blob([data], { type: "application/pdf" }));
      window.open(url, "_blank");
    } finally {
      setGerandoPdf(false);
    }
  }

  async function emitirNfse() {
    setNfseEmitindo(true);
    setErro("");
    try {
      const { data } = await api.post(`/ordens-servico/${id}/nfse`);
      setOrdem(data);
    } catch (err) {
      setErro(err.response?.data?.detail || "Não foi possível emitir a NFS-e.");
    } finally {
      setNfseEmitindo(false);
    }
  }

  async function excluirOrdem() {
    if (confirm(`Tem certeza que deseja excluir permanentemente esta Ordem de Serviço?`)) {
      try {
        await api.delete(`/ordens-servico/${id}?permanente=true`);
        navigate("/ordens");
      } catch (err) {
        setErro(err.response?.data?.detail || "Erro ao excluir a OS.");
      }
    }
  }

  return (
    <div className="max-w-3xl space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="font-display text-2xl">
            {editando ? `OS #${String(ordem?.numero || "").padStart(5, "0")}` : "Nova Ordem de Serviço"}
          </h1>
          <p className="text-sm text-ink/50">Registro de entrada, itens e valores do trabalho</p>
        </div>
        {editando && (
          <div className="flex items-center gap-2">
            <button type="button" onClick={excluirOrdem} className="btn-secondary text-brick hover:bg-brick/10">
              <Trash2 size={16} />
              Excluir OS
            </button>
            <button type="button" onClick={baixarRecibo} disabled={gerandoPdf} className="btn-secondary">
              <FileDown size={16} />
              {gerandoPdf ? "Gerando…" : "Baixar recibo (PDF)"}
            </button>
          </div>
        )}
      </div>

      {editando && ordem && (
        <section className="card p-6 space-y-4">
          <div className="flex items-start justify-between gap-4">
            <div>
              <div className="flex items-center gap-2">
                <ReceiptText size={18} className="text-teal" />
                <h2 className="font-display text-lg">NFS-e</h2>
              </div>
              <p className="text-sm text-ink/50 mt-1">
                Emissão local preparada para envio ao provedor municipal.
              </p>
            </div>
            {ordem.nfse_status !== "emitida" && (
              <button type="button" onClick={emitirNfse} disabled={nfseEmitindo} className="btn-primary">
                <ReceiptText size={16} />
                {nfseEmitindo ? "Emitindo…" : "Emitir NFS-e"}
              </button>
            )}
          </div>

          {ordem.nfse_status === "emitida" ? (
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 border-t border-line pt-4">
              <div>
                <p className="label">Status</p>
                <p className="text-sm font-medium text-teal">Emitida</p>
              </div>
              <div>
                <p className="label">Número</p>
                <p className="text-sm font-mono">{ordem.nfse_numero}</p>
              </div>
              <div>
                <p className="label">Data de emissão</p>
                <p className="text-sm">{ordem.nfse_data_emissao}</p>
              </div>
              <div>
                <p className="label">Chave</p>
                <p className="text-sm font-mono break-all">{ordem.nfse_chave}</p>
              </div>
              {ordem.nfse_mensagem && <p className="col-span-full text-xs text-ink/60">{ordem.nfse_mensagem}</p>}
            </div>
          ) : (
            <p className="text-sm text-ink/60 border-t border-line pt-4">Ainda não emitida.</p>
          )}
        </section>
      )}

      <form onSubmit={salvar} className="space-y-6">
        <div className="card p-6 space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="label">Cliente (dentista/clínica) *</label>
              <select
                required
                className="input"
                value={form.cliente_id}
                onChange={(e) => setForm({ ...form, cliente_id: e.target.value })}
              >
                <option value="">Selecione…</option>
                {clientes.map((c) => (
                  <option key={c.id} value={c.id}>
                    {c.nome}
                  </option>
                ))}
              </select>
            </div>
            <div>
              <label className="label">Paciente</label>
              <input
                className="input"
                value={form.paciente_nome}
                onChange={(e) => setForm({ ...form, paciente_nome: e.target.value })}
              />
            </div>
          </div>

          <div className="grid grid-cols-3 gap-4">
            <div>
              <label className="label">Data de entrada *</label>
              <input
                required
                type="date"
                className="input"
                value={form.data_entrada}
                onChange={(e) => setForm({ ...form, data_entrada: e.target.value })}
              />
            </div>
            <div>
              <label className="label">Previsão de entrega</label>
              <input
                type="date"
                className="input"
                value={form.data_prevista}
                onChange={(e) => setForm({ ...form, data_prevista: e.target.value })}
              />
            </div>
            <div>
              <label className="label">Cor/Escala (ex: A2)</label>
              <input
                className="input"
                value={form.cor_dente}
                onChange={(e) => setForm({ ...form, cor_dente: e.target.value })}
              />
            </div>
          </div>

          {editando && (
            <div className="grid grid-cols-3 gap-4 pt-2 border-t border-line">
              <div>
                <label className="label">Status</label>
                <select
                  className="input"
                  value={form.status}
                  onChange={(e) => setForm({ ...form, status: e.target.value })}
                >
                  {STATUS_OPTIONS.map((s) => (
                    <option key={s.value} value={s.value}>
                      {s.label}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Status do pagamento</label>
                <select
                  className="input"
                  value={form.status_pagamento}
                  onChange={(e) => setForm({ ...form, status_pagamento: e.target.value })}
                >
                  <option value="pendente">Pendente</option>
                  <option value="parcial">Parcial</option>
                  <option value="pago">Pago</option>
                </select>
              </div>
              <div>
                <label className="label">Valor pago (R$)</label>
                <input
                  type="number"
                  step="0.01"
                  min="0"
                  className="input"
                  value={form.valor_pago}
                  onChange={(e) => setForm({ ...form, valor_pago: e.target.value })}
                />
              </div>
            </div>
          )}

          <div>
            <label className="label">Observações</label>
            <textarea
              className="input"
              rows={2}
              value={form.observacoes}
              onChange={(e) => setForm({ ...form, observacoes: e.target.value })}
            />
          </div>
        </div>

        <div className="card p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="font-display text-lg">Serviços</h2>
            <button type="button" onClick={adicionarItem} className="btn-secondary text-xs px-3 py-1.5">
              <Plus size={14} />
              Adicionar serviço
            </button>
          </div>

          <div className="space-y-3">
            {itens.map((item, index) => (
              <div key={index} className="grid grid-cols-12 gap-2 items-end">
                <div className="col-span-4">
                  {index === 0 && <label className="label">Serviço</label>}
                  <select
                    required
                    className="input"
                    value={item.tipo_servico_id}
                    onChange={(e) => atualizarItem(index, "tipo_servico_id", e.target.value)}
                  >
                    {tipos.map((t) => (
                      <option key={t.id} value={t.id}>
                        {t.nome}
                      </option>
                    ))}
                  </select>
                </div>
                <div className="col-span-3">
                  {index === 0 && <label className="label">Dente(s)/Arcada</label>}
                  <input
                    className="input"
                    value={item.dente_arcada}
                    onChange={(e) => atualizarItem(index, "dente_arcada", e.target.value)}
                  />
                </div>
                <div className="col-span-1">
                  {index === 0 && <label className="label">Qtd.</label>}
                  <input
                    type="number"
                    min="1"
                    className="input"
                    value={item.quantidade}
                    onChange={(e) => atualizarItem(index, "quantidade", e.target.value)}
                  />
                </div>
                <div className="col-span-2">
                  {index === 0 && <label className="label">Valor unit.</label>}
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    className="input"
                    value={item.valor_unitario}
                    onChange={(e) => atualizarItem(index, "valor_unitario", e.target.value)}
                  />
                </div>
                <div className="col-span-1 font-mono text-sm text-right pb-2">
                  {formatoMoeda(item.quantidade * item.valor_unitario)}
                </div>
                <div className="col-span-1 pb-2 flex justify-end">
                  <button
                    type="button"
                    onClick={() => removerItem(index)}
                    className="text-ink/30 hover:text-brick"
                  >
                    <Trash2 size={15} />
                  </button>
                </div>
              </div>
            ))}
          </div>

          <div className="flex justify-end mt-4 pt-4 border-t border-line">
            <p className="font-display text-xl">
              Total: <span className="font-mono">{formatoMoeda(valorTotal)}</span>
            </p>
          </div>
        </div>

        {erro && <p className="text-sm text-brick">{erro}</p>}

        <div className="flex justify-end gap-3">
          <button type="button" onClick={() => navigate("/ordens")} className="btn-secondary">
            Cancelar
          </button>
          <button type="submit" disabled={salvando} className="btn-primary">
            {salvando ? "Salvando…" : "Salvar Ordem de Serviço"}
          </button>
        </div>
      </form>
    </div>
  );
}
