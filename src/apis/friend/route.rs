use super::actions::{friends_action, friends_count_action};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/friends")
            .service(friends_action::add_friend)
            .service(friends_action::update_friend)
            .service(friends_action::get_friends)
            .service(friends_count_action::get_count), // .service(friends_action::delete_friend),
    );
}
