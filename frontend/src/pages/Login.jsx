import { Stethoscope } from "lucide-react";
import { useState } from "react";
import { Navigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext.jsx";

import api from "../api.js";

const cadastroEmail = import.meta.env.VITE_ACESSO_EMAIL || "contato@dadg.com.br";
const ambienteDev = import.meta.env.DEV;

export default function Login() {
  const { login, usuario } = useAuth();
  const [modo, setModo] = useState("login");
  const [email, setEmail] = useState(ambienteDev ? "admin@laboratorio.com" : "");
  const [senha, setSenha] = useState("");
  const [form, setForm] = useState({
    nome: "",
    email: "",
    empresa: "",
    telefone: "",
    mensagem: "",
  });
  const [erro, setErro] = useState("");
  const [sucesso, setSucesso] = useState("");
  const [enviando, setEnviando] = useState(false);

  if (usuario) return <Navigate to="/" replace />;

  async function handleSubmit(e) {
    e.preventDefault();
    setErro("");
    setSucesso("");
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

  async function handleSolicitacao(e) {
    e.preventDefault();
    setErro("");
    setSucesso("");
    setEnviando(true);

    try {
      const payload = {
        nome: form.nome,
        email: form.email,
        empresa: form.empresa || undefined,
        telefone: form.telefone || undefined,
        mensagem: form.mensagem || undefined,
      };

      const { data } = await api.post("/auth/solicitar-acesso", payload);
      setSucesso(data.mensagem || "Solicitação enviada com sucesso.");
      setForm({ nome: "", email: "", empresa: "", telefone: "", mensagem: "" });
    } catch (err) {
      const detalhe = err.response?.data?.detail || "Não foi possível enviar a solicitação.";
      setErro(detalhe);
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

        <form
          onSubmit={modo === "login" ? handleSubmit : handleSolicitacao}
          className="card p-5 sm:p-7 space-y-4"
        >
          <div>
            <h1 className="font-display text-xl mb-1">
              {modo === "login" ? "Entrar" : "Solicitar acesso"}
            </h1>
            <p className="text-sm text-ink/50">
              {modo === "login"
                ? "Acesse o painel do laboratório"
                : "Cadastre sua empresa e aguarde a aprovação"}
            </p>
          </div>

          {modo === "login" ? (
            <>
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
            </>
          ) : (
            <>
              <div>
                <label className="label">Nome completo</label>
                <input
                  required
                  className="input"
                  value={form.nome}
                  onChange={(e) => setForm({ ...form, nome: e.target.value })}
                />
              </div>

              <div>
                <label className="label">E-mail</label>
                <input
                  type="email"
                  required
                  className="input"
                  value={form.email}
                  onChange={(e) => setForm({ ...form, email: e.target.value })}
                />
              </div>

              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <label className="label">Empresa</label>
                  <input
                    className="input"
                    value={form.empresa}
                    onChange={(e) => setForm({ ...form, empresa: e.target.value })}
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
                <label className="label">Mensagem</label>
                <textarea
                  rows={3}
                  className="input"
                  value={form.mensagem}
                  onChange={(e) => setForm({ ...form, mensagem: e.target.value })}
                  placeholder="Conte um pouco sobre seu laboratório ou necessidade de acesso."
                />
              </div>
            </>
          )}

          {erro && <p className="text-sm text-brick">{erro}</p>}
          {sucesso && <p className="text-sm text-sage">{sucesso}</p>}

          <button type="submit" disabled={enviando} className="btn-primary w-full justify-center">
            {enviando ? (modo === "login" ? "Entrando…" : "Enviando…") : modo === "login" ? "Entrar" : "Solicitar acesso"}
          </button>

          <button
            type="button"
            onClick={() => {
              setErro("");
              setSucesso("");
              setModo((atual) => (atual === "login" ? "cadastro" : "login"));
            }}
            className="btn-secondary w-full justify-center"
          >
            {modo === "login" ? "Cadastre-se aqui" : "Voltar para login"}
          </button>

          {modo === "cadastro" && (
            <div className="text-center text-xs text-ink/50">
              Também pode enviar diretamente para <span className="text-teal">{cadastroEmail}</span>
            </div>
          )}
        </form>
      </div>
    </div>
  );
}
