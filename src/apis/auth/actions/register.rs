use crate::apis::auth::services::auth_services::register_user;
use crate::models::user::LoginCredentials;
use actix_web::{web, HttpResponse};

pub async fn register(credentials: web::Json<LoginCredentials>) -> HttpResponse {
    match register_user(credentials.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Success"),
        Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}
