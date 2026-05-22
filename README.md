# FakTest Modern Stack

**Modern implementation of FakTest online examination platform**

This repo is the complete migration from legacy **Yii 1 (PHP)** to a modern **React + Rust (Axum) + PostgreSQL** stack, following the MASC_Updated.md design document.

## Project Status
- ✅ Rust backend scaffold with full structure
- ✅ Argon2 authentication
- ✅ Denormalized `test_attempts` models + scoring engine
- ✅ Student and Admin React frontends (Vite + TS)
- ✅ Docker + docker-compose support

## Local Development Setup

### 1. Clone the repository
```bash
git clone https://github.com/seyisulu/faktest-rust.git
cd faktest-rust
```

### 2. Backend (Rust)

1. Copy environment variables:
   ```bash
   cp .env.example .env
   ```
   Edit `.env` and set:
   - `DATABASE_URL=postgres://faktest:password@localhost:5432/faktest`
   - `JWT_SECRET=your-super-secret-jwt-key-here`

2. Start the full stack with Docker (recommended):
   ```bash
   docker compose up --build
   ```
   Backend will be available at `http://localhost:3000`

3. Or run backend natively (requires Rust and Postgres running):
   ```bash
   cargo run
   ```

### 3. Frontend

#### Student Portal
```bash
cd frontend-student
npm install
npm run dev
```
→ Opens at `http://localhost:5173`

#### Admin Dashboard
```bash
cd frontend-admin
npm install
npm run dev
```
→ Opens at `http://localhost:5174`

### 4. Database
The `docker-compose.yml` includes PostgreSQL. After starting, you can run migrations (once implemented):
```bash
cd faktest-rust/backend  # or wherever migrations are
cargo sqlx migrate run
```

## API Documentation
Once backend is running, visit `http://localhost:3000/swagger-ui` for OpenAPI docs.

## Tech Stack
- **Backend**: Rust, Axum, SQLx, Argon2, JWT
- **Frontend**: React, TypeScript, Vite, Tailwind
- **Database**: PostgreSQL with JSONB
- **Auth**: Argon2 + JWT (access + refresh tokens)

See `MASC_Updated.md` for complete architecture, schema, and migration strategy.

## Next Steps
- Database migrations
- Full end-to-end test flow
- CI/CD pipeline
- Data migration from legacy MySQL

---

**Legacy FakTest** is being replaced via **Strangler Fig** pattern. This is the new canonical implementation.