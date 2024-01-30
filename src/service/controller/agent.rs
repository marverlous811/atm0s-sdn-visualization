use std::collections::VecDeque;

use atm0s_sdn_identity::{NodeAddr, NodeId};

use crate::service::msg::VisualizationControllAction;

use super::IVisializationController;

pub struct NodeAgent {
    node_id: NodeId,
    node_addr: NodeAddr,
    queue_action: VecDeque<VisualizationControllAction>,
}

impl NodeAgent {
    pub fn new(node_id: NodeId, node_addr: NodeAddr) -> Self {
        Self {
            node_id: node_id,
            node_addr: node_addr,
            queue_action: VecDeque::new(),
        }
    }
}

impl IVisializationController for NodeAgent {
    fn report_stats(&mut self, ts: u64) {
        let action = VisualizationControllAction::NodeStats(self.node_id, self.node_addr.clone(), ts);
        self.queue_action.push_back(action);
    }

    fn on_node_connected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, dest_addr: NodeAddr, now: u64) {
        let action = VisualizationControllAction::OnNodeConnected(src_id, src_addr, dest_id, dest_addr, now);
        self.queue_action.push_back(action)
    }

    fn on_node_disconnected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, dest_addr: NodeAddr, now: u64) {
        let action = VisualizationControllAction::OnNodeDisconencted(src_id, src_addr, dest_id, dest_addr, now);
        self.queue_action.push_back(action)
    }

    fn pop_action(&mut self) -> Option<VisualizationControllAction> {
        self.queue_action.pop_front()
    }

    fn execute_action(&mut self, _action: VisualizationControllAction) {}

    fn dump_graph(&self) {}
}
