use super::actions::{points_action, transfer_action};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/points")
            .service(points_action::get_points)
            .service(web::scope("/transfer").service(transfer_action::transfer_post)),
    );
}

//Ref: https://github.com/actix/examples/blob/master/basics/nested-routing/src/app_config.rs
