use crate::{
    collector::{NodeConnectionData, NodeData, SdnMonitorController},
    VisualizationAgentMsg,
};

pub struct VisualizationMasterLogic {
    controller: SdnMonitorController,
}

impl Clone for VisualizationMasterLogic {
    fn clone(&self) -> Self {
        Self { controller: self.controller.clone() }
    }
}

impl VisualizationMasterLogic {
    pub fn new(controller: SdnMonitorController) -> Self {
        Self { controller: controller.clone() }
    }

    pub fn process_agent_msg(&mut self, msg: VisualizationAgentMsg) {
        match msg {
            VisualizationAgentMsg::NodePing(node_id, addr, now_ms) => {
                self.controller.upsert_node(node_id, addr, now_ms);
            }
            VisualizationAgentMsg::NodeConnections(addr, conns) => {
                let data: Vec<NodeConnectionData> = conns
                    .into_iter()
                    .map(|conn| NodeConnectionData {
                        id: conn.conn_id,
                        node_id: conn.node_id,
                        protocol: conn.protocol,
                        addr: conn.addr,
                        metric: conn.metric.clone(),
                        direction: conn.direction,
                        status: conn.status,
                        last_updated_at: conn.latest_updated_at,
                    })
                    .collect();
                self.controller.update_node_conns(addr, data);
            }
        }
    }

    pub fn get_nodes(&self) -> Vec<NodeData> {
        self.controller.get_nodes()
    }
}
