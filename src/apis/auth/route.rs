use super::actions::{authenticate, login, register};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth").service(
            web::scope("")
                .service(login::login_post)
                .service(register::register_post)
                .service(authenticate::authenticate_post),
        ),
    );
}
