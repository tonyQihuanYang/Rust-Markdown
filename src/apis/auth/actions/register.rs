use crate::apis::auth::services::auth_services::register_user;
use crate::models::user::LoginCredentials;
use actix_web::{post, web, HttpResponse};

#[post("/register")]
pub async fn register_post(credentials: web::Json<LoginCredentials>) -> HttpResponse {
    match register_user(credentials.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Success"),
        Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}
