import { AlertTriangle, Plus, Search, Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import api from "../api.js";
import { PagamentoBadge, STATUS_OPTIONS, StatusBadge } from "../components/StatusBadge.jsx";

const formatoMoeda = (v) => new Intl.NumberFormat("pt-BR", { style: "currency", currency: "BRL" }).format(v || 0);
const formatoData = (d) => (d ? new Date(d + "T00:00:00").toLocaleDateString("pt-BR") : "-");

export default function OrdensServico() {
  const [ordens, setOrdens] = useState([]);
  const [busca, setBusca] = useState("");
  const [status, setStatus] = useState("");
  const [apenasAtrasadas, setApenasAtrasadas] = useState(false);

  async function carregar() {
    const { data } = await api.get("/ordens-servico", {
      params: { busca: busca || undefined, status: status || undefined, atrasadas: apenasAtrasadas || undefined },
    });
    setOrdens(data);
  }

  async function excluirOrdem(o, e) {
    e.preventDefault();
    e.stopPropagation();
    if (confirm(`Tem certeza que deseja excluir permanentemente a OS #${String(o.numero).padStart(5, "0")}?`)) {
      await api.delete(`/ordens-servico/${o.id}?permanente=true`);
      carregar();
    }
  }

  useEffect(() => {
    const timer = setTimeout(carregar, 250);
    return () => clearTimeout(timer);
  }, [busca, status, apenasAtrasadas]);

  const hoje = new Date().toISOString().slice(0, 10);

  return (
    <div className="space-y-4 sm:space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
        <div>
          <h1 className="font-display text-2xl">Ordens de Serviço</h1>
          <p className="text-sm text-ink/50">Controle de entrada e saída dos trabalhos do laboratório</p>
        </div>
        <Link to="/ordens/nova" className="btn-primary self-start sm:self-auto">
          <Plus size={16} />
          Nova OS
        </Link>
      </div>

      <div className="flex flex-col sm:flex-row sm:items-center gap-3 sm:flex-wrap">
        <div className="relative w-full sm:w-64">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink/30" />
          <input
            className="input pl-9"
            placeholder="Buscar por paciente ou cliente…"
            value={busca}
            onChange={(e) => setBusca(e.target.value)}
          />
        </div>
        <select className="input w-full sm:w-52" value={status} onChange={(e) => setStatus(e.target.value)}>
          <option value="">Todos os status</option>
          {STATUS_OPTIONS.map((s) => (
            <option key={s.value} value={s.value}>
              {s.label}
            </option>
          ))}
        </select>
        <button
          onClick={() => setApenasAtrasadas((v) => !v)}
          className={apenasAtrasadas ? "btn-danger" : "btn-secondary"}
        >
          <AlertTriangle size={15} />
          Somente atrasadas
        </button>
      </div>

      {/* Desktop: tabela */}
      <div className="hidden lg:block card overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-line text-left text-ink/50 text-xs uppercase tracking-wide">
              <th className="px-5 py-3 font-medium">OS</th>
              <th className="px-5 py-3 font-medium">Cliente</th>
              <th className="px-5 py-3 font-medium">Paciente</th>
              <th className="px-5 py-3 font-medium">Entrada</th>
              <th className="px-5 py-3 font-medium">Previsão</th>
              <th className="px-5 py-3 font-medium">Status</th>
              <th className="px-5 py-3 font-medium">Pagamento</th>
              <th className="px-5 py-3 font-medium text-right">Valor</th>
              <th className="px-5 py-3 font-medium w-12"></th>
            </tr>
          </thead>
          <tbody>
            {ordens.map((o) => {
              const atrasada =
                o.data_prevista &&
                o.data_prevista < hoje &&
                !["entregue", "cancelado"].includes(o.status);
              return (
                <tr key={o.id} className="border-b border-line last:border-0 hover:bg-porcelain/60">
                  <td className="px-5 py-3">
                    <Link to={`/ordens/${o.id}`} className="font-mono font-medium text-teal hover:underline">
                      #{String(o.numero).padStart(5, "0")}
                    </Link>
                  </td>
                  <td className="px-5 py-3">{o.cliente.nome}</td>
                  <td className="px-5 py-3 text-ink/70">{o.paciente_nome || "-"}</td>
                  <td className="px-5 py-3 text-ink/70">{formatoData(o.data_entrada)}</td>
                  <td className={`px-5 py-3 ${atrasada ? "text-brick font-medium" : "text-ink/70"}`}>
                    {formatoData(o.data_prevista)}
                    {atrasada && " ⚠"}
                  </td>
                  <td className="px-5 py-3">
                    <StatusBadge status={o.status} />
                  </td>
                  <td className="px-5 py-3">
                    <PagamentoBadge status={o.status_pagamento} />
                  </td>
                  <td className="px-5 py-3 text-right font-mono">{formatoMoeda(o.valor_total)}</td>
                  <td className="px-5 py-3 text-right">
                    <button
                      onClick={(e) => excluirOrdem(o, e)}
                      title="Excluir OS"
                      className="text-ink/30 hover:text-brick transition-colors"
                    >
                      <Trash2 size={16} />
                    </button>
                  </td>
                </tr>
              );
            })}
            {ordens.length === 0 && (
              <tr>
                <td colSpan={9} className="px-5 py-10 text-center text-ink/40">
                  Nenhuma ordem de serviço encontrada.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      {/* Mobile: cards */}
      <div className="lg:hidden space-y-3">
        {ordens.map((o) => {
          const atrasada =
            o.data_prevista &&
            o.data_prevista < hoje &&
            !["entregue", "cancelado"].includes(o.status);
          return (
            <Link key={o.id} to={`/ordens/${o.id}`} className="card block p-4 hover:shadow-md transition-shadow">
              <div className="flex items-start justify-between mb-2">
                <div>
                  <span className="font-mono font-medium text-teal text-sm">
                    #{String(o.numero).padStart(5, "0")}
                  </span>
                  <p className="font-medium text-sm mt-0.5">{o.cliente.nome}</p>
                </div>
                <div className="text-right">
                  <p className="font-mono font-medium text-sm">{formatoMoeda(o.valor_total)}</p>
                  <PagamentoBadge status={o.status_pagamento} />
                </div>
              </div>

              {o.paciente_nome && (
                <p className="text-xs text-ink/60 mb-2">Paciente: {o.paciente_nome}</p>
              )}

              <div className="flex items-center justify-between mt-2">
                <StatusBadge status={o.status} />
                <div className="flex items-center gap-3 text-xs text-ink/50">
                  <span>{formatoData(o.data_entrada)}</span>
                  {o.data_prevista && (
                    <span className={atrasada ? "text-brick font-medium" : ""}>
                      → {formatoData(o.data_prevista)}
                      {atrasada && " ⚠"}
                    </span>
                  )}
                </div>
              </div>

              <div className="flex justify-end mt-2 pt-2 border-t border-line">
                <button
                  onClick={(e) => excluirOrdem(o, e)}
                  title="Excluir OS"
                  className="text-ink/30 hover:text-brick transition-colors p-1"
                >
                  <Trash2 size={15} />
                </button>
              </div>
            </Link>
          );
        })}
        {ordens.length === 0 && (
          <div className="card p-8 text-center text-ink/40">
            Nenhuma ordem de serviço encontrada.
          </div>
        )}
      </div>
    </div>
  );
}
