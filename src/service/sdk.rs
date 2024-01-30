use super::logic::VisualizationLogic;

pub struct VisualizationSdk {
    logic: VisualizationLogic,
}

impl Clone for VisualizationSdk {
    fn clone(&self) -> Self {
        Self { logic: self.logic.clone() }
    }
}

impl VisualizationSdk {
    pub fn new(logic: VisualizationLogic) -> Self {
        Self { logic }
    }

    pub fn dump_graph(self) {
        self.logic.dump_graph();
    }
}
