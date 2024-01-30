use atm0s_sdn_identity::NodeId;
use atm0s_sdn_network::{
    behaviour::{ConnectionHandler, ConnectionHandlerAction},
    transport::ConnectionEvent,
};
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use super::msg::{VisualizationBehaviorEvent, VisualizationHandlerEvent, VisualizationMsg};

pub struct VisualizationConnectionHandler<BE, HE> {
    actions: VecDeque<ConnectionHandlerAction<BE, HE>>,
}

impl<BE, HE> VisualizationConnectionHandler<BE, HE>
where
    BE: From<VisualizationBehaviorEvent> + TryInto<VisualizationBehaviorEvent> + Send + Sync + 'static,
    HE: From<VisualizationHandlerEvent> + TryInto<VisualizationHandlerEvent> + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self { actions: VecDeque::new() }
    }
}

impl<BE, HE> ConnectionHandler<BE, HE> for VisualizationConnectionHandler<BE, HE>
where
    BE: From<VisualizationBehaviorEvent> + TryInto<VisualizationBehaviorEvent> + Send + Sync + 'static,
    HE: From<VisualizationHandlerEvent> + TryInto<VisualizationHandlerEvent> + Send + Sync + 'static,
{
    fn on_awake(&mut self, _ctx: &atm0s_sdn_network::behaviour::ConnectionContext, _now_ms: u64) {}

    fn on_opened(&mut self, _ctx: &atm0s_sdn_network::behaviour::ConnectionContext, _now_ms: u64) {}

    fn on_tick(&mut self, _ctx: &atm0s_sdn_network::behaviour::ConnectionContext, _now_ms: u64, _interval_ms: u64) {}

    fn on_event(&mut self, _ctx: &atm0s_sdn_network::behaviour::ConnectionContext, _now_ms: u64, event: atm0s_sdn_network::transport::ConnectionEvent) {
        match event {
            ConnectionEvent::Msg(msg) => {
                if let Ok(payload) = msg.get_payload_bincode::<VisualizationMsg>() {
                    let behaviour_event = VisualizationBehaviorEvent::OnMsg(payload);
                    self.actions.push_back(ConnectionHandlerAction::ToBehaviour(behaviour_event.into()));
                }
            }
            ConnectionEvent::Stats(_stats) => {}
        }
    }

    fn on_other_handler_event(&mut self, _ctx: &atm0s_sdn_network::behaviour::ConnectionContext, _now_ms: u64, _from_node: NodeId, _from_conn: atm0s_sdn_identity::ConnId, _event: HE) {}

    fn on_behavior_event(&mut self, _ctx: &atm0s_sdn_network::behaviour::ConnectionContext, _now_ms: u64, _event: HE) {}

    fn on_closed(&mut self, _ctx: &atm0s_sdn_network::behaviour::ConnectionContext, _now_ms: u64) {}

    fn pop_action(&mut self) -> Option<atm0s_sdn_network::behaviour::ConnectionHandlerAction<BE, HE>> {
        self.actions.pop_front()
    }
}
