use crate::models::User::{LoginCredentials, User};
use crate::MONGO_DB;
use actix_session::Session;
use actix_web::{error::InternalError, web, Error, HttpResponse, Responder};
use base64::{decode, encode};
use mongodb::bson::{doc, Bson, Document};

pub async fn register(
    credentials: web::Json<LoginCredentials>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    let database = MONGO_DB.get().unwrap();
    let collection = database.collection::<User>("users");

    let hashed_password = encode(credentials.password);
    let created = User {
        id: 1,
        username: credentials.username,
        password: hashed_password,
    };

    match collection.insert_one(created.clone(), None).await {
        Ok(_) => Ok("Welcome!"),
        // Err(_) => Err(HttpResponse::Unauthorized().json("Unauthorized")),
        Err(_) => Ok("ERROR!"),
    }
}
