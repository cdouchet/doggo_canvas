use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UsernameUpdate {
    pub uid: Uuid,
    pub username: String,
}
