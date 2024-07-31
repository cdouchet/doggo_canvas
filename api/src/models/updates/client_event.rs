use models::canvas::{coordinate::Coordinate, line::Line};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::room::ROOMS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientEvent {
    CursorPosition(Coordinate),
    DrawLine(Line),
    UsernameUpdate(String),
}

impl ClientEvent {
    pub async fn handle(self, room_id: Uuid, user_id: Uuid) {
        match self {
            ClientEvent::CursorPosition(coordinate) => {
                ROOMS
                    .lock()
                    .await
                    .update_cursor_position(room_id, user_id, coordinate)
                    .await;
            }
            ClientEvent::DrawLine(line) => {
                ROOMS.lock().await.draw_line(room_id, user_id, line).await;
            }
            ClientEvent::UsernameUpdate(username) => {}
        }
    }
}
