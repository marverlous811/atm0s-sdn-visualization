use atm0s_sdn_identity::{ConnId, NodeAddr, NodeId};
use atm0s_sdn_utils::hashmap::HashMap;
use log::{debug, error};

use crate::identity::{generate_connection_id, ConnectionMetric, ConnectionStatus};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConnectionNode {
    pub uuid: u64,
    pub protocol: u8,
    pub node_id: NodeId,
    pub addr: String,
    pub direction: u8,
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
        let uuid = generate_connection_id(id.protocol(), id.direction(), node_id);
        match self.conns.get_mut(&uuid) {
            Some(node) => {
                node.status = ConnectionStatus::CONNECTED;
                node.latest_updated_at = now
            }
            None => {
                self.conns.insert(
                    uuid,
                    ConnectionNode {
                        uuid,
                        protocol: id.protocol(),
                        node_id,
                        addr: addr.to_string(),
                        direction: id.direction().to_byte(),
                        status: ConnectionStatus::CONNECTED,
                        metric: None,
                        latest_updated_at: now,
                    },
                );
            }
        }
    }

    pub fn update_connection_data(&mut self, id: u64, data: ConnectionModifyData) -> bool {
        match self.conns.get_mut(&id) {
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

    pub fn list_conns(&self) -> Vec<ConnectionNode> {
        let mut ret_val = Vec::<ConnectionNode>::new();
        for (_, conn) in self.conns.iter() {
            ret_val.push(conn.clone());
        }
        ret_val
    }
}
