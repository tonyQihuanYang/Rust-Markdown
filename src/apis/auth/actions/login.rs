use crate::apis::auth::services::auth_services::login_user;
use crate::models::user::LoginCredentials;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse, Responder};

pub async fn login_post(
    credentials: web::Json<LoginCredentials>,
    session: Session,
) -> HttpResponse {
    let credentials = credentials.into_inner();

    match login_user(credentials).await {
        Ok(user_found) => session
            .insert("user_id", user_found.get_id_string())
            .unwrap(),
        Err(_) => return HttpResponse::Unauthorized().json("Unauthorized"),
    };
    HttpResponse::Ok().json("Success")
}

// pub async fn secret(session: Session) -> Result<impl Responder, Error> {
//     log::info!("Comes here...");
//     // only allow access to this resource if the user has an active session
//     // validate_session(&session).map_err(|err| InternalError::from_response("", err))?;

//     Ok("secret revealed")
// }

// pub fn validate_session(session: &Session) -> Result<i64, HttpResponse> {
//     let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

//     match user_id {
//         Some(id) => {
//             log::info!("{}", id);
//             // keep the user's session alive
//             session.renew();
//             Ok(id)
//         }
//         None => Err(HttpResponse::Unauthorized().json("Unauthorized")),
//     }
// }
