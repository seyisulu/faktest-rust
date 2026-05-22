-- Core schema from MASC_Updated.md Section 4
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    matric VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE,
    password_hash TEXT NOT NULL,
    role VARCHAR(20) DEFAULT 'student',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tests (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    duration_minutes INTEGER NOT NULL,
    passing_score DECIMAL(5,2),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Denormalized test_attempts table (key decision)
CREATE TABLE IF NOT EXISTS test_attempts (
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

-- Add other tables as needed (courses, topics, questions, etc.)
