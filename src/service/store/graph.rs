use std::collections::{HashMap, HashSet};

use atm0s_sdn_identity::{NodeAddr, NodeId};

pub struct NetworkNodeData {
    node_id: NodeId,
    node_addr: NodeAddr,
}

impl NetworkNodeData {
    pub fn new(id: NodeId, addr: NodeAddr) -> Self {
        Self { node_id: id, node_addr: addr }
    }
}

struct NetworkNode {
    node_id: NodeId,
    node_addr: NodeAddr,
    latest_ping: u64,
    node_neighbors: Vec<NodeId>,
}

pub struct NetworkGraphEdgeData {
    pub src_node: NetworkNodeData,
    pub dest_node: NetworkNodeData,
    pub time: u64,
}

impl NetworkGraphEdgeData {
    pub fn new(src_node_id: NodeId, src_node_addr: NodeAddr, dest_node_id: NodeId, dest_node_addr: NodeAddr, ts: u64) -> Self {
        Self {
            src_node: NetworkNodeData::new(src_node_id, src_node_addr),
            dest_node: NetworkNodeData::new(dest_node_id, dest_node_addr),
            time: ts,
        }
    }
}

pub struct NetworkGraph {
    nodes: HashMap<NodeId, NetworkNode>,
}

impl NetworkGraph {
    pub fn new() -> NetworkGraph {
        Self { nodes: HashMap::new() }
    }

    pub fn upsert_node(&mut self, node_id: NodeId, addr: NodeAddr, time: u64) {
        match self.nodes.get_mut(&node_id) {
            Some(node) => {
                if time > node.latest_ping {
                    node.latest_ping = time;
                }
            }
            None => {
                self.nodes.insert(
                    node_id,
                    NetworkNode {
                        node_id,
                        node_addr: addr,
                        latest_ping: time,
                        node_neighbors: Vec::new(),
                    },
                );
            }
        };
    }

    pub fn add_edge(&mut self, data: NetworkGraphEdgeData) {
        self.upsert_node(data.src_node.node_id, data.src_node.node_addr, data.time);
        self.upsert_node(data.dest_node.node_id, data.dest_node.node_addr, data.time);
        match self.nodes.get_mut(&data.src_node.node_id) {
            Some(node) => {
                let mut tmp = node.node_neighbors.clone();
                tmp.push(data.dest_node.node_id);
                let mut retval = Vec::new();
                let mut unique_vec = HashSet::<NodeId>::new();
                for &id in &tmp {
                    if unique_vec.insert(id) {
                        retval.push(id);
                    }
                }

                node.node_neighbors = retval;
            }
            None => {
                panic!("This case cannot be happened");
            }
        }
    }

    pub fn remove_edge(&mut self, data: NetworkGraphEdgeData) {
        self.upsert_node(data.src_node.node_id, data.src_node.node_addr, data.time);
        match self.nodes.get_mut(&data.src_node.node_id) {
            Some(node) => {
                node.node_neighbors.retain(|&x| x != data.dest_node.node_id);
            }
            None => {
                panic!("This case cannot be happened");
            }
        }
    }

    pub fn dump_print(&self) {
        for iter in self.nodes.iter() {
            let node_id_str: Vec<String> = iter.1.node_neighbors.iter().map(|&x| x.to_string()).collect();
            println!(
                "Node: {}, Addr: {}, last ping: {}, connected to: {}",
                iter.1.node_id,
                iter.1.node_addr,
                iter.1.latest_ping,
                node_id_str.join(",")
            );
        }
    }
}

#[cfg(test)]
mod tests {

    use atm0s_sdn_identity::NodeAddrBuilder;

    use super::*;
    #[test]
    fn test_upsert_new_node_unique_node_id() {
        let mut network_graph = NetworkGraph::new();
        let node_id = 1;
        let node_addr_builder = NodeAddrBuilder::new(node_id);
        let node_addr = node_addr_builder.addr();
        let time = 123;
        network_graph.upsert_node(node_id, node_addr, time);
        assert_eq!(network_graph.nodes.len(), 1);
        assert_eq!(network_graph.nodes.get(&node_id).unwrap().latest_ping, time);
    }

    #[test]
    fn test_upsert_existing_node_update_latest_ping() {
        let mut network_graph = NetworkGraph::new();
        let node_id = 1;
        let node_addr_builder = NodeAddrBuilder::new(node_id);
        let node_addr = node_addr_builder.addr();
        let time1 = 123;
        let time2 = 456;
        network_graph.upsert_node(node_id, node_addr.clone(), time1);
        network_graph.upsert_node(node_id, node_addr.clone(), time2);
        assert_eq!(network_graph.nodes.len(), 1);
        assert_eq!(network_graph.nodes.get(&node_id).unwrap().latest_ping, time2);
    }

