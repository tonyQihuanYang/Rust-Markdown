use actix_session::Session;
use actix_web::{get, HttpResponse};

use crate::apis::point::services::points_service;

#[get("")]
pub async fn get_points(session: Session) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(id) => match points_service::get_points(id).await {
            Ok(point) => HttpResponse::Ok().json(point),
            Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
        },
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

#[get("/user")]
pub async fn get_points_with_user_info(session: Session) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(id) => match points_service::get_points_with_user_info(id).await {
            Ok(point) => HttpResponse::Ok().json(point),
            Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
        },
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}
