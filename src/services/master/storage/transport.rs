use crate::{
    identity::{ConnectionMetric, ConnectionStatus},
    util::calc_hash,
};
use atm0s_sdn_identity::NodeId;
use atm0s_sdn_utils::hashmap::HashMap;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransportConnectionData {
    pub id: u64,
    pub node_id: NodeId,
    pub addr: String,
    pub metric: ConnectionMetric,
    pub status: ConnectionStatus,
    pub last_updated_at: u64,
    pub direction: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeTransportData {
    id: u64,
    node_id: NodeId,
    addr: String,
    last_ping_ts: u64,
    connections: Vec<TransportConnectionData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NetworkTransportData {
    pub id: u64,
    pub addr: String,
    pub last_ping_ts: u64,
    pub connections: Vec<TransportConnectionData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NetworkNodeData {
    pub node_id: NodeId,
    pub transports: Vec<NetworkTransportData>,
}

impl NetworkNodeData {
    pub fn dump(&self) {
        println!("===================================================================");
        println!("Node: {}", self.node_id);
        for trans in self.transports.iter() {
            println!("Transport addr: {}, last ping ts: {}", trans.addr, trans.last_ping_ts);
            println!("List Connection: ");
            for conns in trans.connections.iter() {
                println!(
                    "- dest node: {}, dest addr: {}, direction: {}, status: {}, latency: {}ms, bandwidth: {}kbps, loss: {}%, last updated at: {}",
                    conns.node_id,
                    conns.addr,
                    conns.direction,
                    conns.status.to_bytes(),
                    conns.metric.latency,
                    conns.metric.bandwidth,
                    conns.metric.loss_percent,
                    conns.last_updated_at,
                );
            }
        }
        println!("===================================================================");
    }
}

impl NodeTransportData {
    pub fn new(node_id: NodeId, addr: String, last_ping_ts: u64) -> Self {
        Self {
            id: calc_hash(&addr.clone()),
            node_id,
            addr,
            last_ping_ts,
            connections: Vec::new(),
        }
    }
}

pub struct NodeConnectionStorage {
    transports: HashMap<u64, NodeTransportData>,
}

impl NodeConnectionStorage {
    pub fn new() -> Self {
        Self { transports: HashMap::new() }
    }

    pub fn upsert_node(&mut self, node_id: NodeId, addr: String, last_ping_ts: u64) {
        let id = calc_hash(&addr.clone());
        match self.transports.get_mut(&id) {
            Some(trans) => {
                if last_ping_ts > trans.last_ping_ts {
                    trans.last_ping_ts = last_ping_ts;
                }
            }
            None => {
                let trans = NodeTransportData::new(node_id, addr.clone(), last_ping_ts);
                self.transports.insert(id, trans);
            }
        }
    }

    pub fn update_node_connection(&mut self, addr: String, connections: Vec<TransportConnectionData>) {
        let id = calc_hash(&addr.clone());
        match self.transports.get_mut(&id) {
            Some(trans) => {
                let mut tmp = HashMap::<String, TransportConnectionData>::new();
                while let Some(node) = trans.connections.pop() {
                    tmp.insert(node.addr.clone(), node);
                }
                for conn in connections {
                    match tmp.get_mut(&conn.addr) {
                        Some(trans) => {
                            trans.metric = conn.metric;
                            trans.status = conn.status;
                            trans.last_updated_at = conn.last_updated_at;
                        }
                        None => {
                            tmp.insert(conn.addr.clone(), conn.clone());
                        }
                    }
                }
                for (_, data) in tmp.iter() {
                    trans.connections.push(data.clone());
                }
            }
            None => {
                error!("[VisualizationMaster][NodeConnectionStorage] transporter not found");
            }
        }
    }

    pub fn get_node(&self, id: NodeId) -> NetworkNodeData {
        let mut node_trans: Vec<NetworkTransportData> = self
            .transports
            .iter()
            .filter(|(_, trans)| trans.node_id == id)
            .map(|trans| NetworkTransportData {
                id: trans.1.id,
                addr: trans.1.addr.clone(),
                last_ping_ts: trans.1.last_ping_ts,
                connections: trans.1.connections.clone(),
            })
            .collect();
        node_trans.sort_by(|a, b| a.id.cmp(&b.id));
        NetworkNodeData { node_id: id, transports: node_trans }
    }

    pub fn list_node(&self) -> Vec<NetworkNodeData> {
        let mut node_map = HashMap::<NodeId, NetworkNodeData>::new();
        for (_, trans) in self.transports.iter() {
            match node_map.get_mut(&trans.node_id) {
                Some(node) => {
                    let transport = NetworkTransportData {
                        id: trans.id,
                        addr: trans.addr.clone(),
                        last_ping_ts: trans.last_ping_ts,
                        connections: trans.connections.clone(),
                    };
                    node.transports.push(transport);
                }
                None => {
                    let transport: NetworkTransportData = NetworkTransportData {
                        id: trans.id,
                        addr: trans.addr.clone(),
                        last_ping_ts: trans.last_ping_ts,
                        connections: trans.connections.clone(),
                    };
                    let node = NetworkNodeData {
                        node_id: trans.node_id,
                        transports: vec![transport],
                    };
                    node_map.insert(node.node_id, node);
                }
            }
        }
        node_map.iter().map(|(_, data)| data.clone()).collect()
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
        let id = calc_hash(&addr.clone());
        let last_ping_ts = 123456789;

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(storage.transports.get(&id), Some(&NodeTransportData::new(node_id, addr, last_ping_ts)));
    }

    #[test]
    fn test_upsert_node_updates_last_ping_ts_if_address_already_present_and_last_ping_ts_greater() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let id = calc_hash(&addr.clone());
        let last_ping_ts1 = 123456789;
        let last_ping_ts2 = 987654321;

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts1);
        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts2);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(storage.transports.get(&id), Some(&NodeTransportData::new(node_id, addr, last_ping_ts2)));
    }

    #[test]
    fn test_upsert_node_does_not_update_last_ping_ts_if_address_already_present_and_last_ping_ts_less_or_equal() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let id = calc_hash(&addr.clone());
        let last_ping_ts1 = 123456789;
        let last_ping_ts2 = 987654321;

        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts1);
        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts2);
        storage.upsert_node(node_id.clone(), addr.clone(), last_ping_ts1);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(storage.transports.get(&id), Some(&NodeTransportData::new(node_id, addr, last_ping_ts2)));
    }

    #[test]
    fn test_update_node_connection_updates_fields_of_existing_transport_data_if_address_and_transport_data_match() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let id = calc_hash(&addr.clone());
        let last_ping_ts = 123456789;
        let transport_data1 = TransportConnectionData {
            id: calc_hash(&addr.clone()),
            node_id: node_id.clone(),
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
        let transport_data2 = TransportConnectionData {
            id: calc_hash(&addr.clone()),
            node_id: node_id.clone(),
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
        storage.update_node_connection(addr.clone(), vec![transport_data1.clone()]);
        storage.update_node_connection(addr.clone(), vec![transport_data2.clone()]);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(
            storage.transports.get(&id),
            Some(&NodeTransportData {
                id: id,
                node_id: node_id.clone(),
                addr: addr.clone(),
                last_ping_ts,
                connections: vec![transport_data2],
            })
        );
    }

    #[test]
    fn test_update_node_connection_removes_all_existing_transport_data_if_address_present_and_connections_empty() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let id = calc_hash(&addr.clone());
        let last_ping_ts = 123456789;
        let transport_data = TransportConnectionData {
            id: calc_hash(&addr.clone()),
            node_id: node_id.clone(),
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
        storage.update_node_connection(addr.clone(), vec![transport_data.clone()]);
        storage.update_node_connection(addr.clone(), vec![]);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(
            storage.transports.get(&id),
            Some(&NodeTransportData {
                id: id,
                node_id: node_id.clone(),
                addr: addr.clone(),
                last_ping_ts,
                connections: vec![transport_data],
            })
        );
    }

    #[test]
    fn test_update_node_connection_does_nothing_if_address_not_present() {
        let mut storage = NodeConnectionStorage::new();
        let addr = String::from("127.0.0.1");
        let transport_data = TransportConnectionData {
            id: calc_hash(&addr.clone()),
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

        storage.update_node_connection(addr.clone(), vec![transport_data]);

        assert_eq!(storage.transports.len(), 0);
    }

    #[test]
    fn test_returns_network_node_data_with_correct_node_id_and_matching_transport_data() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr1 = String::from("127.0.0.1");
        let addr2 = String::from("192.168.0.1");
        let last_ping_ts = 123456789;
        let transport_data1 = TransportConnectionData {
            id: calc_hash(&addr1.clone()),
            node_id: node_id.clone(),
            addr: addr1.clone(),
            metric: ConnectionMetric {
                latency: 1,
                loss_percent: 0,
                bandwidth: 100,
            },
            status: ConnectionStatus::CONNECTED,
            last_updated_at: 0,
            direction: 0,
        };
        let transport_data2 = TransportConnectionData {
            id: calc_hash(&addr2.clone()),
            node_id: node_id.clone(),
            addr: addr2.clone(),
            metric: ConnectionMetric {
                latency: 2,
                loss_percent: 1,
                bandwidth: 100,
            },
            status: ConnectionStatus::DISCONNECTED,
            last_updated_at: 987654321,
            direction: 0,
        };

        storage.upsert_node(node_id.clone(), addr1.clone(), last_ping_ts);
        storage.upsert_node(node_id.clone(), addr2.clone(), last_ping_ts);
        storage.update_node_connection(addr1.clone(), vec![transport_data1.clone()]);
        storage.update_node_connection(addr2.clone(), vec![transport_data2.clone()]);

        let result = storage.get_node(node_id);

        assert_eq!(result.node_id, node_id);
        assert_eq!(result.transports.len(), 2);
        assert_eq!(result.transports[0].addr, addr1);
        assert_eq!(result.transports[0].last_ping_ts, last_ping_ts);
        assert_eq!(result.transports[0].connections.len(), 1);
        assert_eq!(result.transports[0].connections[0], transport_data1);
        assert_eq!(result.transports[1].addr, addr2);
        assert_eq!(result.transports[1].last_ping_ts, last_ping_ts);
        assert_eq!(result.transports[1].connections.len(), 1);
        assert_eq!(result.transports[1].connections[0], transport_data2);
    }
}
