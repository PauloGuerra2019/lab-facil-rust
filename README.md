# DADG — Laboratório de Prótese Dentária (Rust + React)

Sistema de gestão completo para laboratórios de prótese dentária com backend de alta performance em **Rust (Axum)** e frontend em **React (Vite + Tailwind)**.

## 🚀 Tecnologias

- **Backend (Rust):** Axum (Web), SQLx (SQLite Async), Argon2 (Hash de Senhas), JWT, Printpdf (Geração de Recibos em PDF).
- **Frontend (React):** React 18, Vite, Axios, Lucide React icons, TailwindCSS.
- **Infraestrutura:** Docker & Docker Compose (Imagens ultraleves compiladas).

---

## 🔑 Credenciais Iniciais de Acesso

- **E-mail:** `admin@laboratorio.com`
- **Senha:** `admin123`

---

## 🛠️ Como Executar o Projeto

### Opção 1: Via Docker (Recomendado)

Rode o comando na raiz do repositório:

```bash
docker compose up --build
```

- **Frontend:** `http://localhost:5173`
- **Backend API:** `http://localhost:8000`

---

### Opção 2: Execução Manual para Desenvolvimento

#### Backend (Rust):
```bash
cd backend
cargo run
```
*API iniciada em `http://localhost:8000`.*

#### Frontend (React):
```bash
cd frontend
npm install
npm run dev
```
*Interface iniciada em `http://localhost:5173`.*

---

## 📱 Acesso via Celular (Rede Wi-Fi Local)

Para acessar o sistema de qualquer celular ou tablet conectado na mesma rede Wi-Fi do computador:

1. Obtenha o IP local do seu computador (ex: via `ipconfig` no Windows).
2. Abra o navegador do celular e acesse:
   ```text
   http://<SEU_IP_LOCAL>:5173
   ```
   *(Exemplo: `http://192.168.1.231:5173`)*

---

## ✨ Funcionalidades Principais

1. **Gestão de Ordens de Serviço (OS):**
   - Cadastro completo de trabalhos com cliente (dentista/clínica), paciente, cor/escala e itens discriminados.
   - Status de produção (Pendente, Em Produção, Pronto, Entregue, Cancelado) e status de pagamento.
   - Botão para **Exclusão Permanente** de OS em caso de erros de digitação.
   - Geração e download imediato do **Recibo em PDF** com dados do laboratório e tabela financeira.

2. **Gestão de Clientes:**
   - Cadastro de dentistas e clínicas com busca rápida por nome, CPF/CNPJ, telefone e e-mail.

3. **Tipos de Serviço:**
   - Tabela de valores padrão e prazos em dias para cada tipo de prótese/trabalho.

4. **Painel / Dashboard:**
   - Resumo com métricas do mês, faturamento total, OS pendentes e gráfico de serviços mais solicitados.

---

## ⚙️ Variáveis de Ambiente (`.env`)

Exemplo de configuração no arquivo `backend/.env`:

```env
DATABASE_URL=postgresql://postgres:SENHA@db.PROJECT_REF.supabase.co:5432/postgres
JWT_SECRET=lab-facil-rust-chave-secreta-desenvolvimento-2026
CORS_ORIGIN=http://localhost:5173
LAB_NOME=DADG - Laboratório de prótese dentária
LAB_CNPJ=00.000.000/0001-00
LAB_ENDERECO=Rua Exemplo, 123 - Votorantim/SP
LAB_TELEFONE=(15) 90000-0000
```

## ☁️ Deploy gratuito

O banco de produção usa **Supabase (PostgreSQL)**, o backend pode ser publicado no **Render** e o frontend na **Vercel**.

### Supabase

1. No painel do projeto, abra **Connect** e copie a URI PostgreSQL usando o modo **Session pooler** quando disponível.
2. Execute as migrations do diretório `backend/migrations` no **SQL Editor** do Supabase, na ordem `001_initial.sql` e `002_nfse.sql`.

### Render

Crie um **Web Service** conectado ao repositório, selecione **Docker** e use `backend/Dockerfile` como Dockerfile. Configure:

```env
DATABASE_URL=<URI PostgreSQL do Supabase>
JWT_SECRET=<chave longa e aleatória>
CORS_ORIGIN=https://<seu-projeto>.vercel.app
LAB_NOME=DADG - Laboratório de prótese dentária
LAB_CNPJ=64.329.994/0001-77 
LAB_ENDERECO=Rua Carlos Luvison, 376 - Parque Bela Vista - Votorantim/SP - CEP 18110-435
LAB_TELEFONE=(15) 99719-7692
```

O servidor usa automaticamente a variável `PORT` fornecida pelo Render.

### Vercel

Importe o diretório `frontend` como projeto Vite e configure:

```env
VITE_API_URL=https://<seu-backend>.onrender.com
```

O arquivo `frontend/vercel.json` mantém as rotas do React funcionando após atualização da página.
