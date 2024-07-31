use handlers::WebSocketQuery;
use routes::{
    get_public_rooms::get_public_rooms, get_room_state::get_room_state,
    websocket_route::websocket_route,
};
use uuid::Uuid;
use warp::Filter;

pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::query::<WebSocketQuery>())
        .and_then(websocket_route);

    let index_route = warp::path("web").and(warp::fs::dir("web"));

    let public_rooms_route = warp::path("rooms").and_then(get_public_rooms);
    let get_room_state_route = warp::path!("rooms" / Uuid)
        .and(warp::get())
        .and_then(get_room_state);

    let cors = warp::cors().allow_any_origin().build();

    let routes = index_route
        .or(ws_route)
        .or(get_room_state_route)
        .or(public_rooms_route)
        .with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 4576)).await;
}
