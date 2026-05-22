# FakTest Modern Stack

**Modern implementation of FakTest online examination platform**

Migrated from legacy Yii 1 PHP to **React + Rust (Axum) + PostgreSQL** as per MASC Updated Handoff Document.

## Architecture
- **Backend**: Rust with Axum framework, SQLx for DB, Argon2 auth, JWT
- **Frontend**: React for student and admin portals
- **DB**: PostgreSQL with denormalized `test_attempts` table using JSONB

See `MASC_Updated.md` for full handoff details.

## Quick Start
1. `cargo run` for backend
2. Docker compose for full stack

Full code implements the design doc in detail.