use std::collections::{HashMap, HashSet};

use atm0s_sdn_identity::{NodeAddr, NodeId};

pub struct NetworkNodeData {
    pub node_id: NodeId,
    pub node_addr: NodeAddr,
}

impl NetworkNodeData {
    pub fn new(id: NodeId, addr: NodeAddr) -> Self {
        Self { node_id: id, node_addr: addr }
    }
}

pub struct NetworkNode {
    pub node_id: NodeId,
    pub node_addr: NodeAddr,
    pub latest_ping: u64,
    pub node_neighbors: Vec<NodeId>,
}

pub struct NetworkGraph {
    nodes: HashMap<NodeId, NetworkNode>,
}

impl NetworkGraph {
    pub fn new() -> NetworkGraph {
        Self { nodes: HashMap::new() }
    }

    pub fn upsert_node(&mut self, node_id: NodeId, addr: NodeAddr, time: u64, neighbour_ids: Option<Vec<NodeId>>) {
        match self.nodes.get_mut(&node_id) {
            Some(node) => {
                if time > node.latest_ping {
                    node.latest_ping = time;
                }
                match neighbour_ids {
                    Some(neighbour_ids) => {
                        node.node_neighbors = neighbour_ids;
                    }
                    None => {}
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

    pub fn add_edge(&mut self, src_id: NodeId, dest_id: NodeId) {
        match self.nodes.get_mut(&src_id) {
            Some(node) => {
                let mut tmp = node.node_neighbors.clone();
                tmp.push(dest_id);
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

    pub fn remove_edge(&mut self, src_id: NodeId, dest_id: NodeId) {
        match self.nodes.get_mut(&src_id) {
            Some(node) => {
                node.node_neighbors.retain(|&x| x != dest_id);
            }
            None => {
                panic!("This case cannot be happened");
            }
        }
    }

    pub fn get_node(&mut self, id: NodeId) -> Option<&NetworkNode> {
        self.nodes.get(&id)
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
        network_graph.upsert_node(node_id, node_addr, time, None);
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
        network_graph.upsert_node(node_id, node_addr.clone(), time1, None);
        network_graph.upsert_node(node_id, node_addr.clone(), time2, None);
        assert_eq!(network_graph.nodes.len(), 1);
        assert_eq!(network_graph.nodes.get(&node_id).unwrap().latest_ping, time2);
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
        network_graph.upsert_node(src_node_id, src_node_addr.clone(), time, None);
        network_graph.upsert_node(dest_node_id, dest_node_addr.clone(), time, None);
        network_graph.add_edge(src_node_id, dest_node_id);
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
        network_graph.upsert_node(src_node_id, src_node_addr.clone(), time, None);
        network_graph.upsert_node(dest_node_id, dest_node_addr.clone(), time, None);
        network_graph.add_edge(src_node_id, dest_node_id);
        network_graph.add_edge(src_node_id, dest_node_id);
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
        network_graph.upsert_node(src_node_id, src_node_addr.clone(), time, None);
        network_graph.upsert_node(dest_node_id, dest_node_addr.clone(), time, None);
        network_graph.add_edge(src_node_id, dest_node_id);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, vec![dest_node_id]);

        network_graph.remove_edge(src_node_id, dest_node_id);
        assert_eq!(network_graph.nodes.len(), 2);
        assert_eq!(network_graph.nodes.get(&src_node_id).unwrap().node_neighbors, Vec::<NodeId>::new());
    }
}
