use crate::error::EzyTutorError;
use crate::model::tutor::{CreateTutorRequest, Tutor, UpdateTutorRequest};
use sqlx::PgPool;
use time::OffsetDateTime;

pub async fn create_tutor(
    pool: &PgPool,
    create_tutor_request: CreateTutorRequest,
) -> Result<Tutor, EzyTutorError> {
    let now = OffsetDateTime::now_utc();
    let tutor_id = sqlx::query!(
        r#"
    insert into tutor (tutor_name, tutor_pic_url, tutor_profile, created_at)
    values ($1, $2, $3, $4) returning tutor_id
    "#,
        create_tutor_request.tutor_name,
        create_tutor_request.tutor_pic_url,
        create_tutor_request.tutor_profile,
        now
    )
    .fetch_one(pool)
    .await?
    .tutor_id;
    Ok(Tutor {
        tutor_id,
        tutor_name: create_tutor_request.tutor_name,
        tutor_pic_url: create_tutor_request.tutor_pic_url,
        tutor_profile: create_tutor_request.tutor_profile,
        created_at: now
    })
}

pub async fn get_tutors(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    sqlx::query_as!(Tutor, "select * from tutor")
        .fetch_all(pool)
        .await
        .map_err(EzyTutorError::from)
}

pub async fn get_tutor(pool: &PgPool, tutor_id: i64) -> Result<Tutor, EzyTutorError> {
    sqlx::query_as!(Tutor, "select * from tutor where tutor_id = $1", tutor_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| EzyTutorError::NotFound("Tutor id not found".to_owned()))
}

pub async fn update_tutor(
    pool: &PgPool,
    tutor_id: i64,
    update_tutor_request: UpdateTutorRequest,
) -> Result<Tutor, EzyTutorError> {
    let record_opt = sqlx::query!(r#"
    update tutor set tutor_name = COALESCE($1, tutor_name),
                 tutor_pic_url = COALESCE($2, tutor_pic_url),
                 tutor_profile = COALESCE($3, tutor_profile)
                 where tutor_id = $4 returning tutor_name, tutor_pic_url, tutor_profile, created_at;
    "#,
        update_tutor_request.tutor_name,
        update_tutor_request.tutor_pic_url,
        update_tutor_request.tutor_profile,
        tutor_id,
    )
        .fetch_optional(pool)
        .await?;
    match record_opt {
        Some(record) => Ok(Tutor {
            tutor_id,
            tutor_name: record.tutor_name,
            tutor_pic_url: record.tutor_pic_url,
            tutor_profile: record.tutor_profile,
            created_at: record.created_at,
        }),
        None => Err(EzyTutorError::NotFound("Tutor id not found".to_owned()))
    }
}

pub async fn delete_tutor(pool: &PgPool, tutor_id: i64) -> Result<(), EzyTutorError> {
    let result = sqlx::query!("delete from tutor where tutor_id = $1", tutor_id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(EzyTutorError::NotFound("Tutor id not found".to_owned()))
    }
}
