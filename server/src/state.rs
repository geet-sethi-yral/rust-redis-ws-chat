use anyhow::Result;
use socketioxide::extract::SocketRef;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tracing::debug;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Message {
    pub text: String,
    pub user: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct MessageStore {
    pub client: redis::Client,
    active_subscriptions: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl MessageStore {
    pub fn new() -> Self {
        let conn_url = std::env::var("REDIS_URL").unwrap();
        let client = redis::Client::open(conn_url).unwrap();
        Self {
            client,
            active_subscriptions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, room: &str, socket: SocketRef) -> Result<()> {
        // Check if we're already subscribed to this room
        {
            let subscriptions = self.active_subscriptions.lock().unwrap();
            if subscriptions.contains_key(room) {
                return Ok(());
            }
        }
        debug!("subscribing to room: {room:?}");

        let conn = self.client.get_tokio_connection().await?;
        let mut pubsub = conn.into_pubsub();
        pubsub.subscribe(room).await?;

        let room_clone = room.to_string();
        let active_subs = self.active_subscriptions.clone();

        let handle = tokio::spawn(async move {
            let stream = pubsub.on_message();
            tokio::pin!(stream);
            while let Some(msg) = stream.next().await {
                let payload: String = msg.get_payload().unwrap();
                let payload: Message = serde_json::from_str(&payload).unwrap();
                socket
                    .within(room_clone.clone())
                    .emit("message", &payload)
                    .await
                    .unwrap();
            }

            // Clean up subscription when the task ends
            let mut subscriptions = active_subs.lock().unwrap();
            subscriptions.remove(&room_clone);
        });

        // Store the task handle for potential abort later
        {
            let mut subscriptions = self.active_subscriptions.lock().unwrap();
            subscriptions.insert(room.to_string(), handle);
        }

        Ok(())
    }

    pub async fn unsubscribe(&self, room: &str) -> Result<()> {
        let mut subscriptions = self.active_subscriptions.lock().unwrap();
        if let Some(handle) = subscriptions.remove(room) {
            // Abort the subscription task
            handle.abort();
        }
        Ok(())
    }

    pub async fn insert(&self, room: &str, message: Message) -> Result<()> {
        debug!("publishing message: {message:?}");
        let mut conn = self.client.get_tokio_connection().await?;
        let serialized_msg = serde_json::to_string(&message)?;
        let _: () = redis::pipe()
            .atomic()
            .publish(room, serialized_msg)
            .query_async(&mut conn)
            .await?;

        Ok(())
    }

    // pub async fn get(&self, room: &str) -> Result<Vec<Message>> {
    //     let mut conn = self.client.get_tokio_connection().await?;
    //     let messages: Option<Vec<String>> = redis::cmd("LRANGE")
    //         .arg(room)
    //         .arg(0)
    //         .arg(-1)
    //         .query_async(&mut conn)
    //         .await?;
    //     let messages = messages.unwrap_or_default();
    //     let messages = messages
    //         .into_iter()
    //         .map(|msg| serde_json::from_str::<Message>(&msg))
    //         .collect::<Result<Vec<Message>, _>>()?;

    //     let mut sorted_messages = messages.clone();
    //     sorted_messages.sort_by_key(|m| m.date);

    //     Ok(sorted_messages)
    // }
}
