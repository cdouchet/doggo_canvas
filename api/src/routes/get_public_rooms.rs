use crate::models::room::ROOMS;

pub async fn get_public_rooms() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(
        &ROOMS.lock().await.get_public_rooms().await,
    ))
}
