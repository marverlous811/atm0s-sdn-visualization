use crate::collector::{NodeData, SdnMonitorController};

pub struct VisualizationMasterSdk {
    controller: SdnMonitorController,
}

impl Clone for VisualizationMasterSdk {
    fn clone(&self) -> Self {
        Self { controller: self.controller.clone() }
    }
}

impl VisualizationMasterSdk {
    pub fn new(controller: SdnMonitorController) -> Self {
        Self { controller }
    }

    pub fn get_nodes(&self) -> Vec<NodeData> {
        self.controller.get_nodes()
    }
}
