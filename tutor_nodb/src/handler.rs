use std::sync::Arc;

use ntex::web;
use ntex::web::HttpResponse;
use time::OffsetDateTime;

use crate::model::{Course, CourseResponse, CreateCourseRequest};
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

pub async fn create_course(app_state: web::types::State<Arc<AppState>>,
                           create_course_request: web::types::Json<CreateCourseRequest>) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state.courses.lock().unwrap()
        .iter()
        .filter(|&course| course.tutor_id == create_course_request.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: create_course_request.tutor_id,
        course_id: Some((course_count_for_user + 1) as i64),
        course_name: create_course_request.course_name.clone(),
        created_at: Some(OffsetDateTime::now_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json(&"Added course")
}

pub async fn get_tutor_courses(app_state: web::types::State<Arc<AppState>>,
                               params: web::types::Path<i64>, ) -> HttpResponse {
    let tutor_id = params.into_inner();
    let tutor_courses = app_state.courses
        .lock()
        .unwrap()
        .iter()
        .filter(|course| course.tutor_id == tutor_id)
        .map(CourseResponse::from)
        .collect::<Vec<CourseResponse>>();
    HttpResponse::Ok().json(&tutor_courses)
}

pub async fn get_course(app_state: web::types::State<Arc<AppState>>,
        params: web::types::Path<(i64, i64)>) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let found_course = app_state.courses
        .lock()
        .unwrap()
        .iter()
        .filter(|&course| course.tutor_id == tutor_id && course.course_id == Some(course_id))
        .map(CourseResponse::from)
        .next();
    if let Some(ref c) = found_course {
        HttpResponse::Ok().json(c)
    } else {
        HttpResponse::Ok().json(&"Course not found")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use ntex::http::StatusCode;
    use ntex::web::{App, test};

    use super::*;

    #[ntex::test]
    async fn test_create_course() {
        let app_state = Arc::new(AppState {
            health_check_response: String::from("I'm good. You've already asked me"),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let app = test::init_service(App::new()
            .state(app_state)
            .route("/courses", web::post().to(create_course))
        ).await;
        // let create_course_request = Json(CreateCourseRequest {
        //     tutor_id: 1,
        //     course_name: "Hello, this is test course".into(),
        // });
        let req = test::TestRequest::post()
            .uri("/courses")
            .header("Content-Type", "application/json")
            .set_payload(r#"
            {
                "tutor_id": 1,
                "course_name": "Hello, this is test course"
            }
            "#)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, resp.status())
    }

    #[ntex::test]
    async fn test_get_tutor_courses() {
        let app_state = Arc::new(AppState {
            health_check_response: String::from("Health check Success"),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![
                Course {
                    tutor_id: 1,
                    course_id: Some(1),
                    course_name: String::from("Hello Rust"),
                    created_at: Some(OffsetDateTime::now_utc()),
                },
                Course {
                    tutor_id: 2,
                    course_id: Some(1),
                    course_name: String::from("Hello Java"),
                    created_at: Some(OffsetDateTime::now_utc()),
                },
            ]),
        });
        let app = test::init_service(App::new()
            .state(app_state)
            .route("/courses/{tutor_id}", web::get().to(get_tutor_courses))
        ).await;
        let req = test::TestRequest::get()
            .uri("/courses/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, resp.status());
        resp.response().body();
    }

    #[ntex::test]
    async fn test_get_course() {
        let app_state = Arc::new(AppState {
            health_check_response: String::from("Health check Success"),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![
                Course {
                    tutor_id: 1,
                    course_id: Some(1),
                    course_name: String::from("Hello Rust"),
                    created_at: Some(OffsetDateTime::now_utc()),
                },
                Course {
                    tutor_id: 2,
                    course_id: Some(1),
                    course_name: String::from("Hello Java"),
                    created_at: Some(OffsetDateTime::now_utc()),
                },
            ]),
        });
        let app = test::init_service(App::new()
            .state(app_state)
            .route("/courses/{tutor_id}/{course_id}", web::get().to(get_course))
        ).await;
        let req = test::TestRequest::get()
            .uri("/courses/2/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, resp.status());
        resp.response().body();
    }
}
