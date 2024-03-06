use std::sync::Arc;
use ntex::web;
use ntex::web::HttpResponse;
use tracing::{info, instrument};
use crate::state::AppState;

#[instrument]
#[web::get("/health")]
pub async fn health_check(app_state: web::types::State<Arc<AppState>>) -> HttpResponse {
    let count;
    {
        let mut visit_count = app_state.visit_count.lock().unwrap();
        count = *visit_count + 1;
        *visit_count = count;
    }
    info!("increase visit count");
    let response = format!("{} {} times", app_state.health_check_response, count);
    HttpResponse::Ok().json(&response)
}
