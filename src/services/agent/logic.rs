use atm0s_sdn_identity::{ConnId, NodeAddr, NodeId};
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use crate::identity::{generate_connection_id, ConnectionMetric, ConnectionStatus};

use super::{
    msg::{ConnectionMsg, VisualizationAgentMsg, MAX_CONN_STATS_SEND},
    storage::{ConnectionModifyData, ConnectionNode, ConnectionStorage},
};

pub struct VisualizationAgentLogic {
    node_id: NodeId,
    node_addr: NodeAddr,
    msg_queue: VecDeque<VisualizationAgentMsg>,
    storage: ConnectionStorage,
}

fn build_conns_stats_msg(id: NodeId, mut conns: Vec<ConnectionNode>) -> Vec<VisualizationAgentMsg> {
    let mut ret_val = Vec::<VisualizationAgentMsg>::new();
    let mut conn_vec_to_send = Vec::<ConnectionMsg>::new();
    while let Some(conn) = conns.pop() {
        match conn.metric {
            Some(metric) => {
                conn_vec_to_send.push(ConnectionMsg {
                    conn_id: conn.uuid,
                    protocol: conn.protocol,
                    addr: conn.addr.to_string(),
                    node_id: conn.node_id,
                    direction: conn.direction,
                    status: conn.status,
                    metric: metric.clone(),
                    latest_updated_at: conn.latest_updated_at,
                });
                if conn_vec_to_send.len() >= MAX_CONN_STATS_SEND {
                    ret_val.push(VisualizationAgentMsg::NodeConnections(id, conn_vec_to_send.clone()));
                    conn_vec_to_send.clear();
                }
            }
            None => {}
        };
    }
    if conn_vec_to_send.len() > 0 {
        ret_val.push(VisualizationAgentMsg::NodeConnections(id, conn_vec_to_send.clone()));
    }
    ret_val
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

    pub fn report_stats(&mut self, now_ms: u64) {
        let ping_msg = VisualizationAgentMsg::NodePing(self.node_id, self.node_addr.to_string(), now_ms);
        self.msg_queue.push_back(ping_msg);

        let mut stats_msgs = build_conns_stats_msg(self.node_id, self.storage.list_conns());
        while let Some(msg) = stats_msgs.pop() {
            self.msg_queue.push_back(msg);
        }
    }

    pub fn on_node_connected(&mut self, conn_id: ConnId, node_id: NodeId, addr: NodeAddr, now: u64) {
        self.storage.new_connection(conn_id, node_id, addr, now);
    }

    pub fn on_node_disconnected(&mut self, conn_id: ConnId, node_id: NodeId, now: u64) {
        let uuid = generate_connection_id(conn_id.protocol(), conn_id.direction(), node_id);
        self.storage.update_connection_data(
            uuid,
            ConnectionModifyData {
                status: Some(ConnectionStatus::DISCONNECTED),
                metric: None,
                latest_updated_at: now,
            },
        );
    }

    pub fn on_connection_stats(&mut self, conn_id: ConnId, node_id: NodeId, metric: ConnectionMetric, now: u64) {
        let uuid = generate_connection_id(conn_id.protocol(), conn_id.direction(), node_id);
        self.storage.update_connection_data(
            uuid,
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

#[cfg(test)]
mod test {
    use std::ops::Index;

    use atm0s_sdn_identity::NodeAddrBuilder;

    use super::*;

    #[test]
    fn should_return_list_of_msgs_with_metrics_when_given_list_of_connections_with_metrics() {
        let node_id = 1;
        let addr_builder = NodeAddrBuilder::new(1);
        let addr = addr_builder.addr();
        let conns: Vec<ConnectionNode> = vec![
            ConnectionNode {
                uuid: 1,
                protocol: 1,
                addr: addr_builder.addr().to_string(),
                node_id: node_id,
                direction: 1,
                status: ConnectionStatus::CONNECTED,
                metric: Some(ConnectionMetric {
                    latency: 1,
                    loss_percent: 0,
                    bandwidth: 100,
                }),
                latest_updated_at: 0,
            },
            ConnectionNode {
                uuid: 1,
                protocol: 1,
                addr: addr_builder.addr().to_string(),
                node_id: node_id,
                direction: 1,
                status: ConnectionStatus::CONNECTED,
                metric: Some(ConnectionMetric {
                    latency: 1,
                    loss_percent: 0,
                    bandwidth: 100,
                }),
                latest_updated_at: 0,
            },
        ];

        let result = build_conns_stats_msg(node_id, conns);

        assert_eq!(result.len(), 1);
        let data = result.index(0).clone();
        match data {
            VisualizationAgentMsg::NodeConnections(id, conns) => {
                assert_eq!(id, node_id);
                assert_eq!(conns.len(), 2);
            }
            _ => {}
        }
    }

    #[test]
    fn should_split_to_multi_msg_if_number_conn_is_greater_than_max() {
        let node_id = 1;
        let addr_builder = NodeAddrBuilder::new(1);
        let addr = addr_builder.addr();
        let mut conns: Vec<ConnectionNode> = Vec::new();
        for i in 0..MAX_CONN_STATS_SEND + 1 {
            conns.push(ConnectionNode {
                uuid: (i + 1) as u64,
                protocol: 1,
                addr: addr_builder.addr().to_string(),
                node_id: node_id,
                direction: 1,
                status: ConnectionStatus::CONNECTED,
                metric: Some(ConnectionMetric {
                    latency: 1,
                    loss_percent: 0,
                    bandwidth: 100,
                }),
                latest_updated_at: 0,
            })
        }
        let result = build_conns_stats_msg(node_id, conns);

        assert_eq!(result.len(), 2);
    }
}
