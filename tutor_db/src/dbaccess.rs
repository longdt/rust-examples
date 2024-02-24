use crate::model::{Course, CreateCourseRequest};
use sqlx::PgPool;
use time::OffsetDateTime;

pub async fn create_course(pool: &PgPool, create_course_request: &CreateCourseRequest) -> Course {
    let now = OffsetDateTime::now_utc();
    let course_id = sqlx::query!(
        r#"insert into course (tutor_id, course_name, created_at) values ($1, $2, $3) returning course_id"#,
        create_course_request.tutor_id,
        create_course_request.course_name,
        now
    )
        .fetch_one(pool)
        .await
        .unwrap()
        .course_id;
    Course {
        course_id,
        tutor_id: create_course_request.tutor_id,
        course_name: create_course_request.course_name.clone(),
        created_at: now,
    }
}

pub async fn get_tutor_courses(pool: &PgPool, tutor_id: i64) -> Vec<Course> {
    sqlx::query_as!(
        Course,
        r#"select * from course where tutor_id = $1
    "#,
        tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn get_course(pool: &PgPool, tutor_id: i64, course_id: i64) -> Course {
    sqlx::query_as!(
        Course,
        r#"select * from course where tutor_id = $1 and course_id = $2"#,
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
}
