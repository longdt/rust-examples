use std::env;
use std::sync::{Arc, Mutex};
use dotenvy::dotenv;
use ntex::{Pipeline, Service};
use ntex::http::Request;
use ntex::web::{App, DefaultError, Error, test, WebResponse, WebServiceFactory};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tutor_db::state::AppState;

pub async fn init_app<F>(
    factory: F,
) -> Pipeline<impl Service<Request, Response = WebResponse, Error = Error> + Sized>
    where
        F: WebServiceFactory<DefaultError> + 'static,
{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let app_state = Arc::new(AppState {
        health_check_response: String::from("I'm good. You've already asked me"),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    test::init_service(App::new().state(app_state).service(factory)).await
}

pub async fn new_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}

pub async fn execute(pool: &PgPool, sql: &str) -> Result<(), sqlx::Error> {
    let statements = sql.split(";");

    for stm in statements {
        sqlx::query(stm).execute(pool).await?;
    }
    Ok(())
}
