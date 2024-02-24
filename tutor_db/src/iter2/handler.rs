use crate::model::CreateCourseRequest;
use crate::state::AppState;
use ntex::web;
use ntex::web::HttpResponse;
use std::sync::Arc;

#[web::get("/health")]
pub async fn health_check(app_state: web::types::State<Arc<AppState>>) -> HttpResponse {
    let count;
    {
        let mut visit_count = app_state.visit_count.lock().unwrap();
        count = *visit_count + 1;
        *visit_count = count;
    }
    let response = format!("{} {} times", app_state.health_check_response, count);
    HttpResponse::Ok().json(&response)
}

#[web::post("/courses")]
pub async fn create_course(
    app_state: web::types::State<Arc<AppState>>,
    create_course_request: web::types::Json<CreateCourseRequest>,
) -> HttpResponse {
}
