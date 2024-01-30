use std::collections::VecDeque;

use atm0s_sdn_identity::{NodeAddr, NodeId};
use log::debug;

use crate::service::{msg::VisualizationControllAction, store::NetworkGraph};

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
        self.store.upsert_node(self.node_id, self.node_addr.clone(), ts, None);
        // Send stats to other collector
        let stats = self.store.get_node(self.node_id);
        match stats {
            Some(stats) => {
                let action = VisualizationControllAction::NodeStats(self.node_id, self.node_addr.clone(), ts, Some(stats.node_neighbors.clone()));
                self.queue_action.push_back(action)
            }
            None => {
                panic!("current node must be exist in its graph")
            }
        }
    }

    fn pop_action(&mut self) -> Option<VisualizationControllAction> {
        self.queue_action.pop_front()
    }

    fn on_node_connected(&mut self, dest_id: NodeId, dest_addr: NodeAddr, now: u64) {
        self.store.upsert_node(dest_id, dest_addr, now, None);
        self.store.add_edge(self.node_id, dest_id);
    }

    fn on_node_disconnected(&mut self, dest_id: NodeId) {
        self.store.remove_edge(self.node_id, dest_id);
    }

    fn execute_action(&mut self, action: VisualizationControllAction) {
        match action {
            VisualizationControllAction::NodeStats(node_id, node_addr, ts, neighbour_ids) => {
                debug!("[VisualizationService][NodeStatsCollector] got a stats from node {}, with addr {}, at {}", node_id, node_addr, ts);
                self.store.upsert_node(node_id, node_addr, ts, neighbour_ids);
            }
        };
    }

    fn dump_graph(&self) {
        self.store.dump_print();
    }
}
