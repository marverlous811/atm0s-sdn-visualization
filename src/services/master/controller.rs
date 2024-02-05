use atm0s_sdn_identity::NodeId;
use atm0s_sdn_utils::hashmap::HashMap;

use super::storage::{NodeConnectionStorage, TransportData};

pub struct VisualizationController {
    node_storage: NodeConnectionStorage,
}

impl VisualizationController {
    pub fn new() -> VisualizationController {
        Self {
            node_storage: NodeConnectionStorage::new(),
        }
    }

    pub fn upsert_node(&mut self, node_id: NodeId, addr: String, now_ms: u64) {
        self.node_storage.upsertNode(node_id, addr, now_ms);
    }

    pub fn update_node_conns(&mut self, addr: String, conns: Vec<TransportData>) {
        self.node_storage.updateNodeConnection(addr, conns);
    }

    pub fn dump_print(&self) {
        self.node_storage.print_dump();
    }
}
