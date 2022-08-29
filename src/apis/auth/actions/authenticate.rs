use actix_session::Session;
use actix_web::{post, HttpResponse};

#[post("/authenticate")]
pub async fn authenticate_post(session: Session) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(_) => HttpResponse::Ok().json("Success"),
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}
