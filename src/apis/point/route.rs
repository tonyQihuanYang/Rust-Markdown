use super::actions::transfer;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/points")
            // .service(
            //     web::resource("")
            //         .route(web::get().to(products::get_products))
            //         .route(web::post().to(products::add_product)),
            // )
            .service(web::scope("/transfer").service(transfer::transfer_post)),
    );
}

//Ref: https://github.com/actix/examples/blob/master/basics/nested-routing/src/app_config.rs
