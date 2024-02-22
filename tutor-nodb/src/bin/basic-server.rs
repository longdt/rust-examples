use std::io;
use ntex::web;
use ntex::web::{App, HttpResponse, HttpServer, Responder};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json(&"Hello. EzyTutors is alive and kicking")
}

#[ntex::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().configure(general_routes))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
