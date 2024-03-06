use sqlx::PgPool;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u64>,
    pub db: PgPool,
}
