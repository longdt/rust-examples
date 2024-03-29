use std::sync::{Arc, Mutex};
use std::{env, io};

use dotenvy::dotenv;
use ntex::web::middleware::Logger;
use ntex::web::{App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::util::SubscriberInitExt;

use tutor_db::route::{course_routes, general_routes, tutor_routes};
use tutor_db::state::AppState;

#[ntex::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let app_state = Arc::new(AppState {
        health_check_response: String::from("I'm good. You already asked me"),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .state(app_state.clone())
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
