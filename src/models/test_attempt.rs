use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TestAttempt {
    pub id: i64,
    pub test_id: i64,
    pub student_matric: String,
    pub student_name: Option<String>,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub questions_snapshot: Json<serde_json::Value>,
    pub answers: Option<Json<serde_json::Value>>,
    pub total_score: Option<f64>,
    pub percentage: Option<f64>,
    pub passed: Option<bool>,
}
