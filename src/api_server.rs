use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    web, App, HttpResponse, HttpServer,
};

use crate::apis::auth::login;
use crate::apis::auth::secret;
use crate::{setting::ServerSetting, CONFIG};

#[actix_web::main]
pub async fn connect() -> std::io::Result<()> {
    // let AuthSetting { secret_key } = &CONFIG.auth;
    let ServerSetting { url, port } = &CONFIG.api_server;
    log::info!("API Server Connecting on URL {}:{}", url, port);
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
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
            .default_service(web::to(|| HttpResponse::Ok()))
            .route("/login", web::post().to(login))
            .route("/secret", web::get().to(secret))
    })
    .bind((url.to_owned(), port.to_owned()))?
    .run()
    .await
}
