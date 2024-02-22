use std::sync::Arc;
use ntex::web;
use ntex::web::HttpResponse;
use time::OffsetDateTime;
use crate::model::{Course, CreateCourseRequest};
use crate::state::AppState;

#[web::get("/health")]
pub async fn health_check(app_state: web::types::State<Arc<AppState>>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let count;
    {
        let mut visit_count = app_state.visit_count.lock().unwrap();
        count = *visit_count + 1;
        *visit_count = count;
    };
    let response = format!("{} {} times", health_check_response, count);
    HttpResponse::Ok().body(&response)
}

pub async fn create_course(create_course_request: web::types::Json<CreateCourseRequest>,
    app_state: web::types::State<Arc<AppState>>) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state.courses.lock().unwrap()
        .iter()
        .filter(|course| course.tutor_id == create_course_request.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: create_course_request.tutor_id,
        course_id: Some((course_count_for_user + 1) as i64),
        course_name: create_course_request.course_name.clone(),
        created_at: Some(OffsetDateTime::now_local().unwrap())
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json(&"Added course")
}
