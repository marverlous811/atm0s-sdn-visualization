use atm0s_sdn_identity::NodeId;

use super::storage::{NetworkNodeData, NodeConnectionStorage, TransportConnectionData};

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
        self.node_storage.upsert_node(node_id, addr, now_ms);
    }

    pub fn update_node_conns(&mut self, addr: String, conns: Vec<TransportConnectionData>) {
        self.node_storage.update_node_connection(addr, conns);
    }

    pub fn get_nodes(&self) -> Vec<NetworkNodeData> {
        self.node_storage.list_node()
    }

    pub fn dump_print(&self) {
        let nodes = self.node_storage.list_node();
        for node in nodes.iter() {
            node.dump();
        }
    }
}
