use ntex::web::types::Json;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub struct Course {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateCourseRequest {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    pub created_at: Option<OffsetDateTime>,
}

impl From<Json<CreateCourseRequest>> for Course {
    fn from(value: Json<CreateCourseRequest>) -> Self {
        Course {
            course_id: value.course_id,
            tutor_id: value.tutor_id,
            course_name: value.course_name.clone(),
            created_at: value.created_at,
        }
    }
}
