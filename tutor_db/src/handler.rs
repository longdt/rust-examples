use crate::dbaccess;
use crate::model::{CourseResponse, CreateCourseRequest};
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
    let course = dbaccess::create_course(&app_state.db, &create_course_request).await;
    HttpResponse::Ok().json(&CourseResponse::from(course))
}

#[web::get("/courses/{tutor_id}")]
pub async fn get_tutor_courses(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<i64>,
) -> HttpResponse {
    let tutor_id = params.into_inner();
    let courses = dbaccess::get_tutor_courses(&app_state.db, tutor_id).await;
    let response = courses
        .into_iter()
        .map(CourseResponse::from)
        .collect::<Vec<CourseResponse>>();
    HttpResponse::Ok().json(&response)
}

#[web::get("/courses/{tutor_id}/{course_id}")]
pub async fn get_course(
    app_state: web::types::State<Arc<AppState>>,
    params: web::types::Path<(i64, i64)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let course = dbaccess::get_course(&app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(&CourseResponse::from(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use ntex::http::{Request, StatusCode};
    use ntex::web::{test, App, DefaultError, Error, WebResponse, WebServiceFactory};
    use ntex::{Pipeline, Service};
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

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
