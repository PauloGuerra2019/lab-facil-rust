import { Navigate, Route, Routes } from "react-router-dom";
import Layout from "./components/Layout.jsx";
import { useAuth } from "./context/AuthContext.jsx";
import Clientes from "./pages/Clientes.jsx";
import Dashboard from "./pages/Dashboard.jsx";
import Login from "./pages/Login.jsx";
import OrdemDetalhe from "./pages/OrdemDetalhe.jsx";
import OrdensServico from "./pages/OrdensServico.jsx";
import TiposServico from "./pages/TiposServico.jsx";

function RotaPrivada({ children }) {
  const { usuario, carregando } = useAuth();
  if (carregando) {
    return <div className="min-h-screen flex items-center justify-center text-ink/40">Carregando…</div>;
  }
  if (!usuario) return <Navigate to="/login" replace />;
  return children;
}

export default function App() {
  return (
    <Routes>
      <Route path="/login" element={<Login />} />
      <Route
        path="/"
        element={
          <RotaPrivada>
            <Layout />
          </RotaPrivada>
        }
      >
        <Route index element={<Dashboard />} />
        <Route path="ordens" element={<OrdensServico />} />
        <Route path="ordens/nova" element={<OrdemDetalhe />} />
        <Route path="ordens/:id" element={<OrdemDetalhe />} />
        <Route path="clientes" element={<Clientes />} />
        <Route path="tipos-servico" element={<TiposServico />} />
      </Route>
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}
