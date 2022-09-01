use actix_session::Session;
use actix_web::{post, web, HttpResponse};

use crate::apis::point::services::transfer_service::transfer_point;

#[post("/{transfer_to}/{point}")]
pub async fn transfer_post(session: Session, path: web::Path<(String, i64)>) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);
    let (transfer_to, points) = path.into_inner();

    match user_id {
        Some(transfer_from) => match transfer_point(transfer_from, transfer_to, points).await {
            Ok(_) => HttpResponse::Ok().json("Success"),
            Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
        },
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}
