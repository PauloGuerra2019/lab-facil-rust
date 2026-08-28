const STATUS_CONFIG = {
  recebido: { label: "Recebido", color: "#3D7286", bg: "#3D728618" },
  em_producao: { label: "Em Produção", color: "#C08A2E", bg: "#C08A2E18" },
  controle_qualidade: { label: "Controle de Qualidade", color: "#7A5C4C", bg: "#7A5C4C18" },
  pronto: { label: "Pronto", color: "#4C7A5B", bg: "#4C7A5B18" },
  entregue: { label: "Entregue", color: "#26261F", bg: "#26261F12" },
  cancelado: { label: "Cancelado", color: "#C8553D", bg: "#C8553D18" },
};

export function StatusBadge({ status }) {
  const config = STATUS_CONFIG[status] || STATUS_CONFIG.recebido;
  return (
    <span
      className="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium"
      style={{ color: config.color, backgroundColor: config.bg }}
    >
      <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: config.color }} />
      {config.label}
    </span>
  );
}

const PAGAMENTO_CONFIG = {
  pendente: { label: "Pendente", color: "#C8553D" },
  parcial: { label: "Parcial", color: "#C08A2E" },
  pago: { label: "Pago", color: "#4C7A5B" },
};

export function PagamentoBadge({ status }) {
  const config = PAGAMENTO_CONFIG[status] || PAGAMENTO_CONFIG.pendente;
  return (
    <span className="text-xs font-medium" style={{ color: config.color }}>
      {config.label}
    </span>
  );
}

export const STATUS_OPTIONS = Object.entries(STATUS_CONFIG).map(([value, cfg]) => ({
  value,
  label: cfg.label,
}));
