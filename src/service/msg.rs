use atm0s_sdn_identity::{NodeAddr, NodeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VisualziationConf {
    pub node_id: NodeId,
    pub node_addr: NodeAddr,
    pub is_master: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationBehaviorEvent {
    OnMsg(VisualizationMsg),
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationHandlerEvent {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VisualizationSdkEvent {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VisualizationControllAction {
    //node_id, node_address, timestamp
    NodeStats(NodeId, NodeAddr, u64),
    //src_node_id, src_node_addr, dest_node_id, dest_node_addr, timestamp
    OnNodeConnected(NodeId, NodeAddr, NodeId, NodeAddr, u64),
    //src_node_id, src_node_addr, dest_node_id, dest_node_addr, timestamp
    OnNodeDisconencted(NodeId, NodeAddr, NodeId, NodeAddr, u64),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum VisualizationMsg {
    //node_id, node_address, timestamp
    NodeStats(NodeId, Vec<u8>, u64),
    //src_node_id, src_node_addr, dest_node_id, dest_node_addr, timestamp
    OnNodeConnected(NodeId, Vec<u8>, NodeId, Vec<u8>, u64),
    //src_node_id, src_node_addr, dest_node_id, dest_node_addr, timestamp
    OnNodeDisconencted(NodeId, Vec<u8>, NodeId, Vec<u8>, u64),
}
