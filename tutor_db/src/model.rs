use ntex::web::types::Json;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub struct Course {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
