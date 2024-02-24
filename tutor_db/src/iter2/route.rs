use super::handler::*;
use ntex::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_course)
        .service(get_tutor_courses)
        .service(get_course);
}
