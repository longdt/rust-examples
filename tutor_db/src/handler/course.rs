use std::sync::Arc;

use ntex::web;
use ntex::web::HttpResponse;

use crate::dbaccess;
use crate::error::EzyTutorError;
use crate::model::course::{CourseResponse, CreateCourseRequest, UpdateCourseRequest};
use crate::state::AppState;

#[web::post("/courses")]
pub async fn create_course(
    app_state: web::types::State<Arc<AppState>>,
    create_course_request: web::types::Json<CreateCourseRequest>,
) -> Result<HttpResponse, EzyTutorError> {
    dbaccess::course::create_course(&app_state.db, create_course_request.into_inner())
        .await
        .map(|course| HttpResponse::Ok().json(&CourseResponse::from(course)))
}

#[web::get("/courses/{tutor_id}")]
pub async fn get_tutor_courses(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<i64>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    let courses = dbaccess::course::get_tutor_courses(&app_state.db, tutor_id).await?;
    let response = courses
        .into_iter()
        .map(CourseResponse::from)
        .collect::<Vec<CourseResponse>>();
    Ok(HttpResponse::Ok().json(&response))
}

#[web::get("/courses/{tutor_id}/{course_id}")]
pub async fn get_course(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<(i64, i64)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let course = dbaccess::course::get_course(&app_state.db, tutor_id, course_id).await?;
    Ok(HttpResponse::Ok().json(&CourseResponse::from(course)))
}

#[web::put("/courses/{tutor_id}/{course_id}")]
pub async fn update_course(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<(i64, i64)>,
    update_course_request: web::types::Json<UpdateCourseRequest>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let course = dbaccess::course::update_course(
        &app_state.db,
        tutor_id,
        course_id,
        update_course_request.into_inner(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(&CourseResponse::from(course)))
}

#[web::delete("/courses/{tutor_id}/{course_id}")]
pub async fn delete_course(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<(i64, i64)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let _ = dbaccess::course::delete_course(&app_state.db, tutor_id, course_id).await?;
    Ok(HttpResponse::Ok().finish())
}
