use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Messages from clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Join { name: String },
    Disconnect {},
    Player(PlayerState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    player_id: u64,
    position_x: i32,
    position_y: i32,
    orientation_degrees: u32,
    strafing: i8, // -1, 0, 1 left/not/right
    turning: i8,  // -1, 0, 1, left/not/right
    forward: i8,  // -1, 0, 1, backward/not/forward
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
    Entities {
        players: Vec<PlayerState>,
    },
    InitClient {
        client_id: u64,
        usernames: HashMap<u64, String>,
    },
}
