use super::{logic::VisualizationMasterLogic, storage::NetworkNodeData};

pub struct VisualizationMasterSdk {
    logic: VisualizationMasterLogic,
}

impl Clone for VisualizationMasterSdk {
    fn clone(&self) -> Self {
        Self { logic: self.logic.clone() }
    }
}

impl VisualizationMasterSdk {
    pub fn new(logic: VisualizationMasterLogic) -> Self {
        Self { logic }
    }

    pub fn get_nodes(&self) -> Vec<NetworkNodeData> {
        self.logic.get_nodes()
    }

    pub fn dump_graph(self) {
        self.logic.dump_graph()
    }
}
