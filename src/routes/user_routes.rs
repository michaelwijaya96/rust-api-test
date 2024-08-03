use actix_web::web;

use crate::handlers::user_handlers::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_users))
        .route("/{id}", web::get().to(get_user))
        .route("", web::post().to(create_user));
}
