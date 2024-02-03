use atm0s_sdn_identity::{ConnDirection, ConnId, NodeAddr, NodeId};
use atm0s_sdn_utils::hashmap::HashMap;
use log::{debug, error};

use crate::identity::{ConnectionMetric, ConnectionStatus};

pub struct ConnectionNode {
    pub uuid: u64,
    pub protocol: u8,
    pub node_id: NodeId,
    pub addr: String,
    pub direction: ConnDirection,
    pub status: ConnectionStatus,
    pub metric: Option<ConnectionMetric>,
    pub latest_updated_at: u64,
}

pub struct ConnectionModifyData {
    pub status: Option<ConnectionStatus>,
    pub metric: Option<ConnectionMetric>,
    pub latest_updated_at: u64,
}

pub struct ConnectionStorage {
    conns: HashMap<u64, ConnectionNode>,
}

impl ConnectionStorage {
    pub fn new() -> Self {
        Self { conns: HashMap::new() }
    }

    pub fn new_connection(&mut self, id: ConnId, node_id: NodeId, addr: NodeAddr, now: u64) {
        match self.conns.get_mut(&id.uuid()) {
            Some(node) => {
                node.status = ConnectionStatus::CONNECTED;
                node.latest_updated_at = now
            }
            None => {
                self.conns.insert(
                    id.uuid(),
                    ConnectionNode {
                        uuid: id.uuid(),
                        protocol: id.protocol(),
                        node_id,
                        addr: addr.to_string(),
                        direction: id.direction(),
                        status: ConnectionStatus::CONNECTED,
                        metric: None,
                        latest_updated_at: now,
                    },
                );
            }
        }
    }

    pub fn update_connection_data(&mut self, id: ConnId, data: ConnectionModifyData) -> bool {
        match self.conns.get_mut(&id.uuid()) {
            Some(node) => {
                match data.status {
                    Some(status) => node.status = status,
                    None => {
                        debug!("[VisualizationAgentService][ConnectionStorage] not have status data for update")
                    }
                };
                match data.metric {
                    Some(metric) => node.metric = Some(metric),
                    None => {
                        debug!("[VisualizationAgentService][ConnectionStorage] not have status data for update")
                    }
                };
                node.latest_updated_at = data.latest_updated_at;
                true
            }
            None => {
                error!("[VisualizationAgentService][ConnectionStorage] node not found");
                false
            }
        }
    }
}
