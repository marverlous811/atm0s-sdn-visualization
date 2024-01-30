use std::collections::VecDeque;

use atm0s_sdn_identity::{NodeAddr, NodeId};
use atm0s_sdn_utils::hashmap::HashMap;

use crate::service::{msg::VisualizationControllAction, store::NetworkNodeData};

use super::IVisializationController;

pub struct NodeAgent {
    node_id: NodeId,
    node_addr: NodeAddr,
    neighbours: HashMap<NodeId, NetworkNodeData>,
    queue_action: VecDeque<VisualizationControllAction>,
}

impl NodeAgent {
    pub fn new(node_id: NodeId, node_addr: NodeAddr) -> Self {
        Self {
            node_id: node_id,
            node_addr: node_addr,
            neighbours: HashMap::new(),
            queue_action: VecDeque::new(),
        }
    }
}

impl IVisializationController for NodeAgent {
    fn report_stats(&mut self, ts: u64) {
        let neighbour_ids: Vec<NodeId> = self.neighbours.iter().map(|(&id, _)| id).collect();
        let action = VisualizationControllAction::NodeStats(self.node_id, self.node_addr.clone(), ts, Some(neighbour_ids));
        self.queue_action.push_back(action);
    }

    fn on_node_connected(&mut self, node_id: NodeId, addr: NodeAddr, _now: u64) {
        self.neighbours.insert(node_id, NetworkNodeData::new(node_id, addr));
    }

    fn on_node_disconnected(&mut self, node_id: NodeId) {
        self.neighbours.remove(&node_id);
    }

    fn pop_action(&mut self) -> Option<VisualizationControllAction> {
        self.queue_action.pop_front()
    }

    fn execute_action(&mut self, _action: VisualizationControllAction) {}

    fn dump_graph(&self) {}
}
