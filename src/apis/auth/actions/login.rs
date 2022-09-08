use crate::apis::auth::services::auth_services::login_user;
use crate::models::user::LoginCredentials;
use actix_session::Session;
use actix_web::{post, web, HttpResponse};

#[post("/login")]
pub async fn login_post(
    credentials: web::Json<LoginCredentials>,
    session: Session,
) -> HttpResponse {
    let credentials = credentials.into_inner();

    match login_user(credentials).await {
        Ok(user_found) => {
            log::info!("{:?}", user_found.clone().get_id_string());
            session
                .insert("user_id", user_found.get_id_string())
                .unwrap();
        }
        Err(_) => return HttpResponse::Unauthorized().json("Unauthorized"),
    };
    HttpResponse::Ok().json("Success")
}
