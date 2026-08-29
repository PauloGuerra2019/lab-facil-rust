import {
  ClipboardList,
  LayoutDashboard,
  ListTree,
  LogOut,
  Stethoscope,
  Users,
} from "lucide-react";
import { NavLink, Outlet } from "react-router-dom";
import { useAuth } from "../context/AuthContext.jsx";

const NAV_ITEMS = [
  { to: "/", label: "Painel", icon: LayoutDashboard, end: true },
  { to: "/ordens", label: "Ordens de Serviço", icon: ClipboardList },
  { to: "/clientes", label: "Clientes", icon: Users },
  { to: "/tipos-servico", label: "Tipos de Serviço", icon: ListTree },
];

export default function Layout() {
  const { usuario, logout } = useAuth();

  return (
    <div className="min-h-screen min-w-[1200px] flex bg-porcelain">
      <aside className="w-64 shrink-0 bg-teal-dark text-white flex flex-col">
        <div className="flex items-center gap-2 px-6 py-6 border-b border-white/10">
          <Stethoscope size={22} className="text-white/80" />
          <div>
            <p className="font-display text-lg leading-none">DADG</p>
            <p className="text-[11px] text-white/50 mt-1 tracking-wide uppercase">
              Prótese Dentária
            </p>
          </div>
        </div>

        <nav className="flex-1 px-3 py-6 space-y-1">
          {NAV_ITEMS.map(({ to, label, icon: Icon, end }) => (
            <NavLink
              key={to}
              to={to}
              end={end}
              className={({ isActive }) =>
                `flex items-center gap-3 px-3 py-2.5 rounded-md text-sm transition-colors ${
                  isActive
                    ? "bg-white/10 text-white font-medium"
                    : "text-white/70 hover:bg-white/5 hover:text-white"
                }`
              }
            >
              <Icon size={17} />
              {label}
            </NavLink>
          ))}
        </nav>

        <div className="px-4 py-4 border-t border-white/10">
          <p className="text-sm text-white/90 truncate">{usuario?.nome}</p>
          <p className="text-xs text-white/40 truncate mb-3">{usuario?.email}</p>
          <button
            onClick={logout}
            className="flex items-center gap-2 text-xs text-white/60 hover:text-white transition-colors"
          >
            <LogOut size={14} />
            Sair
          </button>
        </div>
      </aside>

      <main className="flex-1 bg-porcelain min-h-screen overflow-y-auto">
        <div className="max-w-[1400px] mx-auto px-8 py-8">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
