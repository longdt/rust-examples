use std::sync::Arc;
use ntex::web;
use ntex::web::HttpResponse;
use crate::dbaccess;
use crate::error::EzyTutorError;
use crate::model::course::{CourseResponse, CreateCourseRequest};
use crate::state::AppState;

#[web::post("/courses")]
pub async fn create_course(
    app_state: web::types::State<Arc<AppState>>,
    create_course_request: web::types::Json<CreateCourseRequest>,
) -> Result<HttpResponse, EzyTutorError> {
    dbaccess::course::create_course(&app_state.db, &create_course_request)
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

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;

    use dotenvy::dotenv;
    use ntex::{Pipeline, Service};
    use ntex::http::{Request, StatusCode};
    use ntex::web::{App, DefaultError, Error, test, WebResponse, WebServiceFactory};
    use sqlx::postgres::PgPoolOptions;

    use super::*;

    async fn init_app<F>(
        factory: F,
    ) -> Pipeline<impl Service<Request, Response = WebResponse, Error = Error> + Sized>
        where
            F: WebServiceFactory<DefaultError> + 'static,
    {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        let app_state = Arc::new(AppState {
            health_check_response: String::from("I'm good. You've already asked me"),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        test::init_service(App::new().state(app_state).service(factory)).await
    }

    #[ntex::test]
    async fn get_tutor_courses_success() {
        let app = init_app(get_tutor_courses).await;
        let req = test::TestRequest::get().uri("/courses/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[ntex::test]
    async fn get_course_success() {
        let app = init_app(get_course).await;
        let req = test::TestRequest::get().uri("/courses/1/2").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[ntex::test]
    async fn create_course_success() {
        let app = init_app(create_course).await;
        let req = test::TestRequest::post()
            .uri("/courses")
            .header("Content-Type", "application/json")
            .set_payload(
                r#"
             {
                "tutor_id": 1,
                "course_name": "This is the next course"
             }
            "#,
            )
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, resp.status());
    }
}
