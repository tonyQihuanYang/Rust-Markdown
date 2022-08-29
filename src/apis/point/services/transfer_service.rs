use crate::apis::auth::db::users::find_user_by_id;
use crate::apis::point::db::points::{add_point, remove_point};

pub async fn transfer_point(
    transfer_from: String,
    transfer_to: String,
    points: i64,
) -> Result<(), ()> {
    let user_transfer_from = find_user_by_id(transfer_from.clone()).await;
    if user_transfer_from.is_none() {
        log::error!("transfer_point | Could not find transfer_from");
        return Err(());
    }

    let user_transfer_to = find_user_by_id(transfer_to.clone()).await;
    if user_transfer_to.is_none() {
        log::error!("transfer_point | Could not find transfer_to");
        return Err(());
    }

    add_point(transfer_to, points.clone()).await.ok_or(())?;
    remove_point(transfer_from, points.clone())
        .await
        .ok_or(())?;
    Ok(())
}
