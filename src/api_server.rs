use crate::apis::{auth, point};
use crate::{setting::ServerSetting, CONFIG};
use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    http, App, HttpServer,
};

#[actix_web::main]
pub async fn connect(secret_key: Key) -> std::io::Result<()> {
    // let AuthSetting { secret_key } = &CONFIG.auth;
    let ServerSetting { url, port } = &CONFIG.api_server;
    log::info!("API Server Connecting on URL {}:{}", url, port);
    // let secret_key = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
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
            .configure(auth::route::config_routes)
            .configure(point::route::config_routes);
        return service;
    })
    .bind((url.to_owned(), port.to_owned()))?
    .run()
    .await
}
