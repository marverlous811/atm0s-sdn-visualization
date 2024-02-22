use std::any::Any;

use atm0s_sdn_identity::{NodeAddr, NodeId};
use atm0s_sdn_network::behaviour::{BehaviorContext, ConnectionHandler, NetworkBehavior, NetworkBehaviorAction};
use atm0s_sdn_network::msg::{MsgHeader, TransportMsg};
use atm0s_sdn_network::transport::{ConnectionRejectReason, ConnectionSender, OutgoingConnectionError};
use atm0s_sdn_router::RouteRule;
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use crate::services::master::VISUALIZATION_MASTER_SERVICE;

use super::handler::VisualizationAgentHandler;
use super::logic::VisualizationAgentLogic;
use super::msg::{VisualizationAgentBehaviourEvent, VisualizationAgentHandlerEvent};
use super::VISUALIZATION_AGENT_SERVICE;

pub struct VisualizationAgentBehaviourConf {
    pub node_id: NodeId,
    pub node_addr: NodeAddr,
}

pub struct VisualizationAgentBehaviour<HE, SE> {
    logic: VisualizationAgentLogic,
    queue_action: VecDeque<NetworkBehaviorAction<HE, SE>>,
}

impl<HE, SE> VisualizationAgentBehaviour<HE, SE> {
    pub fn new(conf: VisualizationAgentBehaviourConf) -> Self {
        Self {
            logic: VisualizationAgentLogic::new(conf.node_id, conf.node_addr),
            queue_action: VecDeque::new(),
        }
    }

    pub fn process_all_msg(&mut self) {
        while let Some(msg) = self.logic.pop_msg() {
            let header = MsgHeader::new()
                .set_to_service_id(VISUALIZATION_MASTER_SERVICE)
                .set_route(RouteRule::ToService(VISUALIZATION_MASTER_SERVICE as u32));
            let action = TransportMsg::from_payload_bincode(header, &msg);
            self.queue_action.push_back(NetworkBehaviorAction::ToNet(action))
        }
    }
}

impl<BE, HE, SE> NetworkBehavior<BE, HE, SE> for VisualizationAgentBehaviour<HE, SE>
where
    BE: From<VisualizationAgentBehaviourEvent> + TryInto<VisualizationAgentBehaviourEvent> + Send + Sync + 'static,
    HE: From<VisualizationAgentHandlerEvent> + TryInto<VisualizationAgentHandlerEvent> + Send + Sync + 'static,
    SE: Send + Sync + 'static,
{
    fn service_id(&self) -> u8 {
        return VISUALIZATION_AGENT_SERVICE;
    }

    fn on_started(&mut self, ctx: &BehaviorContext, now_ms: u64) {
        self.logic.report_stats(now_ms)
    }

    fn on_awake(&mut self, ctx: &BehaviorContext, now_ms: u64) {}

    fn on_tick(&mut self, ctx: &BehaviorContext, now_ms: u64, interval_ms: u64) {
        self.process_all_msg();
        self.logic.report_stats(now_ms);
    }

    fn on_local_msg(&mut self, ctx: &BehaviorContext, now_ms: u64, msg: TransportMsg) {}

    fn on_handler_event(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId, event: BE) {
        let msg: Result<VisualizationAgentBehaviourEvent, _> = event.try_into();
        match msg {
            Ok(msg) => match msg {
                VisualizationAgentBehaviourEvent::ConnectionStats(conn_id, node_id, metric) => self.logic.on_connection_stats(conn_id, node_id, metric, now_ms),
            },
            Err(_e) => {}
        }
    }

    fn on_sdk_msg(&mut self, ctx: &BehaviorContext, now_ms: u64, from_service: u8, event: SE) {}

    fn check_incoming_connection(&mut self, ctx: &BehaviorContext, now_ms: u64, node: NodeId, conn_id: atm0s_sdn_identity::ConnId) -> Result<(), ConnectionRejectReason> {
        Ok(())
    }

    fn on_incoming_connection_connected(&mut self, ctx: &BehaviorContext, now_ms: u64, conn: std::sync::Arc<dyn ConnectionSender>) -> Option<Box<dyn ConnectionHandler<BE, HE>>> {
        self.logic.on_node_connected(conn.conn_id(), conn.remote_node_id(), conn.remote_addr(), now_ms);
        Some(Box::new(VisualizationAgentHandler::new(conn.conn_id(), conn.remote_node_id())))
    }

    fn on_incoming_connection_disconnected(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId) {
        self.logic.on_node_disconnected(conn_id, node_id, now_ms);
    }

    fn check_outgoing_connection(&mut self, ctx: &BehaviorContext, now_ms: u64, node: NodeId, conn_id: atm0s_sdn_identity::ConnId) -> Result<(), ConnectionRejectReason> {
        Ok(())
    }

    fn on_outgoing_connection_connected(&mut self, ctx: &BehaviorContext, now_ms: u64, conn: std::sync::Arc<dyn ConnectionSender>) -> Option<Box<dyn ConnectionHandler<BE, HE>>> {
        self.logic.on_node_connected(conn.conn_id(), conn.remote_node_id(), conn.remote_addr(), now_ms);
        Some(Box::new(VisualizationAgentHandler::new(conn.conn_id(), conn.remote_node_id())))
    }

    fn on_outgoing_connection_disconnected(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId) {
        self.logic.on_node_disconnected(conn_id, node_id, now_ms);
    }

    fn on_outgoing_connection_error(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId, err: &OutgoingConnectionError) {}

    fn on_stopped(&mut self, ctx: &BehaviorContext, now_ms: u64) {}

    fn pop_action(&mut self) -> Option<NetworkBehaviorAction<HE, SE>> {
        self.queue_action.pop_front()
    }
}
