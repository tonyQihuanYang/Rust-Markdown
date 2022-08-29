use actix_web::{post, web, HttpResponse};

// #[post("/point/transfer/{transfer_to}/{points}")]
// async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
// }
//

use super::actions::transfer;

pub fn config_point_router(cfg: &mut web::ServiceConfig) {
    // domain includes: /products/{product_id}/parts/{part_id}
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
