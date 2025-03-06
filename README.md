# Rust + Redis - WebSocket Based Chat

A real-time chat application built with Rust using Redis and WebSockets. This application demonstrates how to implement a scalable chat system using Rust as a web socket server combined with Redis' pub/sub for distributing messages between instances of the rust web socket server.

## Key Features

- Real-time messaging using WebSockets
- Redis pub/sub for message distribution across multiple server instances

## System Architecture

The application follows a layered architecture:
- **WebSocket Server (Rust)**: Handles client connections and WebSocket communication
- **Redis Backend**: Manages message distribution of messages to all subscribed (rust) servers
- **Web Interface (Next.js)**: Provides a simple chat UI for users

## Setup and Installation

1. Make sure Redis is installed and running on your system
1. Clone the repository
1. Run the application:

```bash
# running the server
cd server
cargo run

# running the client
cd client
pnpm dev
```
1. Visit the web app on [localhost:3000](http://localhost:3000)

## Configuration

The application can be configured through environment variables or a `.env` file:

- `REDIS_URL`: Redis connection string (default: "redis://localhost:6379/")
- `PORT`: WebSocket server port (default: "4000")
- `LOG_LEVEL`: Logging level (default: "debug")
