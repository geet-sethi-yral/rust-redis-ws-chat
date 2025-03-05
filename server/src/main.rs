mod handlers;
mod state;

use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    // attach handlers
    socket.on("join", handlers::handle_join);
    socket.on("message", handlers::message);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let store = state::MessageStore::new();

    let (layer, io) = SocketIo::builder().with_state(store).build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/health", get(|| async { "ok" }))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server");

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
