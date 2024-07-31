use futures::SinkExt;
use models::{
    canvas::{canvas_state::CanvasState, coordinate::Coordinate, line::Line, user_line::UserLine},
    room::Room,
};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::error::CanvasError;

use super::{updates::server_event::ServerEvent, user::ServerUser};

#[derive(Debug)]
pub struct ServerRoom {
    uid: Uuid,
    users: Vec<ServerUser>,
    public: bool,
    state: CanvasState,
}

impl Into<Room> for ServerRoom {
    fn into(self) -> Room {
        Room {
            uid: self.uid,
            users: self.users.iter().map(|e| e.user.clone()).collect(),
            public: self.public,
            state: self.state,
        }
    }
}

impl Into<Room> for &ServerRoom {
    fn into(self) -> Room {
        Room {
            uid: self.uid,
            users: self.users.iter().map(|e| e.user.clone()).collect(),
            public: self.public,
            state: self.state.clone(),
        }
    }
}

impl ServerRoom {
    pub fn new(public: bool, existing_id: Option<Uuid>) -> Self {
        Self {
            uid: existing_id.unwrap_or(Uuid::new_v4()),
            users: Vec::new(),
            public,
            state: CanvasState::new(),
        }
    }

    async fn send_message_to_all(&mut self, event: ServerEvent, except: Option<Uuid>) {
        let payload = event.to_websocket_message();
        let sendable_users: Vec<&mut ServerUser> = match except {
            None => self.users.iter_mut().collect(),
            Some(id) => self.users.iter_mut().filter(|e| e.user.uid != id).collect(),
        };
        for user in sendable_users {
            if let Err(err) = user.sink.send(payload.clone()).await {
                eprintln!(
                    "Failed to send message {:?} in room {} to user {}",
                    event, &self.uid, &user.user.uid
                );
                eprintln!("The initial error was: {:?}", err);
                continue;
            }
        }
    }

    pub async fn add_user(&mut self, user: ServerUser) {
        let event = ServerEvent::UserJoin(user.user.clone());
        self.send_message_to_all(event, None).await;
        self.users.push(user);
    }

    pub async fn remove_user(&mut self, user: Uuid) {
        let event = ServerEvent::UserLeft(user);
        println!("User left: ");
        self.users.retain(|e| e.user.uid != user);
        self.send_message_to_all(event, None).await;
    }

    pub async fn update_cursor_position(&mut self, user: Uuid, cursor_coordinate: Coordinate) {
        let event = ServerEvent::UserCursorPositionUpdate {
            user_id: user,
            coordinate: cursor_coordinate,
        };
        self.send_message_to_all(event, Some(user)).await;
    }

    pub async fn draw_line(&mut self, user: Uuid, line: Line) {
        let user_line = UserLine {
            user_id: user,
            line,
        };
        let event = ServerEvent::DrawLine(user_line.clone());
        self.state.add_user_line(user_line);
        self.send_message_to_all(event, Some(user)).await;
        println!(
            "The size of the current state is: {:?}",
            &self.state.get_memory_size()
        );
    }
}

#[derive(Debug)]
pub struct Rooms {
    inner: Vec<ServerRoom>,
}

impl Rooms {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub async fn create_room(&mut self, public: bool, existing_id: Option<Uuid>) {
        self.inner.push(ServerRoom::new(public, existing_id));
    }

    pub async fn remove_room(&mut self, room_id: Uuid) {
        let pos = self.inner.iter().position(|e| e.uid == room_id);
        if let None = pos {
            return;
        }
        let pos = pos.unwrap();
        self.inner.remove(pos);
    }

    pub async fn add_user(&mut self, room_id: Uuid, user: ServerUser) {
        if let Some(room) = self.inner.iter_mut().find(|e| e.uid == room_id) {
            room.add_user(user).await;
        }
    }

    pub async fn remove_user(&mut self, room_id: Uuid, user_id: Uuid) {
        if let Some(room) = self.inner.iter_mut().find(|e| e.uid == room_id) {
            room.remove_user(user_id).await;
            if room.users.is_empty() {
                self.remove_room(room_id).await;
            }
        }
    }

    pub async fn update_cursor_position(
        &mut self,
        room_id: Uuid,
        user_id: Uuid,
        coordinate: Coordinate,
    ) {
        if let Some(room) = self.inner.iter_mut().find(|e| e.uid == room_id) {
            room.update_cursor_position(user_id, coordinate).await;
        }
    }

    pub async fn draw_line(&mut self, room_id: Uuid, user_id: Uuid, line: Line) {
        if let Some(room) = self.inner.iter_mut().find(|e| e.uid == room_id) {
            room.draw_line(user_id, line).await;
        }
    }

    pub async fn get_room_by_uid(&self, uid: Uuid) -> Result<Room, CanvasError> {
        self.inner
            .iter()
            .find(|e| e.uid == uid)
            .map(|e| e.into())
            .ok_or(CanvasError::RoomNotFound)
    }

    pub async fn get_public_rooms(&self) -> Vec<Room> {
        self.inner
            .iter()
            .filter(|e| e.public)
            .map(|e| e.into())
            .collect::<Vec<Room>>()
    }
}

pub static ROOMS: Lazy<Mutex<Rooms>> = Lazy::new(|| Mutex::new(Rooms::new()));
