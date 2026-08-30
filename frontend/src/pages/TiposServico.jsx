import { Pencil, Plus, Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import api from "../api.js";
import Modal from "../components/Modal.jsx";

const VAZIO = { nome: "", categoria: "", valor_padrao: "", prazo_dias: 5 };
const formatoMoeda = (v) => new Intl.NumberFormat("pt-BR", { style: "currency", currency: "BRL" }).format(v || 0);

export default function TiposServico() {
  const [tipos, setTipos] = useState([]);
  const [modalAberto, setModalAberto] = useState(false);
  const [editando, setEditando] = useState(null);
  const [form, setForm] = useState(VAZIO);
  const [salvando, setSalvando] = useState(false);

  async function carregar() {
    const { data } = await api.get("/tipos-servico");
    setTipos(data);
  }

  useEffect(() => {
    carregar();
  }, []);

  function abrirNovo() {
    setEditando(null);
    setForm(VAZIO);
    setModalAberto(true);
  }

  function abrirEdicao(tipo) {
    setEditando(tipo);
    setForm({
      nome: tipo.nome,
      categoria: tipo.categoria || "",
      valor_padrao: tipo.valor_padrao,
      prazo_dias: tipo.prazo_dias,
    });
    setModalAberto(true);
  }

  async function salvar(e) {
    e.preventDefault();
    setSalvando(true);
    const payload = { ...form, valor_padrao: Number(form.valor_padrao), prazo_dias: Number(form.prazo_dias) };
    try {
      if (editando) {
        await api.put(`/tipos-servico/${editando.id}`, payload);
      } else {
        await api.post("/tipos-servico", payload);
      }
      setModalAberto(false);
      carregar();
    } finally {
      setSalvando(false);
    }
  }

  async function remover(tipo) {
    if (!confirm(`Desativar o serviço "${tipo.nome}"?`)) return;
    await api.delete(`/tipos-servico/${tipo.id}`);
    carregar();
  }

  const porCategoria = tipos.reduce((acc, t) => {
    const cat = t.categoria || "Outros";
    (acc[cat] ||= []).push(t);
    return acc;
  }, {});

  return (
    <div className="space-y-4 sm:space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
        <div>
          <h1 className="font-display text-2xl">Tipos de Serviço</h1>
          <p className="text-sm text-ink/50">Catálogo de serviços e valores padrão do laboratório</p>
        </div>
        <button onClick={abrirNovo} className="btn-primary self-start sm:self-auto">
          <Plus size={16} />
          Novo Serviço
        </button>
      </div>

      <div className="space-y-4 sm:space-y-6">
        {Object.entries(porCategoria).map(([categoria, itens]) => (
          <div key={categoria} className="card overflow-hidden">
            <div className="px-4 sm:px-5 py-3 border-b border-line bg-porcelain/60 flex items-center justify-between">
              <h3 className="text-xs font-semibold uppercase tracking-wider text-teal-dark">{categoria}</h3>
              <span className="text-xs text-ink/40 font-mono">{itens.length} {itens.length === 1 ? "item" : "itens"}</span>
            </div>

            {/* Desktop: tabela com colunas alinhadas à esquerda */}
            <table className="hidden sm:table w-full text-sm text-left">
              <thead>
                <tr className="border-b border-line text-xs uppercase tracking-wider text-ink/40 bg-porcelain/30">
                  <th className="px-5 py-2.5 font-medium text-left">Serviço</th>
                  <th className="px-5 py-2.5 font-medium text-left w-48">Prazo</th>
                  <th className="px-5 py-2.5 font-medium text-right w-40">Valor Padrão</th>
                  <th className="px-5 py-2.5 font-medium text-right w-24">Ações</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-line">
                {itens.map((t) => (
                  <tr key={t.id} className="hover:bg-porcelain/40 transition-colors">
                    <td className="px-5 py-3.5 font-medium text-ink text-left">{t.nome}</td>
                    <td className="px-5 py-3.5 text-ink/70 text-left">{t.prazo_dias} dias de prazo</td>
                    <td className="px-5 py-3.5 font-mono text-right font-medium text-ink">{formatoMoeda(t.valor_padrao)}</td>
                    <td className="px-5 py-3.5 text-right">
                      <div className="flex items-center gap-3 justify-end">
                        <button onClick={() => abrirEdicao(t)} title="Editar" className="text-ink/40 hover:text-teal transition-colors p-1">
                          <Pencil size={15} />
                        </button>
                        <button onClick={() => remover(t)} title="Excluir" className="text-ink/40 hover:text-brick transition-colors p-1">
                          <Trash2 size={15} />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            {/* Mobile: lista alinhada à esquerda */}
            <div className="sm:hidden divide-y divide-line">
              {itens.map((t) => (
                <div key={t.id} className="px-4 py-3 text-left">
                  <div className="flex items-start justify-between">
                    <div className="min-w-0 flex-1 text-left">
                      <p className="font-medium text-sm text-ink">{t.nome}</p>
                      <div className="flex items-center gap-3 mt-1 text-xs text-ink/60">
                        <span>⏱ {t.prazo_dias} dias de prazo</span>
                        <span className="font-mono font-medium text-teal">{formatoMoeda(t.valor_padrao)}</span>
                      </div>
                    </div>
                    <div className="flex items-center gap-2 ml-3 shrink-0">
                      <button onClick={() => abrirEdicao(t)} className="text-ink/40 hover:text-teal p-1.5 rounded-md hover:bg-porcelain">
                        <Pencil size={15} />
                      </button>
                      <button onClick={() => remover(t)} className="text-ink/40 hover:text-brick p-1.5 rounded-md hover:bg-porcelain">
                        <Trash2 size={15} />
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        ))}
        {tipos.length === 0 && <p className="text-ink/40 text-center py-10">Nenhum tipo de serviço cadastrado.</p>}
      </div>

      {modalAberto && (
        <Modal titulo={editando ? "Editar Serviço" : "Novo Serviço"} onClose={() => setModalAberto(false)}>
          <form onSubmit={salvar} className="space-y-4">
            <div>
              <label className="label">Nome *</label>
              <input
                required
                className="input"
                value={form.nome}
                onChange={(e) => setForm({ ...form, nome: e.target.value })}
              />
            </div>
            <div>
              <label className="label">Categoria</label>
              <input
                className="input"
                placeholder="Ex: Prótese Fixa, Prótese Removível, Ortodontia…"
                value={form.categoria}
                onChange={(e) => setForm({ ...form, categoria: e.target.value })}
              />
            </div>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label className="label">Valor padrão (R$) *</label>
                <input
                  required
                  type="number"
                  step="0.01"
                  min="0"
                  className="input"
                  value={form.valor_padrao}
                  onChange={(e) => setForm({ ...form, valor_padrao: e.target.value })}
                />
              </div>
              <div>
                <label className="label">Prazo padrão (dias)</label>
                <input
                  type="number"
                  min="0"
                  className="input"
                  value={form.prazo_dias}
                  onChange={(e) => setForm({ ...form, prazo_dias: e.target.value })}
                />
              </div>
            </div>
            <div className="flex flex-col-reverse sm:flex-row justify-end gap-3 pt-2">
              <button type="button" onClick={() => setModalAberto(false)} className="btn-secondary">
                Cancelar
              </button>
              <button type="submit" disabled={salvando} className="btn-primary">
                {salvando ? "Salvando…" : "Salvar"}
              </button>
            </div>
          </form>
        </Modal>
      )}
    </div>
  );
}
