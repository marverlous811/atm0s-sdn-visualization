mod behaviour;
mod handler;
mod logic;
mod msg;
mod storage;

pub static VISUALIZATION_AGENT_SERVICE: u8 = 9;
pub use behaviour::{VisualizationAgentBehaviour, VisualizationAgentBehaviourConf};
pub use msg::{VisualizationAgentBehaviourEvent, VisualizationAgentHandlerEvent, VisualizationAgentMsg};
