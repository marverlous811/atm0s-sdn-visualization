use crate::identity::{ConnectionMetric, ConnectionStatus};
use atm0s_sdn_identity::NodeId;
use atm0s_sdn_utils::hashmap::HashMap;
use log::error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransportData {
    pub node_id: NodeId,
    pub addr: String,
    pub metric: ConnectionMetric,
    pub status: ConnectionStatus,
    pub last_updated_at: u64,
    pub direction: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NodeTransportData {
    node_id: NodeId,
    addr: String,
    last_ping_ts: u64,
    connections: Vec<TransportData>,
}

impl NodeTransportData {
    pub fn new(node_id: NodeId, addr: String, last_ping_ts: u64) -> Self {
        Self {
            node_id,
            addr,
            last_ping_ts,
            connections: Vec::new(),
        }
    }
}

pub struct NodeConnectionStorage {
    transports: HashMap<String, NodeTransportData>,
}

impl NodeConnectionStorage {
    pub fn new() -> Self {
        Self { transports: HashMap::new() }
    }

    pub fn upsertNode(&mut self, node_id: NodeId, addr: String, last_ping_ts: u64) {
        match self.transports.get_mut(&addr) {
            Some(trans) => {
                if last_ping_ts > trans.last_ping_ts {
                    trans.last_ping_ts = last_ping_ts;
                }
            }
            None => {
                let trans = NodeTransportData::new(node_id, addr.clone(), last_ping_ts);
                self.transports.insert(addr, trans);
            }
        }
    }

    pub fn updateNodeConnection(&mut self, addr: String, connections: Vec<TransportData>) {
        match self.transports.get_mut(&addr) {
            Some(trans) => {
                let mut tmp = HashMap::<String, TransportData>::new();
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

    pub fn print_dump(&self) {
        for (_, trans) in self.transports.iter() {
            println!("Node: {}, Addr: {}, last ping: {}", trans.node_id, trans.addr, trans.last_ping_ts);
            println!("List Connections: ");
            for conn in trans.connections.clone().into_iter() {
                println!("- Node: {}, Addr: {}, direction: {}, status: {}", conn.node_id, conn.addr, conn.direction, conn.status.to_bytes());
                println!(
                    "- Connection stats: latency: {}ms, bandwidth: {}kbps, loss: {}%",
                    conn.metric.latency, conn.metric.bandwidth, conn.metric.loss_percent
                );
                println!("======================================")
            }

            println!("====================END_CONNECTION_INFO=======================")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_upsertNode_adds_new_NodeTransportData_if_address_not_present() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts = 123456789;

        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(storage.transports.get(&addr), Some(&NodeTransportData::new(node_id, addr, last_ping_ts)));
    }

    #[test]
    fn test_upsertNode_updates_last_ping_ts_if_address_already_present_and_last_ping_ts_greater() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts1 = 123456789;
        let last_ping_ts2 = 987654321;

        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts1);
        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts2);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(storage.transports.get(&addr), Some(&NodeTransportData::new(node_id, addr, last_ping_ts2)));
    }

    #[test]
    fn test_upsertNode_does_not_update_last_ping_ts_if_address_already_present_and_last_ping_ts_less_or_equal() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts1 = 123456789;
        let last_ping_ts2 = 987654321;

        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts1);
        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts2);
        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts1);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(storage.transports.get(&addr), Some(&NodeTransportData::new(node_id, addr, last_ping_ts2)));
    }

    #[test]
    fn test_updateNodeConnection_updates_fields_of_existing_TransportData_if_address_and_TransportData_match() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts = 123456789;
        let transport_data1 = TransportData {
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
        let transport_data2 = TransportData {
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

        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts);
        storage.updateNodeConnection(addr.clone(), vec![transport_data1.clone()]);
        storage.updateNodeConnection(addr.clone(), vec![transport_data2.clone()]);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(
            storage.transports.get(&addr),
            Some(&NodeTransportData {
                node_id: node_id.clone(),
                addr: addr.clone(),
                last_ping_ts,
                connections: vec![transport_data2],
            })
        );
    }

    #[test]
    fn test_updateNodeConnection_removes_all_existing_TransportData_if_address_present_and_connections_empty() {
        let mut storage = NodeConnectionStorage::new();
        let node_id = 1;
        let addr = String::from("127.0.0.1");
        let last_ping_ts = 123456789;
        let transport_data = TransportData {
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

        storage.upsertNode(node_id.clone(), addr.clone(), last_ping_ts);
        storage.updateNodeConnection(addr.clone(), vec![transport_data.clone()]);
        storage.updateNodeConnection(addr.clone(), vec![]);

        assert_eq!(storage.transports.len(), 1);
        assert_eq!(
            storage.transports.get(&addr),
            Some(&NodeTransportData {
                node_id: node_id.clone(),
                addr: addr.clone(),
                last_ping_ts,
                connections: vec![transport_data],
            })
        );
    }

    #[test]
    fn test_updateNodeConnection_does_nothing_if_address_not_present() {
        let mut storage = NodeConnectionStorage::new();
        let addr = String::from("127.0.0.1");
        let transport_data = TransportData {
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

        storage.updateNodeConnection(addr.clone(), vec![transport_data]);

        assert_eq!(storage.transports.len(), 0);
    }
}
