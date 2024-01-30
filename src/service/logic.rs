use std::sync::Arc;

use atm0s_sdn_identity::{NodeAddr, NodeAddrBuilder, NodeId};
use parking_lot::RwLock;

use super::{
    controller::{visualization_controller_build, IVisializationController},
    msg::{VisualizationControllAction, VisualziationConf},
};

pub struct VisualizationLogic {
    node_id: NodeId,
    node_addr: NodeAddr,
    controller: Arc<RwLock<Box<dyn IVisializationController + Sync + Send + 'static>>>,
}

impl Clone for VisualizationLogic {
    fn clone(&self) -> Self {
        Self {
            node_id: self.node_id,
            node_addr: self.node_addr.clone(),
            controller: self.controller.clone(),
        }
    }
}

impl VisualizationLogic {
    pub fn new(conf: VisualziationConf) -> Self {
        let controller = Arc::new(RwLock::new(visualization_controller_build(conf.clone())));
        Self {
            node_id: conf.node_id,
            node_addr: conf.node_addr.clone(),
            controller: controller,
        }
    }

    pub fn report_stats(&mut self, now_ms: u64) {
        self.controller.write().report_stats(now_ms);
    }

    pub fn pop_action(&mut self) -> Option<VisualizationControllAction> {
        self.controller.write().pop_action()
    }

    pub fn on_node_connected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, dest_addr: NodeAddr, now: u64) {
        self.controller.write().on_node_connected(src_id, src_addr, dest_id, dest_addr, now);
    }

    pub fn on_node_disconnected(&mut self, src_id: NodeId, src_addr: NodeAddr, dest_id: NodeId, now: u64) {
        let dest_addr_builder = NodeAddrBuilder::new(dest_id);
        self.controller.write().on_node_disconnected(src_id, src_addr, dest_id, dest_addr_builder.addr(), now);
    }

    pub fn execute_action(&mut self, action: VisualizationControllAction) {
        self.controller.write().execute_action(action);
    }

    pub fn dump_graph(self) {
        self.controller.read().dump_graph();
    }
}
