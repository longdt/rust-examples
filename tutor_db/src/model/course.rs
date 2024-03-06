use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct Course {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub tutor_id: i64,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl From<Course> for CourseResponse {
    fn from(value: Course) -> Self {
        Self {
            course_id: value.course_id,
            tutor_id: value.tutor_id,
            course_name: value.course_name,
            course_description: value.course_description,
            course_format: value.course_format,
            course_structure: value.course_structure,
            course_duration: value.course_duration,
            course_price: value.course_price,
            course_language: value.course_language,
            course_level: value.course_level,
            created_at: value.created_at,
        }
    }
}
