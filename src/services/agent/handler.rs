use atm0s_sdn_identity::{ConnId, NodeId};
use atm0s_sdn_network::behaviour::{ConnectionContext, ConnectionHandler, ConnectionHandlerAction};
use atm0s_sdn_network::transport::ConnectionEvent;
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use crate::identity::ConnectionMetric;
use crate::{VisualizationAgentBehaviourEvent, VisualizationAgentHandlerEvent};

pub struct VisualizationAgentHandler<BE, HE> {
    conn_id: ConnId,
    actions: VecDeque<ConnectionHandlerAction<BE, HE>>,
}

impl<BE, HE> VisualizationAgentHandler<BE, HE> {
    pub fn new(conn_id: ConnId) -> Self {
        Self { conn_id, actions: VecDeque::new() }
    }
}

impl<BE, HE> ConnectionHandler<BE, HE> for VisualizationAgentHandler<BE, HE>
where
    BE: From<VisualizationAgentBehaviourEvent> + TryInto<VisualizationAgentBehaviourEvent> + Send + Sync + 'static,
    HE: From<VisualizationAgentHandlerEvent> + TryInto<VisualizationAgentHandlerEvent> + Send + Sync + 'static,
{
    fn on_awake(&mut self, ctx: &ConnectionContext, now_ms: u64) {}

    fn on_opened(&mut self, ctx: &ConnectionContext, now_ms: u64) {}

    fn on_tick(&mut self, ctx: &ConnectionContext, now_ms: u64, interval_ms: u64) {}

    fn on_event(&mut self, ctx: &ConnectionContext, now_ms: u64, event: ConnectionEvent) {
        match event {
            ConnectionEvent::Msg(msg) => {}
            ConnectionEvent::Stats(stats) => {
                // println!("on stats event...");
                let metric = ConnectionMetric {
                    latency: stats.rtt_ms,
                    bandwidth: stats.sending_kbps,
                    loss_percent: stats.loss_percent,
                };
                let be = VisualizationAgentBehaviourEvent::ConnectionStats(self.conn_id, metric);
                self.actions.push_back(ConnectionHandlerAction::ToBehaviour(be.into()));
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
