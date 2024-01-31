use atm0s_sdn_identity::NodeId;
use serde::{Deserialize, Serialize};

use crate::VisualizationAgentMsg;

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationMasterBehaviourEvent {
    OnMsg(VisualizationAgentMsg),
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisualizationMasterHandlerEvent {}
