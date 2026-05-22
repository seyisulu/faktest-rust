# FakTest - Modernized Stack

**Modern replacement for the legacy Yii 1 FakTest examination platform**

## Tech Stack
- **Backend**: Rust (Axum) + SQLx + PostgreSQL
- **Frontend**: React + TypeScript + Vite (Student + Admin portals)
- **Database**: PostgreSQL with denormalized `test_attempts` (JSONB)
- **Auth**: Argon2id + JWT (Access + Refresh tokens)

## Quick Start - Run Locally

### Prerequisites
- Docker & Docker Compose (recommended)
- Node.js 20+ and npm (for frontend development)
- Rust toolchain (for backend development)

### 1. Clone the repository
```bash
git clone https://github.com/seyisulu/faktest-rust.git
cd faktest-rust
```

### 2. Environment Setup
```bash
cp .env.example .env
# Edit .env and set:
# - DATABASE_URL
# - JWT_SECRET (strong random string)
```

### 3. Run with Docker Compose (Recommended - Backend + DB)
```bash
docker compose up --build
```

Backend will be available at: **http://localhost:3000**

### 4. Run Frontends
In separate terminals:

**Student Portal**
```bash
cd frontend-student
npm install
npm run dev
```
→ Opens at http://localhost:5173

**Admin Dashboard**
```bash
cd frontend-admin
npm install
npm run dev
```
→ Opens at http://localhost:5174

### Alternative: Backend without Docker
```bash
# Start PostgreSQL separately (or use local instance)
cargo run
```

## Project Structure
- `faktest-backend/` - Rust Axum backend (main application)
- `frontend-student/` - Student test-taking interface
- `frontend-admin/` - Administration dashboard
- `MASC_Updated.md` - Complete migration handoff document

## API Documentation
Once backend is running, visit `/swagger-ui` for OpenAPI/Swagger docs.

## Next Steps
See `MASC_Updated.md` for full migration plan and Section 8 priorities.

**Legacy Yii 1 application is being replaced via Strangler Fig pattern.**

Built as part of the FakTest modernization project.
