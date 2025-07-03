use crate::infrastructure::web::user_routes::get_users_route;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/users").route("", web::get().to(get_users_route)));
}
