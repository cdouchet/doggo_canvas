use futures::stream::SplitSink;
use models::user::User;
use warp::filters::ws::{Message, WebSocket};

#[derive(Debug)]
pub struct ServerUser {
    pub user: User,
    pub sink: SplitSink<WebSocket, Message>,
}

impl Into<User> for ServerUser {
    fn into(self) -> User {
        self.user
    }
}