    #[test]
    fn test_new_edge_added_correctly() {
        let mut network_graph = NetworkGraph::new();
        let src_node_id = 1;
        let dest_node_id = 2;
        let src_node_addr_builder = NodeAddrBuilder::new(src_node_id);
        let dest_node_addr_builder = NodeAddrBuilder::new(dest_node_id);
        let src_node_addr = src_node_addr_builder.addr();
        let dest_node_addr = dest_node_addr_builder.addr();
        let time = 123;
        let data = NetworkGraphEdgeData {
            src_node: NetworkNodeData {
                node_id: src_node_id,
                node_addr: src_node_addr,
            },
            dest_node: NetworkNodeData {
                node_id: dest_node_id,
                node_addr: dest_node_addr,
            },
            time,
        };
        network_graph.add_edge(data);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, vec![dest_node_id]);
    }

    #[test]
    fn test_new_edge_added_with_existing_nodes() {
        let mut network_graph = NetworkGraph::new();
        let src_node_id = 1;
        let dest_node_id = 2;
        let src_node_addr_builder = NodeAddrBuilder::new(src_node_id);
        let dest_node_addr_builder = NodeAddrBuilder::new(dest_node_id);
        let src_node_addr = src_node_addr_builder.addr();
        let dest_node_addr = dest_node_addr_builder.addr();
        let time = 123;
        network_graph.upsert_node(src_node_id, src_node_addr.clone(), time);
        network_graph.upsert_node(dest_node_id, dest_node_addr.clone(), time);
        let data = NetworkGraphEdgeData {
            src_node: NetworkNodeData {
                node_id: src_node_id,
                node_addr: src_node_addr,
            },
            dest_node: NetworkNodeData {
                node_id: dest_node_id,
                node_addr: dest_node_addr,
            },
            time,
        };
        network_graph.add_edge(data);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, vec![dest_node_id]);
    }

    #[test]
    fn test_new_edge_added_with_one_existing_node() {
        let mut network_graph = NetworkGraph::new();
        let src_node_id = 1;
        let dest_node_id = 2;
        let src_node_addr_builder = NodeAddrBuilder::new(src_node_id);
        let dest_node_addr_builder = NodeAddrBuilder::new(dest_node_id);
        let src_node_addr = src_node_addr_builder.addr();
        let dest_node_addr = dest_node_addr_builder.addr();
        let time = 123;
        network_graph.upsert_node(src_node_id, src_node_addr.clone(), time);
        let data = NetworkGraphEdgeData {
            src_node: NetworkNodeData {
                node_id: src_node_id,
                node_addr: src_node_addr,
            },
            dest_node: NetworkNodeData {
                node_id: dest_node_id,
                node_addr: dest_node_addr,
            },
            time,
        };
        network_graph.add_edge(data);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, vec![dest_node_id]);
    }

    #[test]
    fn test_new_edge_not_added_same_source_dest() {
        let mut network_graph = NetworkGraph::new();
        let src_node_id = 1;
        let dest_node_id = 2;
        let src_node_addr_builder = NodeAddrBuilder::new(src_node_id);
        let dest_node_addr_builder = NodeAddrBuilder::new(dest_node_id);
        let src_node_addr = src_node_addr_builder.addr();
        let dest_node_addr = dest_node_addr_builder.addr();
        let time = 123;
        let data = NetworkGraphEdgeData {
            src_node: NetworkNodeData {
                node_id: src_node_id,
                node_addr: src_node_addr.clone(),
            },
            dest_node: NetworkNodeData {
                node_id: dest_node_id,
                node_addr: dest_node_addr.clone(),
            },
            time,
        };
        network_graph.add_edge(data);
        let data2 = NetworkGraphEdgeData {
            src_node: NetworkNodeData {
                node_id: src_node_id,
                node_addr: src_node_addr.clone(),
            },
            dest_node: NetworkNodeData {
                node_id: dest_node_id,
                node_addr: dest_node_addr.clone(),
            },
            time: 456,
        };
        network_graph.add_edge(data2);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, vec![dest_node_id]);
    }

    #[test]
    fn test_remove_edge_successfully_removes_edge() {
        let mut network_graph = NetworkGraph::new();
        let src_node_id = 1;
        let dest_node_id = 2;
        let src_node_addr_builder = NodeAddrBuilder::new(src_node_id);
        let dest_node_addr_builder = NodeAddrBuilder::new(dest_node_id);
        let src_node_addr = src_node_addr_builder.addr();
        let dest_node_addr = dest_node_addr_builder.addr();
        let time = 123;
        let data = NetworkGraphEdgeData {
            src_node: NetworkNodeData {
                node_id: src_node_id,
                node_addr: src_node_addr.clone(),
            },
            dest_node: NetworkNodeData {
                node_id: dest_node_id,
                node_addr: dest_node_addr.clone(),
            },
            time,
        };
        network_graph.add_edge(data);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, vec![dest_node_id]);

        let remove_data = NetworkGraphEdgeData {
            src_node: NetworkNodeData {
                node_id: src_node_id,
                node_addr: src_node_addr.clone(),
            },
            dest_node: NetworkNodeData {
                node_id: dest_node_id,
                node_addr: dest_node_addr.clone(),
            },
            time: 456,
        };
        network_graph.remove_edge(remove_data);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, Vec::<NodeId>::new());
    }
}
