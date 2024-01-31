use atm0s_sdn_identity::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationAgentBehaviourEvent {}

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationAgentHandlerEvent {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum VisualizationAgentMsg {
    //node_id, node_address, timestamp, neighbour_ids
    NodeStats(NodeId, Vec<u8>, u64, Option<Vec<NodeId>>),
}
