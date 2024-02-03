use atm0s_sdn_identity::NodeId;
use atm0s_sdn_network::behaviour::{ConnectionContext, ConnectionHandler, ConnectionHandlerAction};
use atm0s_sdn_network::transport::ConnectionEvent;
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use crate::VisualizationAgentMsg;

use super::msg::{VisualizationMasterBehaviourEvent, VisualizationMasterHandlerEvent};

pub struct VisualizationMasterHandler<BE, HE> {
    actions: VecDeque<ConnectionHandlerAction<BE, HE>>,
}

impl<BE, HE> VisualizationMasterHandler<BE, HE>
where
    BE: From<VisualizationMasterBehaviourEvent> + TryInto<VisualizationMasterBehaviourEvent> + Send + Sync + 'static,
    HE: From<VisualizationMasterHandlerEvent> + TryInto<VisualizationMasterHandlerEvent> + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self { actions: VecDeque::new() }
    }
}

impl<BE, HE> ConnectionHandler<BE, HE> for VisualizationMasterHandler<BE, HE>
where
    BE: From<VisualizationMasterBehaviourEvent> + TryInto<VisualizationMasterBehaviourEvent> + Send + Sync + 'static,
    HE: From<VisualizationMasterHandlerEvent> + TryInto<VisualizationMasterHandlerEvent> + Send + Sync + 'static,
{
    fn on_awake(&mut self, ctx: &ConnectionContext, now_ms: u64) {}

    fn on_opened(&mut self, ctx: &ConnectionContext, now_ms: u64) {}

    fn on_tick(&mut self, ctx: &ConnectionContext, now_ms: u64, interval_ms: u64) {}

    fn on_event(&mut self, ctx: &ConnectionContext, now_ms: u64, event: ConnectionEvent) {
        match event {
            ConnectionEvent::Msg(msg) => {
                if let Ok(payload) = msg.get_payload_bincode::<VisualizationAgentMsg>() {
                    let behaviour_event = VisualizationMasterBehaviourEvent::OnMsg(payload);
                    self.actions.push_back(ConnectionHandlerAction::ToBehaviour(behaviour_event.into()));
                }
            }
            _ => {}
        }
    }

    fn on_behavior_event(&mut self, ctx: &ConnectionContext, now_ms: u64, event: HE) {}

    fn on_other_handler_event(&mut self, ctx: &ConnectionContext, now_ms: u64, from_node: NodeId, from_conn: atm0s_sdn_identity::ConnId, event: HE) {}

    fn on_closed(&mut self, ctx: &ConnectionContext, now_ms: u64) {}

    fn pop_action(&mut self) -> Option<ConnectionHandlerAction<BE, HE>> {
        self.actions.pop_front()
    }
}
