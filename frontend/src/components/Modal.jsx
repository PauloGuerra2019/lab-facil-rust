import { X } from "lucide-react";

export default function Modal({ titulo, onClose, children, largura = "max-w-lg" }) {
  return (
    <div className="fixed inset-0 bg-ink/40 flex items-start justify-center p-3 sm:p-6 overflow-y-auto z-50">
      <div className={`card w-full ${largura} mt-4 sm:mt-12 shadow-xl`}>
        <div className="flex items-center justify-between px-4 sm:px-6 py-3 sm:py-4 border-b border-line">
          <h2 className="text-lg font-display">{titulo}</h2>
          <button
            onClick={onClose}
            className="text-ink/40 hover:text-ink transition-colors"
            aria-label="Fechar"
          >
            <X size={18} />
          </button>
        </div>
        <div className="p-4 sm:p-6">{children}</div>
      </div>
    </div>
  );
}
