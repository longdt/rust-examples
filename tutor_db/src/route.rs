use ntex::web;

use crate::handler::course::{create_course, get_course, get_tutor_courses, update_course};
use crate::handler::general::health_check;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_course)
        .service(get_tutor_courses)
        .service(get_course)
        .service(update_course);
}
