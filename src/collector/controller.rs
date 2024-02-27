use std::sync::Arc;

use atm0s_sdn_identity::NodeId;
use parking_lot::RwLock;

use super::storage::{NodeConnectionData, NodeConnectionStorage, NodeData};

pub struct SdnMonitorController {
    node_storage: Arc<RwLock<NodeConnectionStorage>>,
}

impl Clone for SdnMonitorController {
    fn clone(&self) -> Self {
        Self {
            node_storage: self.node_storage.clone(),
        }
    }
}

impl SdnMonitorController {
    pub fn new() -> SdnMonitorController {
        Self {
            node_storage: Arc::new(RwLock::new(NodeConnectionStorage::new())),
        }
    }

    pub fn upsert_node(&mut self, node_id: NodeId, addr: String, now_ms: u64) {
        self.node_storage.write().upsert_node(node_id, addr, now_ms);
    }

    pub fn update_node_conns(&mut self, node_id: NodeId, conns: Vec<NodeConnectionData>) {
        self.node_storage.write().update_node_connection(node_id, conns);
    }

    pub fn get_nodes(&self) -> Vec<NodeData> {
        self.node_storage.read().list_node()
    }
}
