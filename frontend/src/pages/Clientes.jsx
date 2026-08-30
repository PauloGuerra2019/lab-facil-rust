import { Pencil, Plus, Search, Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import api from "../api.js";
import Modal from "../components/Modal.jsx";

const CLIENTE_VAZIO = { nome: "", cpf_cnpj: "", telefone: "", email: "", endereco: "", observacoes: "" };

export default function Clientes() {
  const [clientes, setClientes] = useState([]);
  const [busca, setBusca] = useState("");
  const [modalAberto, setModalAberto] = useState(false);
  const [editando, setEditando] = useState(null);
  const [form, setForm] = useState(CLIENTE_VAZIO);
  const [salvando, setSalvando] = useState(false);
  const [erro, setErro] = useState("");

  async function carregar() {
    const { data } = await api.get("/clientes", { params: { busca: busca || undefined } });
    setClientes(data);
  }

  useEffect(() => {
    const timer = setTimeout(carregar, 250);
    return () => clearTimeout(timer);
  }, [busca]);

  function abrirNovo() {
    setEditando(null);
    setForm(CLIENTE_VAZIO);
    setErro("");
    setModalAberto(true);
  }

  function abrirEdicao(cliente) {
    setEditando(cliente);
    setForm({
      nome: cliente.nome,
      cpf_cnpj: cliente.cpf_cnpj || "",
      telefone: cliente.telefone || "",
      email: cliente.email || "",
      endereco: cliente.endereco || "",
      observacoes: cliente.observacoes || "",
    });
    setErro("");
    setModalAberto(true);
  }

  async function salvar(e) {
    e.preventDefault();
    setErro("");
    setSalvando(true);

    const payload = {
      nome: form.nome.trim(),
      cpf_cnpj: form.cpf_cnpj.trim() || undefined,
      telefone: form.telefone.trim() || undefined,
      email: form.email.trim() || undefined,
      endereco: form.endereco.trim() || undefined,
      observacoes: form.observacoes.trim() || undefined,
    };

    try {
      if (editando) {
        await api.put(`/clientes/${editando.id}`, payload);
      } else {
        await api.post("/clientes", payload);
      }
      setModalAberto(false);
      carregar();
    } catch (err) {
      setErro(err.response?.data?.detail || err.message || "Não foi possível salvar o cliente.");
    } finally {
      setSalvando(false);
    }
  }

  async function remover(cliente) {
    if (!confirm(`Desativar o cliente "${cliente.nome}"? O histórico de OS é mantido.`)) return;
    await api.delete(`/clientes/${cliente.id}`);
    carregar();
  }

  return (
    <div className="space-y-4 sm:space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
        <div>
          <h1 className="font-display text-2xl">Clientes</h1>
          <p className="text-sm text-ink/50">Dentistas e clínicas atendidos pelo laboratório</p>
        </div>
        <button onClick={abrirNovo} className="btn-primary self-start sm:self-auto">
          <Plus size={16} />
          Novo Cliente
        </button>
      </div>

      <div className="relative w-full sm:max-w-sm">
        <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink/30" />
        <input
          className="input pl-9"
          placeholder="Buscar por nome ou CPF/CNPJ…"
          value={busca}
          onChange={(e) => setBusca(e.target.value)}
        />
      </div>

      {/* Desktop: tabela */}
      <div className="hidden lg:block card overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-line text-left text-ink/50 text-xs uppercase tracking-wide">
              <th className="px-5 py-3 font-medium">Nome</th>
              <th className="px-5 py-3 font-medium">CPF/CNPJ</th>
              <th className="px-5 py-3 font-medium">Telefone</th>
              <th className="px-5 py-3 font-medium">E-mail</th>
              <th className="px-5 py-3 font-medium w-24"></th>
            </tr>
          </thead>
          <tbody>
            {clientes.map((c) => (
              <tr key={c.id} className="border-b border-line last:border-0 hover:bg-porcelain/60">
                <td className="px-5 py-3 font-medium">{c.nome}</td>
                <td className="px-5 py-3 font-mono text-xs text-ink/60">{c.cpf_cnpj || "-"}</td>
                <td className="px-5 py-3 text-ink/70">{c.telefone || "-"}</td>
                <td className="px-5 py-3 text-ink/70">{c.email || "-"}</td>
                <td className="px-5 py-3">
                  <div className="flex items-center gap-3 justify-end">
                    <button onClick={() => abrirEdicao(c)} className="text-ink/40 hover:text-teal">
                      <Pencil size={15} />
                    </button>
                    <button onClick={() => remover(c)} className="text-ink/40 hover:text-brick">
                      <Trash2 size={15} />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
            {clientes.length === 0 && (
              <tr>
                <td colSpan={5} className="px-5 py-10 text-center text-ink/40">
                  Nenhum cliente encontrado.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      {/* Mobile: cards */}
      <div className="lg:hidden space-y-3">
        {clientes.map((c) => (
          <div key={c.id} className="card p-4">
            <div className="flex items-start justify-between">
              <div className="min-w-0 flex-1">
                <p className="font-medium text-sm truncate">{c.nome}</p>
                {c.cpf_cnpj && (
                  <p className="font-mono text-xs text-ink/50 mt-0.5">{c.cpf_cnpj}</p>
                )}
              </div>
              <div className="flex items-center gap-2 ml-3 shrink-0">
                <button onClick={() => abrirEdicao(c)} className="text-ink/40 hover:text-teal p-1">
                  <Pencil size={15} />
                </button>
                <button onClick={() => remover(c)} className="text-ink/40 hover:text-brick p-1">
                  <Trash2 size={15} />
                </button>
              </div>
            </div>
            <div className="flex flex-wrap gap-x-4 gap-y-1 mt-2 text-xs text-ink/60">
              {c.telefone && <span>📞 {c.telefone}</span>}
              {c.email && <span>✉ {c.email}</span>}
            </div>
          </div>
        ))}
        {clientes.length === 0 && (
          <div className="card p-8 text-center text-ink/40">
            Nenhum cliente encontrado.
          </div>
        )}
      </div>

      {modalAberto && (
        <Modal titulo={editando ? "Editar Cliente" : "Novo Cliente"} onClose={() => setModalAberto(false)}>
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
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label className="label">CPF/CNPJ</label>
                <input
                  className="input"
                  value={form.cpf_cnpj}
                  onChange={(e) => setForm({ ...form, cpf_cnpj: e.target.value })}
                />
              </div>
              <div>
                <label className="label">Telefone</label>
                <input
                  className="input"
                  value={form.telefone}
                  onChange={(e) => setForm({ ...form, telefone: e.target.value })}
                />
              </div>
            </div>
            <div>
              <label className="label">E-mail</label>
              <input
                type="email"
                className="input"
                value={form.email}
                onChange={(e) => setForm({ ...form, email: e.target.value })}
              />
            </div>
            <div>
              <label className="label">Endereço</label>
              <input
                className="input"
                value={form.endereco}
                onChange={(e) => setForm({ ...form, endereco: e.target.value })}
              />
            </div>
            <div>
              <label className="label">Observações</label>
              <textarea
                className="input"
                rows={2}
                value={form.observacoes}
                onChange={(e) => setForm({ ...form, observacoes: e.target.value })}
              />
            </div>

            {erro && <p className="text-sm text-brick">{erro}</p>}

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
