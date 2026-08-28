import { AlertTriangle, ClipboardList, TrendingUp, Wallet } from "lucide-react";
import { useEffect, useState } from "react";
import { Bar, BarChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";
import { Link } from "react-router-dom";
import api from "../api.js";
import { STATUS_OPTIONS } from "../components/StatusBadge.jsx";

const formatoMoeda = (v) =>
  new Intl.NumberFormat("pt-BR", { style: "currency", currency: "BRL" }).format(v || 0);

function CardStat({ icone: Icone, label, valor, destaque }) {
  return (
    <div className="card p-5">
      <div className="flex items-center gap-2 text-ink/50 text-xs uppercase tracking-wide mb-3">
        <Icone size={14} />
        {label}
      </div>
      <p className={`font-display text-3xl ${destaque ? "text-brick" : "text-ink"}`}>{valor}</p>
    </div>
  );
}

export default function Dashboard() {
  const [stats, setStats] = useState(null);

  useEffect(() => {
    api.get("/dashboard").then(({ data }) => setStats(data));
  }, []);

  if (!stats) {
    return <p className="text-ink/40">Carregando painel…</p>;
  }

  const dadosGrafico = stats.faturamento_ultimos_6_meses.map((m) => ({
    mes: new Date(m.mes + "-02").toLocaleDateString("pt-BR", { month: "short" }),
    total: m.total,
  }));

  return (
    <div className="space-y-8">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="font-display text-2xl">Painel</h1>
          <p className="text-sm text-ink/50">Visão geral do laboratório</p>
        </div>
        <Link to="/ordens/nova" className="btn-primary">
          Nova Ordem de Serviço
        </Link>
      </div>

      <div className="grid grid-cols-4 gap-4">
        <CardStat icone={ClipboardList} label="OS em aberto" valor={stats.total_os_abertas} />
        <CardStat
          icone={AlertTriangle}
          label="OS atrasadas"
          valor={stats.total_os_atrasadas}
          destaque={stats.total_os_atrasadas > 0}
        />
        <CardStat icone={TrendingUp} label="Faturamento do mês" valor={formatoMoeda(stats.faturamento_mes)} />
        <CardStat icone={Wallet} label="A receber" valor={formatoMoeda(stats.a_receber)} />
      </div>

      <div className="grid grid-cols-3 gap-6">
        <div className="card p-6 col-span-2">
          <h2 className="font-display text-lg mb-4">Faturamento — últimos 6 meses</h2>
          <ResponsiveContainer width="100%" height={240}>
            <BarChart data={dadosGrafico}>
              <CartesianGrid strokeDasharray="3 3" stroke="#E4DFD3" vertical={false} />
              <XAxis dataKey="mes" tick={{ fontSize: 12, fill: "#26261F99" }} axisLine={false} tickLine={false} />
              <YAxis tick={{ fontSize: 12, fill: "#26261F99" }} axisLine={false} tickLine={false} />
              <Tooltip formatter={(v) => formatoMoeda(v)} />
              <Bar dataKey="total" fill="#1F4E5F" radius={[4, 4, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </div>

        <div className="card p-6">
          <h2 className="font-display text-lg mb-4">OS por status</h2>
          <div className="space-y-3">
            {STATUS_OPTIONS.filter((s) => s.value !== "cancelado").map((s) => (
              <div key={s.value} className="flex items-center justify-between text-sm">
                <span className="text-ink/70">{s.label}</span>
                <span className="font-mono font-medium">{stats.os_por_status[s.value] || 0}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
