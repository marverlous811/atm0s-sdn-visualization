use atm0s_sdn_identity::{NodeAddr, NodeId};
use atm0s_sdn_utils::hashmap::HashMap;

pub struct NetworkNode {
    pub id: NodeId,
    pub addr: NodeAddr,
    pub latest_ping: u64,
    pub neighbour_ids: Vec<NodeId>,
}

pub struct VisualizationController {
    nodes: HashMap<NodeId, NetworkNode>,
}

impl VisualizationController {
    pub fn new() -> VisualizationController {
        Self { nodes: HashMap::new() }
    }

    pub fn upsert_node_stats(&mut self, data: NetworkNode) {
        match self.nodes.get_mut(&data.id) {
            Some(node) => {
                if data.latest_ping > node.latest_ping {
                    node.latest_ping = data.latest_ping;
                }
                node.neighbour_ids = data.neighbour_ids;
            }
            None => {
                self.nodes.insert(data.id, data);
            }
        };
    }

    pub fn dump_print(&self) {
        for iter in self.nodes.iter() {
            let node_id_str: Vec<String> = iter.1.neighbour_ids.iter().map(|&x| x.to_string()).collect();
            println!("Node: {}, Addr: {}, last ping: {}, connected to: {}", iter.1.id, iter.1.addr, iter.1.latest_ping, node_id_str.join(","));
        }
    }
}
