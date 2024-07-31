use futures::{SinkExt, StreamExt};
use models::{room::Room, user::User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::filters::ws::WebSocket;

use crate::models::{room::ROOMS, updates::client_event::ClientEvent, user::ServerUser};

pub mod draw_line;

#[derive(Serialize, Deserialize)]
pub struct WebSocketQuery {
    pub room_id: Uuid,
    pub username: String,
}

pub async fn on_upgrade(websocket: WebSocket, room: Result<Room, Uuid>, username: String) {
    let room = match room {
        Ok(room) => room,
        Err(room_id) => {
            ROOMS.lock().await.create_room(false, Some(room_id)).await;
            ROOMS.lock().await.get_room_by_uid(room_id).await.unwrap()
        }
    };
    let (tx, mut rx) = websocket.split();
    let user_uid = Uuid::new_v4();
    let user = User {
        uid: user_uid,
        username,
    };
    let server_user = ServerUser {
        user: user.clone(),
        sink: tx,
    };
    ROOMS.lock().await.add_user(room.uid, server_user).await;
    println!("New user was added");

    while let Some(msg) = rx.next().await {
        let msg = match msg {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error getting websocket message: {:?}", err);
                continue;
            }
        };
        if msg.is_close() {
            break;
        }
        if !msg.is_text() {
            continue;
        }
        let event = match serde_json::from_str::<ClientEvent>(msg.to_str().unwrap()) {
            Ok(e) => e,
            Err(_) => continue,
        };
        event.handle(room.uid, user_uid).await;
    }
    println!("End of ws stream");

    ROOMS.lock().await.remove_user(room.uid, user_uid).await;
}
