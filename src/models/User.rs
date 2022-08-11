use crate::MONGO_DB;
use actix_web::{error::InternalError, web, Error, HttpResponse, Responder};
use base64::{decode, encode};
use mongodb::bson::{doc, Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn authenticate(credentials: LoginCredentials) -> Result<Self, HttpResponse> {
        let hashed_password = encode(&credentials.password);

        let database = MONGO_DB.get().unwrap();
        let collection = database.collection::<User>("users");
        let filter =
            doc! {"username":&credentials.username.to_owned(), "password": hashed_password};
        match collection.find_one(filter, None).await {
            Ok(result) => match result {
                Some(user_found) => Ok(User {
                    id: 42,
                    username: user_found.username,
                    password: user_found.password,
                }),
                None => Err(HttpResponse::Unauthorized().json("Unauthorized")),
            },
            Err(_) => Err(HttpResponse::Unauthorized().json("Unauthorized")),
        }
    }
}
