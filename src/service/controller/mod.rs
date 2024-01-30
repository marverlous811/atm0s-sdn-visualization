mod agent;
mod collector;

use atm0s_sdn_identity::{NodeAddr, NodeId};

use self::{agent::NodeAgent, collector::NodeStatsCollector};

use super::msg::{VisualizationControllAction, VisualziationConf};

pub trait IVisializationController {
    fn report_stats(&mut self, ts: u64);
    fn on_node_connected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, dest_addr: NodeAddr, now: u64);
    fn on_node_disconnected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, dest_addr: NodeAddr, now: u64);
    fn pop_action(&mut self) -> Option<VisualizationControllAction>;
    fn execute_action(&mut self, action: VisualizationControllAction);
    fn dump_graph(&self);
}

pub fn visualization_controller_build(conf: VisualziationConf) -> Box<dyn IVisializationController + Sync + Send + 'static> {
    match conf.is_master {
        true => Box::new(NodeStatsCollector::new(conf.node_id, conf.node_addr)),
        false => Box::new(NodeAgent::new(conf.node_id, conf.node_addr)),
    }
}
