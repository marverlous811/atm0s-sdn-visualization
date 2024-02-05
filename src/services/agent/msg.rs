use atm0s_sdn_identity::{ConnId, NodeId};
use serde::{Deserialize, Serialize};

use crate::identity::{ConnectionMetric, ConnectionStatus};

pub const MAX_CONN_STATS_SEND: usize = 10;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ConnectionMsg {
    pub addr: String,
    pub node_id: NodeId,
    pub direction: u8,
    pub status: ConnectionStatus,
    pub metric: ConnectionMetric,
    pub latest_updated_at: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationAgentBehaviourEvent {
    ConnectionStats(ConnId, ConnectionMetric),
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationAgentHandlerEvent {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum VisualizationAgentMsg {
    // node_id, address, timestamp
    NodePing(NodeId, String, u64),

    // node_address, list connections with length not greater than 20
    NodeConnections(String, Vec<ConnectionMsg>),
}
