use models::room::Room;
use uuid::Uuid;

use crate::{
    handlers::{on_upgrade, WebSocketQuery},
    models::room::ROOMS,
};

pub async fn websocket_route(
    ws: warp::ws::Ws,
    query: WebSocketQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    let room: Result<Room, Uuid> = ROOMS
        .lock()
        .await
        .get_room_by_uid(query.room_id)
        .await
        .map_err(|_| query.room_id);

    Ok(ws.on_upgrade(|websocket| on_upgrade(websocket, room, query.username)))
}
