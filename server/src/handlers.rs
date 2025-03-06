use socketioxide::extract::{Data, SocketRef, State};
use tracing::info;

use super::state;

pub type MessageIn = String;

#[derive(Debug, serde::Serialize)]
pub struct Messages {
    pub messages: Vec<state::Message>,
}

pub async fn handle_join(
    socket: SocketRef,
    Data(room): Data<String>,
    store: State<state::MessageStore>,
) {
    info!("Received join: {room:?}");

    socket.leave_all();
    socket.join(room.clone());

    // Subscribe to new messages for this room
    store.subscribe(&room, socket).await.unwrap();

    // TODO: Handle disconnection to unsubscribe if last user
    // let room_size = socket.within(room).sockets().len();
    // socket.on_disconnect(|socket: SocketRef| async move {
    //     if room_size == 0 {
    //         store.unsubscribe(&room);
    //     }
    // });
}

pub async fn message(
    socket: SocketRef,
    Data(msg): Data<MessageIn>,
    store: State<state::MessageStore>,
) {
    info!("Received message: {msg:?}");

    // The user should be in a SINGLE room
    let rooms = socket.rooms();
    let room = rooms.iter().next();
    if room.is_none() {
        return;
    }
    let room = room.unwrap().to_string();

    let response = state::Message {
        text: msg,
        user: format!("anon-{}", socket.id),
        date: chrono::Utc::now(),
    };

    store
        .insert(&room, response.clone())
        .await
        .unwrap_or_else(|e| {
            info!("Failed to insert message: {e:?}");
        });
}
