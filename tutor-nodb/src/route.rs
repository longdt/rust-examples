use ntex::web;
use crate::handler::{create_course, get_tutor_courses, health_check};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses")
        .route("/", web::post().to(create_course))
        .route("/{tutor_id}", web::get().to(get_tutor_courses))
    );
}
