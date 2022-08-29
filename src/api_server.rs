use crate::apis::{auth::actions::login::login_post, auth::actions::register::register, point};
use crate::{setting::ServerSetting, CONFIG};
use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    http, web, App, HttpServer,
};

#[actix_web::main]
pub async fn connect() -> std::io::Result<()> {
    // let AuthSetting { secret_key } = &CONFIG.auth;
    let ServerSetting { url, port } = &CONFIG.api_server;
    log::info!("API Server Connecting on URL {}:{}", url, port);
    let secret_key = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["OPTIONS", "GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let service = App::new()
            .wrap(cors)
            // .wrap(Cors::permissive())
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    secret_key.clone(),
                )
                // .cookie_http_only(false)
                .cookie_same_site(SameSite::None)
                .cookie_secure(false)
                .build(),
            )
            // .default_service(web::to(|| HttpResponse::Ok()))
            .route("/auth/login", web::post().to(login_post))
            .route("/auth/register", web::post().to(register))
            // .route("/auth/authenticate", web::get().to(secret))
            .configure(point::route::config_point_router);
        // .route("/transfer", web::post().to(transfer_post));
        return service;
    })
    .bind((url.to_owned(), port.to_owned()))?
    .run()
    .await
}
