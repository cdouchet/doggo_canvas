use serde::Serialize;
use warp::{
    reject::Reject,
    reply::{Reply, Response},
};

#[derive(Debug, Serialize)]
pub enum CanvasError {
    RoomNotFound,
    PixelDeserializationError(String),
}

impl Reply for CanvasError {
    fn into_response(self) -> warp::reply::Response {
        Response::new(serde_json::to_string(&self).unwrap().into())
    }
}
