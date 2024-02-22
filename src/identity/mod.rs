mod conn;

use serde::{Deserialize, Serialize};

pub use conn::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    DISCONNECTED = 0,
    CONNECTED = 1,
}

impl ConnectionStatus {
    pub fn to_bytes(&self) -> u8 {
        match self {
            ConnectionStatus::CONNECTED => 1,
            ConnectionStatus::DISCONNECTED => 0,
        }
    }
}

pub const CONNECTION_TIMEOUT_MS: u64 = 1000 * 60 * 2;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ConnectionMetric {
    pub latency: u16,      // in milisec
    pub bandwidth: u32,    // kps
    pub loss_percent: u32, // percentage of package loss
}
