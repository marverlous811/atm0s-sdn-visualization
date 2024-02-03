use atm0s_sdn_identity::{ConnId, NodeAddr, NodeId};
use atm0s_sdn_network::transport::ConnectionStats;
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use crate::identity::{ConnectionMetric, ConnectionStatus};

use super::{
    msg::VisualizationAgentMsg,
    storage::{ConnectionModifyData, ConnectionNode, ConnectionStorage},
};

pub struct VisualizationAgentLogic {
    node_id: NodeId,
    node_addr: NodeAddr,
    msg_queue: VecDeque<VisualizationAgentMsg>,
    storage: ConnectionStorage,
}

impl VisualizationAgentLogic {
    pub fn new(node_id: NodeId, node_addr: NodeAddr) -> Self {
        Self {
            node_id: node_id,
            node_addr: node_addr,
            msg_queue: VecDeque::new(),
            storage: ConnectionStorage::new(),
        }
    }

    pub fn report_stats(&mut self, now_ms: u64) {}

    pub fn on_node_connected(&mut self, conn_id: ConnId, node_id: NodeId, addr: NodeAddr, now: u64) {
        self.storage.new_connection(conn_id, node_id, addr, now);
    }

    pub fn on_node_disconnected(&mut self, conn_id: ConnId, now: u64) {
        self.storage.update_connection_data(
            conn_id,
            ConnectionModifyData {
                status: Some(ConnectionStatus::DISCONNECTED),
                metric: None,
                latest_updated_at: now,
            },
        );
    }

    pub fn on_connection_stats(&mut self, conn_id: ConnId, metric: ConnectionMetric, now: u64) {
        self.storage.update_connection_data(
            conn_id,
            ConnectionModifyData {
                status: None,
                metric: Some(metric),
                latest_updated_at: now,
            },
        );
    }

    pub fn pop_msg(&mut self) -> Option<VisualizationAgentMsg> {
        self.msg_queue.pop_front()
    }
}
