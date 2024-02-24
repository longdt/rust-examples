use ntex::web::types::Json;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct Course {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCourseRequest {
    pub tutor_id: i64,
    pub course_name: String,
}

impl From<Json<CreateCourseRequest>> for Course {
    fn from(value: Json<CreateCourseRequest>) -> Self {
        Course {
            course_id: 0,
            tutor_id: value.tutor_id,
            course_name: value.course_name.clone(),
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponse {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl From<Course> for CourseResponse {
    fn from(value: Course) -> Self {
        Self {
            course_id: value.course_id,
            tutor_id: value.tutor_id,
            course_name: value.course_name,
            created_at: value.created_at,
        }
    }
}
