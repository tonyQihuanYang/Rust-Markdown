use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::{io, sync::Arc};

use crate::{
    graphql::schema::{create_schema, Schema},
    setting::ServerSetting,
    CONFIG,
};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    st: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
    session: Session,
) -> impl Responder {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);
    let result = data.execute(&st, &()).await;
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
pub async fn connect(secret_key: Key) -> io::Result<()> {
    let ServerSetting { url, port } = &CONFIG.server;
    // Create Juniper schema
    let schema = Arc::new(create_schema());
    // Start HTTP server

    log::info!("GraphiQL playground: {}/graphiql", url);
    log::info!("starting HTTP server on port {}", port);

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
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind((url.to_owned(), port.to_owned()))?
    .run()
    .await
}
