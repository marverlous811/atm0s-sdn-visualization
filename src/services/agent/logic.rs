use std::collections::HashSet;

use atm0s_sdn_identity::{NodeAddr, NodeId};
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use super::msg::VisualizationAgentMsg;

pub struct VisualizationAgentLogic {
    node_id: NodeId,
    node_addr: NodeAddr,
    neighbour_ids: Vec<NodeId>,
    msg_queue: VecDeque<VisualizationAgentMsg>,
}

impl VisualizationAgentLogic {
    pub fn new(node_id: NodeId, node_addr: NodeAddr) -> Self {
        Self {
            node_id: node_id,
            node_addr: node_addr,
            neighbour_ids: Vec::new(),
            msg_queue: VecDeque::new(),
        }
    }

    pub fn report_stats(&mut self, now_ms: u64) {
        let msg = VisualizationAgentMsg::NodeStats(self.node_id, self.node_addr.clone().to_vec(), now_ms, Some(self.neighbour_ids.clone()));
        self.msg_queue.push_back(msg);
    }

    pub fn on_node_connected(&mut self, dest_id: NodeId) {
        let mut tmp = self.neighbour_ids.clone();
        tmp.push(dest_id);
        let mut retval = Vec::new();
        let mut unique_vec = HashSet::<NodeId>::new();
        for &id in &tmp {
            if unique_vec.insert(id) {
                retval.push(id);
            }
        }

        self.neighbour_ids = retval;
    }

    pub fn on_node_disconnected(&mut self, dest_id: NodeId) {
        self.neighbour_ids.retain(|&x| x != dest_id)
    }

    pub fn pop_msg(&mut self) -> Option<VisualizationAgentMsg> {
        self.msg_queue.pop_front()
    }
}
