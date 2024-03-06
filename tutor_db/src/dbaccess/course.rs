use sqlx::PgPool;
use time::OffsetDateTime;
use crate::error::EzyTutorError;
use crate::model::course::{Course, CreateCourseRequest};

pub async fn create_course(
    pool: &PgPool,
    create_course_request: &CreateCourseRequest,
) -> Result<Course, EzyTutorError> {
    let now = OffsetDateTime::now_utc();
    let course_id = sqlx::query!(
        "insert into course (tutor_id, course_name, created_at) values ($1, $2, $3) returning course_id",
        create_course_request.tutor_id,
        create_course_request.course_name,
        now
    )
        .fetch_one(pool)
        .await?
        .course_id;
    Ok(Course {
        course_id,
        tutor_id: create_course_request.tutor_id,
        course_name: create_course_request.course_name.clone(),
        created_at: now,
    })
}

pub async fn get_tutor_courses(pool: &PgPool, tutor_id: i64) -> Result<Vec<Course>, EzyTutorError> {
    let courses = sqlx::query_as!(Course, "select * from course where tutor_id = $1", tutor_id)
        .fetch_all(pool)
        .await?;
    if courses.len() == 0 {
        Err(EzyTutorError::NotFound(
            "Courses not found for tutor".into(),
        ))
    } else {
        Ok(courses)
    }
}

pub async fn get_course(
    pool: &PgPool,
    tutor_id: i64,
    course_id: i64,
) -> Result<Course, EzyTutorError> {
    let course_opt = sqlx::query_as!(
        Course,
        "select * from course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
        .fetch_optional(pool)
        .await?;
    match course_opt {
        Some(c) => Ok(c),
        None => Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}
