use uuid::Uuid;

use crate::models::room::ROOMS;

pub async fn get_room_state(room_id: Uuid) -> Result<impl warp::Reply, warp::Rejection> {
    let room = ROOMS.lock().await.get_room_by_uid(room_id).await;
    if let Err(err) = room {
        return Ok(warp::reply::with_status(
            warp::reply::json(&err),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }
    Ok(warp::reply::with_status(
        warp::reply::json(&room.unwrap()),
        warp::http::StatusCode::OK,
    ))
}
