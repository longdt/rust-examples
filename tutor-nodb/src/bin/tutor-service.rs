use std::io;
use std::sync::{Arc, Mutex};

use ntex::web::{App, HttpServer};

use crate::route::{course_routes, general_routes};
use crate::state::AppState;

#[path = "../handler.rs"]
mod handler;
#[path = "../route.rs"]
mod route;
#[path = "../state.rs"]
mod state;
#[path = "../model.rs"]
mod model;
#[ntex::main]
async fn main() -> io::Result<()> {
    let app_state = Arc::new(AppState {
        health_check_response: "I'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .state(app_state.clone())
            .configure(general_routes)
            .configure(course_routes)
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
