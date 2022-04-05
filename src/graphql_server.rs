use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::{graphql_schema::{create_schema, Schema}, CONFIG, setting::{ServerSetting}};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
pub async fn connect() -> io::Result<()> {
    let ServerSetting {url, port} = &CONFIG.server;
    // Create Juniper schema
    let schema = Arc::new(create_schema());
    // Start HTTP server
    
    log::info!("GraphiQL playground: {}/graphiql", url);
    log::info!("starting HTTP server on port {}", port);

    HttpServer::new(move || {
        App::new()
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
