use time::OffsetDateTime;
use ntex::web::types::Json;
use serde::{Deserialize, Serialize};

pub struct Course {
    pub tutor_id: i64,
    pub course_id: Option<i64>,
    pub course_name: String,
    pub created_at: Option<OffsetDateTime>
}

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub tutor_id: i64,
    pub course_name: String,
}

impl From<Json<CreateCourseRequest>> for Course {
    fn from(course: Json<CreateCourseRequest>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: None,
            course_name: course.course_name.clone(),
            created_at: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub tutor_id: i64,
    pub course_id: i64,
    pub course_name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl From<Course> for CourseResponse {
    fn from(value: Course) -> Self {
        Self {
            tutor_id: value.tutor_id,
            course_id: value.course_id.unwrap(),
            course_name: value.course_name,
            created_at: value.created_at.unwrap()
        }
    }
}

impl From<&Course> for CourseResponse {
    fn from(value: &Course) -> Self {
        Self {
            tutor_id: value.tutor_id,
            course_id: value.course_id.unwrap(),
            course_name: value.course_name.clone(),
            created_at: value.created_at.unwrap()
        }
    }
}
