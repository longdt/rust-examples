use sqlx::PgPool;
use time::OffsetDateTime;

use crate::error::EzyTutorError;
use crate::model::course::{Course, CreateCourseRequest, UpdateCourseRequest};

pub async fn create_course(
    pool: &PgPool,
    create_course_request: CreateCourseRequest,
) -> Result<Course, EzyTutorError> {
    let now = OffsetDateTime::now_utc();
    let course_id = sqlx::query!(
        r#"insert into course (tutor_id, course_name, course_description, course_format,
        course_structure, course_duration, course_price, course_language, course_level, created_at)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning course_id
        "#,
        create_course_request.tutor_id,
        create_course_request.course_name,
        create_course_request.course_description,
        create_course_request.course_format,
        create_course_request.course_structure,
        create_course_request.course_duration,
        create_course_request.course_price,
        create_course_request.course_language,
        create_course_request.course_level,
        now
    )
    .fetch_one(pool)
    .await?
    .course_id;
    Ok(Course {
        course_id,
        tutor_id: create_course_request.tutor_id,
        course_name: create_course_request.course_name,
        course_description: create_course_request.course_description,
        course_format: create_course_request.course_format,
        course_structure: create_course_request.course_structure,
        course_duration: create_course_request.course_duration,
        course_price: create_course_request.course_price,
        course_language: create_course_request.course_language,
        course_level: create_course_request.course_level,
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
    sqlx::query_as!(
        Course,
        "select * from course where tutor_id = $1 and course_id = $2 ",
        tutor_id,
        course_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| EzyTutorError::NotFound("Course id not found".into()))
}

pub async fn update_course(
    pool: &PgPool,
    tutor_id: i64,
    course_id: i64,
    update_course_request: UpdateCourseRequest,
) -> Result<Course, EzyTutorError> {
    let record_opt = sqlx::query!(
        r#"update course set course_name = $1, course_description = $2, course_format = $3,
         course_structure = $4, course_duration = $5, course_price = $6, course_language = $7, course_level = $8
         where tutor_id = $9 and course_id = $10 returning created_at"#,
        update_course_request.course_name,
        update_course_request.course_description,
        update_course_request.course_format,
        update_course_request.course_structure,
        update_course_request.course_duration,
        update_course_request.course_price,
        update_course_request.course_language,
        update_course_request.course_level,
        tutor_id,
        course_id
    )
        .fetch_optional(pool)
        .await?;
    match record_opt {
        Some(record) => Ok(Course {
            course_id,
            tutor_id,
            course_name: update_course_request.course_name,
            course_description: update_course_request.course_description,
            course_format: update_course_request.course_format,
            course_structure: update_course_request.course_structure,
            course_duration: update_course_request.course_duration,
            course_price: update_course_request.course_price,
            course_language: update_course_request.course_language,
            course_level: update_course_request.course_level,
            created_at: record.created_at,
        }),
        None => Err(EzyTutorError::NotFound("Course id not found".into())),
    }
}

pub async fn delete_course(
    pool: &PgPool,
    tutor_id: i64,
    course_id: i64,
) -> Result<(), EzyTutorError> {
    let result = sqlx::query!(
        "delete from course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(EzyTutorError::NotFound("Course id not found".to_owned()))
    }
}
