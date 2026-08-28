import { createContext, useContext, useEffect, useState } from "react";
import api from "../api.js";

const AuthContext = createContext(null);

export function AuthProvider({ children }) {
  const [usuario, setUsuario] = useState(() => {
    const salvo = localStorage.getItem("lab_facil_user");
    return salvo ? JSON.parse(salvo) : null;
  });
  const [carregando, setCarregando] = useState(true);

  useEffect(() => {
    const token = localStorage.getItem("lab_facil_token");
    if (!token) {
      setCarregando(false);
      return;
    }
    api
      .get("/auth/me")
      .then(({ data }) => {
        setUsuario(data);
        localStorage.setItem("lab_facil_user", JSON.stringify(data));
      })
      .catch(() => {
        localStorage.removeItem("lab_facil_token");
        localStorage.removeItem("lab_facil_user");
        setUsuario(null);
      })
      .finally(() => setCarregando(false));
  }, []);

  async function login(email, senha) {
    const { data } = await api.post("/auth/login", { email: email.trim(), senha });
    localStorage.setItem("lab_facil_token", data.access_token);
    localStorage.setItem("lab_facil_user", JSON.stringify(data.usuario));
    setUsuario(data.usuario);
  }

  function logout() {
    localStorage.removeItem("lab_facil_token");
    localStorage.removeItem("lab_facil_user");
    setUsuario(null);
  }

  return (
    <AuthContext.Provider value={{ usuario, login, logout, carregando }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  return useContext(AuthContext);
}
