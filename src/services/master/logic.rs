use std::sync::Arc;

use atm0s_sdn_identity::NodeAddr;
use parking_lot::RwLock;

use crate::VisualizationAgentMsg;

use super::controller::{NetworkNode, VisualizationController};

pub struct VisualizationMasterLogic {
    controller: Arc<RwLock<Box<VisualizationController>>>,
}

impl Clone for VisualizationMasterLogic {
    fn clone(&self) -> Self {
        Self { controller: self.controller.clone() }
    }
}

impl VisualizationMasterLogic {
    pub fn new() -> Self {
        let controller = Arc::new(RwLock::new(Box::new(VisualizationController::new())));
        Self { controller: controller }
    }

    pub fn process_agent_msg(&mut self, msg: VisualizationAgentMsg) {
        match msg {
            VisualizationAgentMsg::NodeStats(id, addr_vec, ts, neighbour_ids) => {
                let neighbours = match neighbour_ids {
                    Some(data) => data,
                    None => Vec::new(),
                };
                if let Some(addr) = NodeAddr::from_vec(&addr_vec) {
                    let node = NetworkNode {
                        id,
                        addr,
                        latest_ping: ts,
                        neighbour_ids: neighbours,
                    };
                    self.controller.write().upsert_node_stats(node)
                }
            }
        }
    }

    pub fn dump_graph(self) {
        self.controller.read().dump_print();
    }
}
