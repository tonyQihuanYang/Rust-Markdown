use crate::apis::friend::services::friends_count_service;
use crate::models::friend::friend_state::FriendState;
use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetCountSearch {
    state: Option<FriendState>,
}

// /count?state=Pending
#[get("/count")]
pub async fn get_count(
    session: Session,
    web::Query(search): web::Query<GetCountSearch>,
) -> HttpResponse {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(current_user_id) => {
            match friends_count_service::get_count(current_user_id, search.state).await {
                Ok(count) => HttpResponse::Ok().json(count),
                Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
            }
        }
        None => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}
