use std::{env, io};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::OffsetDateTime;

#[derive(Debug)]
pub struct Course {
    pub course_id: i64,
    pub tutor_id: i64,
    pub course_name: String,
    pub created_at: Option<OffsetDateTime>,
}

#[ntex::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let courses = sqlx::query!(
        "select course_id, tutor_id, course_name, created_at from course where course_id = $1",
        1
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();
    let mut course_list = vec![];
    for course in courses {
        course_list.push(Course {
            course_id: course.course_id,
            tutor_id: course.tutor_id,
            course_name: course.course_name,
            created_at: course.created_at,
        })
    }
    println!("Courses = {:?}", course_list);
    Ok(())
}
