use ntex::web;
use crate::handler::{create_course, health_check};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses").route("/", web::post().to(create_course)));
}
