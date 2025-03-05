use socketioxide::extract::{Data, SocketRef, State};
use tracing::info;

use super::state;

#[derive(Debug, serde::Deserialize)]
pub struct MessageIn {
    pub room: String,
    pub text: String,
}

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
    let messages = store.get(&room).await;
    let _ = socket.emit("messages", &Messages { messages });
}

pub async fn message(
    socket: SocketRef,
    Data(data): Data<MessageIn>,
    store: State<state::MessageStore>,
) {
    info!("Received message: {data:?}");

    let response = state::Message {
        text: data.text,
        user: format!("anon-{}", socket.id),
        date: chrono::Utc::now(),
    };

    store.insert(&data.room, response.clone()).await;

    let _ = socket.within(data.room).emit("message", &response).await;
}
