import { Stethoscope } from "lucide-react";
import { useState } from "react";
import { Navigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext.jsx";

const ambienteDev = import.meta.env.DEV;

export default function Login() {
  const { login, usuario } = useAuth();
  const [email, setEmail] = useState(ambienteDev ? "admin@laboratorio.com" : "");
  const [senha, setSenha] = useState("");
  const [erro, setErro] = useState("");
  const [enviando, setEnviando] = useState(false);

  if (usuario) return <Navigate to="/" replace />;

  async function handleSubmit(e) {
    e.preventDefault();
    setErro("");
    setEnviando(true);
    try {
      await login(email, senha);
    } catch (err) {
      if (err.response?.status === 401) {
        setErro("E-mail ou senha incorretos.");
      } else {
        setErro("Não foi possível conectar ao servidor. Verifique a internet e tente novamente.");
      }
    } finally {
      setEnviando(false);
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-teal-dark px-4">
      <div className="w-full max-w-md">
        <div className="flex items-center gap-2 justify-center mb-6 sm:mb-8 text-white">
          <Stethoscope size={26} />
          <span className="font-display text-lg sm:text-xl text-center">DADG - Laboratório de prótese dentária</span>
        </div>

        <form onSubmit={handleSubmit} className="card p-5 sm:p-7 space-y-4">
          <div>
            <h1 className="font-display text-xl mb-1">Entrar</h1>
            <p className="text-sm text-ink/50">Acesse o painel do laboratório</p>
          </div>

          <div>
            <label className="label">E-mail</label>
            <input
              type="email"
              required
              autoComplete="email"
              className="input"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </div>

          <div>
            <label className="label">Senha</label>
            <input
              type="password"
              required
              autoComplete="current-password"
              className="input"
              value={senha}
              onChange={(e) => setSenha(e.target.value)}
              placeholder={ambienteDev ? "admin123 (usuário padrão inicial)" : ""}
            />
          </div>

          {erro && <p className="text-sm text-brick">{erro}</p>}

          <button type="submit" disabled={enviando} className="btn-primary w-full justify-center">
            {enviando ? "Entrando…" : "Entrar"}
          </button>
        </form>
      </div>
    </div>
  );
}
