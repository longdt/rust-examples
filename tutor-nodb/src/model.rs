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
