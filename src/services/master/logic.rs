use std::sync::Arc;

use parking_lot::RwLock;

use crate::VisualizationAgentMsg;

use super::{
    controller::VisualizationController,
    storage::{NetworkNodeData, TransportConnectionData},
};

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
            VisualizationAgentMsg::NodePing(node_id, addr, now_ms) => {
                self.controller.write().upsert_node(node_id, addr, now_ms);
            }
            VisualizationAgentMsg::NodeConnections(addr, conns) => {
                let data: Vec<TransportConnectionData> = conns
                    .into_iter()
                    .map(|conn| TransportConnectionData {
                        node_id: conn.node_id,
                        addr: conn.addr,
                        metric: conn.metric.clone(),
                        direction: conn.direction,
                        status: conn.status,
                        last_updated_at: conn.latest_updated_at,
                    })
                    .collect();
                self.controller.write().update_node_conns(addr, data);
            }
        }
    }

    pub fn get_nodes(&self) -> Vec<NetworkNodeData> {
        self.controller.read().get_nodes()
    }

    pub fn dump_graph(self) {
        self.controller.read().dump_print();
    }
}
