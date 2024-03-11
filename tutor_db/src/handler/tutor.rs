use crate::error::EzyTutorError;
use crate::model::tutor::{CreateTutorRequest, TutorResponse, UpdateTutorRequest};
use crate::state::AppState;
use crate::*;
use ntex::web;
use ntex::web::HttpResponse;
use std::sync::Arc;

#[web::post("/tutors")]
pub async fn create_tutor(
    app_state: web::types::State<Arc<AppState>>,
    create_tutor_request: web::types::Json<CreateTutorRequest>,
) -> Result<HttpResponse, EzyTutorError> {
    dbaccess::tutor::create_tutor(&app_state.db, create_tutor_request.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(&TutorResponse::from(tutor)))
}

#[web::get("/tutors")]
pub async fn get_tutors(
    app_state: web::types::State<Arc<AppState>>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutors = dbaccess::tutor::get_tutors(&app_state.db).await?;
    let response: Vec<TutorResponse> = tutors.into_iter().map(TutorResponse::from).collect();
    Ok(HttpResponse::Ok().json(&response))
}

#[web::get("/tutors/{tutor_id}")]
pub async fn get_tutor(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<i64>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    dbaccess::tutor::get_tutor(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(&TutorResponse::from(tutor)))
}

#[web::get("/tutors/{tutor_id}")]
pub async fn update_tutor(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<i64>,
    update_tutor_request: web::types::Json<UpdateTutorRequest>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    dbaccess::tutor::update_tutor(&app_state.db, tutor_id, update_tutor_request.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(&TutorResponse::from(tutor)))
}

#[web::delete("/tutors/{tutor_id}")]
pub async fn delete_tutor(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<i64>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    let _ = dbaccess::tutor::delete_tutor(&app_state.db, tutor_id)
        .await?;
    Ok(HttpResponse::Ok().finish())
}
