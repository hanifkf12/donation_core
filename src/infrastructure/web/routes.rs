use crate::infrastructure::web::user_routes::*;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users_route))
            .route("", web::post().to(create_user_route))
            .route("/{id}", web::get().to(get_user_by_id_route))
            .route("/{id}", web::put().to(update_user_route))
            .route("/{id}", web::delete().to(delete_user_route))
    );
}
