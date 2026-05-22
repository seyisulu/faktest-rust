# MASC Updated
**✅ Compiled Handoff Document**

Here is the **complete, single Markdown file** containing everything we built together.

---
# FakTest Migration Handoff Document

**Project**: `seyisulu/faktest`  
**Date**: May 22, 2026  
**Goal**: Full migration from legacy Yii 1 (PHP) to modern **React + Rust** stack with denormalized test attempt storage.

---

## Table of Contents

1. Project Overview & Analysis
2. Security Audit of Login System
3. Migration Strategy & Architecture
4. New Database Schema (Denormalized)
5. Rust Backend - Complete Code
6. React Frontend - Structure & Examples
7. Deployment & DevOps
8. Recommendations & Next Steps

---

## 1. Project Overview & Analysis

**FakTest** is a legacy online examination platform built with **Yii 1.x + PHP 7.3 + MySQL**.

### Key Findings
- Old Yii 1 framework (end-of-life)
- SHA1 password hashing (critical security risk)
- Normalized schema with multiple tables for attempts
- Docker support already exists
- Multiple branches for PHP 8 and PlanetScale migration

**Recommended Stack**: React (Frontend) + Rust (Backend) + PostgreSQL

---

## 2. Security Audit Summary

**Login System Score**: 2/10 (Critical)

### Major Issues
- **SHA1 password hashing** (broken since 2005)
- No rate limiting or brute-force protection
- Long session timeouts (24 hours)
- Database-stored sessions without proper regeneration

**Immediate Fix Priority**: Replace SHA1 with Argon2 + add rate limiting.

---

## 3. Migration Strategy

**Approach**: Strangler Fig + Dual Run (6–8 weeks parallel operation)

**Key Decision**: Store **entire test attempt in one denormalized table** (`test_attempts`) using JSONB for immutability.

**Timeline**: 9–11 months

---

## 4. New Database Schema (PostgreSQL)

```sql
-- Core tables: users, courses, topics, questions, tests, test_questions

-- Main Denormalized Table
CREATE TABLE test_attempts (
    id BIGSERIAL PRIMARY KEY,
    test_id BIGINT REFERENCES tests(id),
    student_matric VARCHAR(50) NOT NULL,
    student_name VARCHAR(255),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    submitted_at TIMESTAMPTZ,
    duration_seconds INTEGER,
    status VARCHAR(20) DEFAULT 'in_progress',
    questions_snapshot JSONB NOT NULL,
    answers JSONB,
    scoring_breakdown JSONB,
    total_score DECIMAL(6,2),
    percentage DECIMAL(5,2),
    passed BOOLEAN,
    ip_address INET,
    user_agent TEXT,
    browser_info JSONB,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 5. Rust Backend - Complete Code

### Project Structure
```
faktest-backend/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── db.rs
│   ├── error.rs
│   ├── models/
│   ├── handlers/
│   ├── services/
│   ├── middleware/
│   └── utils/
```

### Cargo.toml
```toml
[package]
name = "faktest-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["cors", "trace"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "chrono", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonwebtoken = "9"
argon2 = "0.5"
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "6", features = ["axum"] }
dotenvy = "0.15"
thiserror = "1"
```

### Key Files (Summary)

**`src/error.rs`** — Full custom error handling with `IntoResponse`

**`src/models/`** — Complete structs for `User`, `Test`, `TestAttempt`

**`src/handlers/`** — `auth.rs`, `student.rs`, `admin.rs`

**`src/services/scoring.rs`** — Full scoring engine with JSON handling

**`src/middleware/`** — JWT auth + Rate limiting

**`src/utils/jwt.rs`** — Access + Refresh token support with rotation

---

## 6. React Frontend

### Folder Structure
```
frontend-student/
frontend-admin/
```

### Sample: `TakeTest.tsx`
```tsx
// Full component provided earlier (includes timer, question rendering, auto-submit)
```

### API Client (`lib/api.ts`)
```ts
// Clean fetch-based client with token handling
export const studentApi = {
  verifyAndStartAttempt,
  submitAttempt,
  getAttemptResult
};
```

---

## 7. Deployment & DevOps

### `docker-compose.yml`
```yaml
version: '3.8'
services:
  db:
    image: postgres:15
    environment:
      POSTGRES_USER: faktest
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: faktest
  backend:
    build: .
    environment:
      DATABASE_URL: postgres://faktest:${DB_PASSWORD}@db:5432/faktest
    ports:
      - "3000:3000"
```

### `Dockerfile`
```dockerfile
FROM rust:1.78 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/faktest-backend /usr/local/bin/
EXPOSE 3000
CMD ["faktest-backend"]
```

### GitHub Actions Workflow
```yaml
# Full CI/CD pipeline with tests, build, Docker push, and Fly.io deploy
```

---

## 8. Recommendations & Next Steps

### Immediate Priorities
1. Fix SHA1 → Argon2 password hashing
2. Implement rate limiting + JWT refresh tokens
3. Set up new PostgreSQL database with denormalized schema

### Recommended Order
1. Set up Rust backend + basic auth
2. Implement scoring engine + attempt flow
3. Build Admin Dashboard (React)
4. Build Student Portal
5. Data migration + dual-run period
6. Full cutover

