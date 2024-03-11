use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow)]
pub struct Tutor {
    pub tutor_id: i64,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTutorRequest {
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTutorRequest {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TutorResponse {
    pub tutor_id: i64,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
    pub created_at: OffsetDateTime,
}

impl From<Tutor> for TutorResponse {
    fn from(value: Tutor) -> Self {
        Self {
            tutor_id: value.tutor_id,
            tutor_name: value.tutor_name,
            tutor_pic_url: value.tutor_pic_url,
            tutor_profile: value.tutor_profile,
            created_at: value.created_at,
        }
    }
}
