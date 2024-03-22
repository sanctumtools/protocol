use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Messages from clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Join { name: String },
    Disconnect {},
    ChatMessage { message: String },
}

// Messages from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    ClientConnected {
        client_id: u64,
        username: String,
    },
    ClientDisconnected {
        client_id: u64,
    },
    ChatMessage {
        client_id: u64,
        message: String,
    },
    InitClient {
        client_id: u64,
        usernames: HashMap<u64, String>,
    },
}
