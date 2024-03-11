use ntex::web;

use crate::handler::course::{create_course, get_course, get_tutor_courses, update_course};
use crate::handler::general::health_check;
use crate::handler::tutor::{create_tutor, get_tutor, get_tutors, update_tutor, delete_tutor};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_course)
        .service(get_tutor_courses)
        .service(get_course)
        .service(update_course);
}

pub fn tutor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_tutor)
        .service(get_tutors)
        .service(get_tutor)
        .service(update_tutor)
        .service(delete_tutor);
}
