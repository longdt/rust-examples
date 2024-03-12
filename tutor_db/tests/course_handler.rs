use ntex::http::StatusCode;
use ntex::web::{test, WebServiceFactory};
use ntex::Service;
use serial_test::serial;
use sqlx::Executor;

use tutor_db::handler::course::{
    create_course, delete_course, get_course, get_tutor_courses, update_course,
};

use crate::common::{execute, init_app, new_pool};

mod common;
#[ntex::test]
async fn get_tutor_courses_success() {
    let app = init_app(get_tutor_courses).await;
    let req = test::TestRequest::get().uri("/courses/1").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(StatusCode::OK, resp.status());
}

#[serial(course)]
#[ntex::test]
async fn get_course_success() {
    let pool = new_pool().await;
    execute(&pool, r#"truncate course;
            INSERT INTO course (course_id, tutor_id, course_name, course_description, course_format, course_structure, course_duration, course_price, course_language, course_level, created_at) VALUES (1, 1, 'Hello C++', null, null, null, null, null, null, '100', '2024-03-06 11:32:00.728068 +00:00');
        "#)
        .await
        .unwrap();
    let app = init_app(get_course).await;
    let req = test::TestRequest::get().uri("/courses/1/1").to_request();
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

#[serial(course)]
#[ntex::test]
async fn update_course_success() {
    let pool = new_pool().await;
    execute(&pool, r#"truncate course;
            INSERT INTO course (course_id, tutor_id, course_name, course_description, course_format, course_structure, course_duration, course_price, course_language, course_level, created_at) VALUES (1, 1, 'Hello C++', null, null, null, null, null, null, '100', '2024-03-06 11:32:00.728068 +00:00');
        "#)
        .await
        .unwrap();
    let app = init_app(update_course).await;
    let req = test::TestRequest::put()
        .uri("/courses/1/1")
        .header("Content-Type", "application/json")
        .set_payload(
            r#"
            {
                "course_name": "This is the next course",
                "course_description": "Update new book description"
             }
            "#,
        )
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(StatusCode::OK, resp.status());
}

#[serial(course)]
#[ntex::test]
async fn delete_course_success() {
    let pool = new_pool().await;
    execute(&pool, r#"truncate course;
            INSERT INTO course (course_id, tutor_id, course_name, course_description, course_format, course_structure, course_duration, course_price, course_language, course_level, created_at) VALUES (1, 1, 'Hello C++', null, null, null, null, null, null, '100', '2024-03-06 11:32:00.728068 +00:00');
        "#)
        .await
        .unwrap();
    let app = init_app(delete_course).await;
    let req = test::TestRequest::delete().uri("/courses/1/1").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
