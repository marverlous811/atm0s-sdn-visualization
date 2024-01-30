use std::collections::VecDeque;

use atm0s_sdn_identity::{NodeAddr, NodeId};

use crate::service::{
    msg::VisualizationControllAction,
    store::{NetworkGraph, NetworkGraphEdgeData},
};

use super::IVisializationController;

pub struct NodeStatsCollector {
    node_id: NodeId,
    node_addr: NodeAddr,
    store: NetworkGraph,
    queue_action: VecDeque<VisualizationControllAction>,
}

impl NodeStatsCollector {
    pub fn new(node_id: NodeId, node_addr: NodeAddr) -> Self {
        Self {
            node_id: node_id,
            node_addr: node_addr,
            store: NetworkGraph::new(),
            queue_action: VecDeque::new(),
        }
    }
}

impl IVisializationController for NodeStatsCollector {
    fn report_stats(&mut self, ts: u64) {
        let action = VisualizationControllAction::NodeStats(self.node_id, self.node_addr.clone(), ts);
        self.execute_action(action.clone());
        self.queue_action.push_back(action);
    }

    fn pop_action(&mut self) -> Option<VisualizationControllAction> {
        self.queue_action.pop_front()
    }

    fn on_node_connected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, dest_addr: NodeAddr, now: u64) {
        let action = VisualizationControllAction::OnNodeConnected(src_id, src_addr, dest_id, dest_addr, now);
        if src_id == self.node_id || dest_id == self.node_id {
            self.execute_action(action.clone())
        }
        self.queue_action.push_back(action)
    }

    fn on_node_disconnected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, dest_addr: NodeAddr, now: u64) {
        let action = VisualizationControllAction::OnNodeDisconencted(src_id, src_addr, dest_id, dest_addr, now);
        if src_id == self.node_id || dest_id == self.node_id {
            self.execute_action(action.clone())
        }
        self.queue_action.push_back(action)
    }

    fn execute_action(&mut self, action: VisualizationControllAction) {
        match action {
            VisualizationControllAction::NodeStats(node_id, node_addr, ts) => {
                // println!("[NodeStatsCollector] got a stats from node {}, with addr {}, at {}", node_id, node_addr, ts);
                self.store.upsert_node(node_id, node_addr, ts);
            }
            VisualizationControllAction::OnNodeConnected(src_node_id, src_node_addr, dest_node_id, dest_node_addr, ts) => {
                let node_edge_data = NetworkGraphEdgeData::new(src_node_id, src_node_addr, dest_node_id, dest_node_addr, ts);
                // println!("[NodeStatsCollector] node {} connected to {} at {}", src_node_id, dest_node_id, ts);
                self.store.add_edge(node_edge_data)
            }
            VisualizationControllAction::OnNodeDisconencted(src_node_id, src_node_addr, dest_node_id, dest_node_addr, ts) => {
                let node_edge_data = NetworkGraphEdgeData::new(src_node_id, src_node_addr, dest_node_id, dest_node_addr, ts);
                // println!("[NodeStatsCollector] node {} disconnected to {} at {}", src_node_id, dest_node_id, ts);
                self.store.remove_edge(node_edge_data)
            }
        };
    }

    fn dump_graph(&self) {
        self.store.dump_print();
    }
}
