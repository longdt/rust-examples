use ntex::http::StatusCode;
use ntex::web::test;
use serial_test::serial;

use tutor_db::handler::tutor::{create_tutor, delete_tutor, get_tutor, get_tutors, update_tutor};
use tutor_db::model::tutor::TutorResponse;

use crate::common::{execute, init_app, new_pool};

mod common;

#[ntex::test]
async fn create_tutor_success() {
    let app = init_app(create_tutor).await;
    let req = test::TestRequest::post()
        .uri("/tutors")
        .header("Content-Type", "application/json")
        .set_payload(
            r#"
        {
            "tutor_name": "Andrew Ng",
            "tutor_pic_url": "https://avatar.com/andrew_ng.png",
            "tutor_profile": "Hello Everyone"
        }
        "#,
        )
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[serial(tutor)]
#[ntex::test]
async fn get_tutors_success() {
    let pool = new_pool().await;
    execute(&pool, r#"truncate tutor;
    INSERT INTO public.tutor (tutor_id, tutor_name, tutor_pic_url, tutor_profile, created_at)
     VALUES (1, 'Andrew Ng', 'https://avatar.com/andrew_ng.png', 'Hello Everyone', '2024-03-11 10:32:42.485431 +00:00'),
     (2, 'Mathew Ng', 'https://avatar.com/mathew_ng.png', 'Hello Rust', '2024-03-11 10:32:42.485431 +00:00');
    "#).await.unwrap();
    let app = init_app(get_tutors).await;
    let req = test::TestRequest::get().uri("/tutors").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let mut tutors: Vec<TutorResponse> = serde_json::from_slice(&body).unwrap();
    assert_eq!(tutors.len(), 2);
    tutors.sort_by_cached_key(|tutor_response| tutor_response.tutor_id);
    assert_eq!(tutors[0].tutor_id, 1);
    assert_eq!(tutors[0].tutor_name, "Andrew Ng");
    assert_eq!(tutors[0].tutor_pic_url, "https://avatar.com/andrew_ng.png");
    assert_eq!(tutors[0].tutor_profile, "Hello Everyone");

    assert_eq!(tutors[1].tutor_id, 2);
    assert_eq!(tutors[1].tutor_name, "Mathew Ng");
    assert_eq!(tutors[1].tutor_pic_url, "https://avatar.com/mathew_ng.png");
    assert_eq!(tutors[1].tutor_profile, "Hello Rust");
}

#[serial(tutor)]
#[ntex::test]
async fn get_tutor_success() {
    let pool = new_pool().await;
    execute(&pool, r#"
    truncate tutor;
    INSERT INTO public.tutor (tutor_id, tutor_name, tutor_pic_url, tutor_profile, created_at)
     VALUES (1, 'Andrew Ng', 'https://avatar.com/andrew_ng.png', 'Hello Everyone', '2024-03-11 10:32:42.485431 +00:00');
    "#).await.unwrap();
    let app = init_app(get_tutor).await;
    let req = test::TestRequest::get().uri("/tutors/1").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let tutor_response: TutorResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(tutor_response.tutor_id, 1);
    assert_eq!(tutor_response.tutor_name, "Andrew Ng");
    assert_eq!(
        tutor_response.tutor_pic_url,
        "https://avatar.com/andrew_ng.png"
    );
    assert_eq!(tutor_response.tutor_profile, "Hello Everyone");
}

#[serial(tutor)]
#[ntex::test]
async fn update_tutor_success() {
    let pool = new_pool().await;
    execute(&pool, r#"
    truncate tutor;
    INSERT INTO public.tutor (tutor_id, tutor_name, tutor_pic_url, tutor_profile, created_at)
     VALUES (1, 'Andrew Ng', 'https://avatar.com/andrew_ng.png', 'Hello Everyone', '2024-03-11 10:32:42.485431 +00:00');
    "#).await.unwrap();
    let app = init_app(update_tutor).await;
    let req = test::TestRequest::patch()
        .uri("/tutors/1")
        .header("Content-Type", "application/json")
        .set_payload(
            r#"
        {
            "tutor_name": "Mathew",
            "tutor_profile": "Hello Rust"
        }
        "#,
        )
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let tutor_response: TutorResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(tutor_response.tutor_id, 1);
    assert_eq!(tutor_response.tutor_name, "Mathew");
    assert_eq!(
        tutor_response.tutor_pic_url,
        "https://avatar.com/andrew_ng.png"
    );
    assert_eq!(tutor_response.tutor_profile, "Hello Rust");
}

#[serial(tutor)]
#[ntex::test]
async fn delete_tutor_success() {
    let pool = new_pool().await;
    execute(&pool, r#"
    truncate tutor;
    INSERT INTO public.tutor (tutor_id, tutor_name, tutor_pic_url, tutor_profile, created_at)
     VALUES (1, 'Andrew Ng', 'https://avatar.com/andrew_ng.png', 'Hello Everyone', '2024-03-11 10:32:42.485431 +00:00');
    "#).await.unwrap();
    let app = init_app(delete_tutor).await;
    let req = test::TestRequest::delete().uri("/tutors/1").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
