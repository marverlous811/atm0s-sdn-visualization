use atm0s_sdn_identity::NodeId;
use atm0s_sdn_utils::hashmap::HashMap;
use log::error;
use serde::{Deserialize, Serialize};

use crate::identity::{ConnectionMetric, ConnectionStatus};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NodeConnectionData {
    pub id: u64,
    pub node_id: NodeId,
    pub protocol: u8,
    pub addr: String,
    pub metric: ConnectionMetric,
    pub status: ConnectionStatus,
    pub last_updated_at: u64,
    pub direction: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub id: NodeId,
    pub addr: String,
    pub last_ping_ts: u64,
    pub conns: Vec<NodeConnectionData>,
}

impl NodeData {
    pub fn new(node_id: NodeId, addr: String, last_ping_ts: u64) -> NodeData {
        Self {
            id: node_id,
            addr,
            last_ping_ts,
            conns: vec![],
        }
    }

    pub fn dump(&self) {
        println!("===================================================================");
        println!("Node info: id {}, addr: {}, last_ping: {}", self.id, self.addr, self.last_ping_ts);
        for conn in self.conns.iter() {
            println!(
                "- dest conn_id: {}, node: {}, dest addr: {}, direction: {}, status: {}, latency: {}ms, bandwidth: {}kbps, loss: {}%, last updated at: {}",
                conn.id,
                conn.node_id,
                conn.addr,
                conn.direction,
                conn.status.to_bytes(),
                conn.metric.latency,
                conn.metric.bandwidth,
                conn.metric.loss_percent,
                conn.last_updated_at,
            );
        }
    }
}

pub struct NodeConnectionStorage {
    nodes: HashMap<NodeId, NodeData>,
}

impl NodeConnectionStorage {
    pub fn new() -> NodeConnectionStorage {
        Self { nodes: HashMap::new() }
    }

    pub fn upsert_node(&mut self, node_id: NodeId, addr: String, last_ping_ts: u64) {
        match self.nodes.get_mut(&node_id) {
            Some(node) => {
                if last_ping_ts > node.last_ping_ts {
                    node.last_ping_ts = last_ping_ts;
                }
            }
            None => {
                let node = NodeData::new(node_id, addr, last_ping_ts);
                self.nodes.insert(node_id, node);
            }
        }
    }

    pub fn update_node_connection(&mut self, node_id: NodeId, conns: Vec<NodeConnectionData>) {
        match self.nodes.get_mut(&node_id) {
            Some(node) => {
                let mut tmp = HashMap::<u64, NodeConnectionData>::new();
                while let Some(conn) = node.conns.pop() {
                    tmp.insert(conn.id, conn);
                }
                for conn in conns {
                    match tmp.get_mut(&conn.id) {
                        Some(conn_tmp) => {
                            conn_tmp.metric = conn.metric;
                            conn_tmp.status = conn.status;
                            conn_tmp.last_updated_at = conn.last_updated_at;
                        }
                        None => {
                            tmp.insert(conn.id, conn.clone());
                        }
                    }
                }
                for (_, data) in tmp.iter() {
                    node.conns.push(data.clone());
                }
            }
            None => {
                error!("[VisualizationMaster][NodeConnectionStorage] node not found");
            }
        };
    }

    pub fn list_node(&self) -> Vec<NodeData> {
        self.nodes.values().into_iter().map(|data| data.clone()).collect()
    }

    pub fn get_node(&self, id: NodeId) -> Option<NodeData> {
        match self.nodes.get(&id) {
            Some(node) => Some(node.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_upsert_node_adds_new_node_transport_data_if_address_not_present() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts = 123456789;

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts);

        assert_eq!(storage.nodes.len(), 1);
        assert_eq!(storage.nodes.get(&node_id), Some(&NodeData::new(node_id, addr, last_ping_ts)));
    }

    #[test]
    fn test_upsert_node_updates_last_ping_ts_if_address_already_present_and_last_ping_ts_greater() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts1 = 123456789;
        let last_ping_ts2 = 987654321;

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts1);
        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts2);

        assert_eq!(storage.nodes.len(), 1);
        assert_eq!(storage.nodes.get(&node_id), Some(&NodeData::new(node_id, addr, last_ping_ts2)));
    }

    #[test]
    fn test_upsert_node_does_not_update_last_ping_ts_if_address_already_present_and_last_ping_ts_less_or_equal() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts1 = 123456789;
        let last_ping_ts2 = 987654321;

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts1);
        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts2);
        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts1);

        assert_eq!(storage.nodes.len(), 1);
        assert_eq!(storage.nodes.get(&node_id), Some(&NodeData::new(node_id, addr, last_ping_ts2)));
    }

    #[test]
    fn test_update_node_connection_updates_fields_of_existing_transport_data_if_address_and_transport_data_match() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts = 123456789;
        let conn1 = NodeConnectionData {
            id: 1,
            node_id: node_id.clone(),
            protocol: 1,
            addr: addr.clone(),
            metric: ConnectionMetric {
                latency: 1,
                loss_percent: 0,
                bandwidth: 100,
            },
            status: ConnectionStatus::CONNECTED,
            last_updated_at: 0,
            direction: 0,
        };
        let conn2 = NodeConnectionData {
            id: 1,
            node_id: node_id.clone(),
            protocol: 1,
            addr: addr.clone(),
            metric: ConnectionMetric {
                latency: 2,
                loss_percent: 1,
                bandwidth: 100,
            },
            status: ConnectionStatus::DISCONNECTED,
            last_updated_at: 987654321,
            direction: 0,
        };

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts);
        storage.update_node_connection(node_id, vec![conn1.clone()]);
        storage.update_node_connection(node_id, vec![conn2.clone()]);

        assert_eq!(storage.nodes.len(), 1);
        assert_eq!(
            storage.nodes.get(&node_id),
            Some(&NodeData {
                id: node_id,
                addr: addr.clone(),
                last_ping_ts,
                conns: vec![conn2],
            })
        );
    }

    #[test]
    fn test_update_node_connection_removes_all_existing_transport_data_if_address_present_and_connections_empty() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts = 123456789;
        let conn = NodeConnectionData {
            id: 1,
            node_id: node_id.clone(),
            protocol: 1,
            addr: addr.clone(),
            metric: ConnectionMetric {
                latency: 1,
                loss_percent: 0,
                bandwidth: 100,
            },
            status: ConnectionStatus::CONNECTED,
            last_updated_at: 0,
            direction: 0,
        };

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts);
        storage.update_node_connection(node_id, vec![conn.clone()]);
        storage.update_node_connection(node_id, vec![]);

        assert_eq!(storage.nodes.len(), 1);
        assert_eq!(
            storage.nodes.get(&node_id),
            Some(&NodeData {
                id: node_id,
                addr: addr.clone(),
                last_ping_ts,
                conns: vec![conn],
            })
        );
    }

    #[test]
    fn test_update_node_connection_does_nothing_if_address_not_present() {
        let mut storage = NodeConnectionStorage::new();
        let addr = String::from("127.0.0.1");
        let conn = NodeConnectionData {
            id: 1,
            protocol: 1,
            node_id: 1,
            addr: addr.clone(),
            metric: ConnectionMetric {
                latency: 1,
                loss_percent: 0,
                bandwidth: 100,
            },
            status: ConnectionStatus::CONNECTED,
            last_updated_at: 0,
            direction: 0,
        };

        storage.update_node_connection(1, vec![conn]);

        assert_eq!(storage.nodes.len(), 0);
    }
}
