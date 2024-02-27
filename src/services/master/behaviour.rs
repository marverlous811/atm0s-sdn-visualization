use atm0s_sdn_identity::NodeId;
use atm0s_sdn_network::behaviour::{BehaviorContext, ConnectionHandler, NetworkBehavior, NetworkBehaviorAction};
use atm0s_sdn_network::msg::TransportMsg;
use atm0s_sdn_network::transport::{ConnectionRejectReason, ConnectionSender, OutgoingConnectionError};
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use crate::collector::SdnMonitorController;
use crate::{VisualizationAgentMsg, VisualizationMasterSdk, VISUALIZATION_MASTER_SERVICE};

use super::handler::VisualizationMasterHandler;
use super::logic::VisualizationMasterLogic;
use super::msg::{VisualizationMasterBehaviourEvent, VisualizationMasterHandlerEvent};

pub struct VisualizationMasterBehaviour<HE, SE> {
    logic: VisualizationMasterLogic,
    queue_action: VecDeque<NetworkBehaviorAction<HE, SE>>,
}

impl<HE, SE> VisualizationMasterBehaviour<HE, SE> {
    pub fn new(controller: SdnMonitorController) -> (Self, VisualizationMasterSdk) {
        let logic = VisualizationMasterLogic::new(controller.clone());
        let sdk = VisualizationMasterSdk::new(controller);
        (Self { logic, queue_action: VecDeque::new() }, sdk)
    }
}

impl<BE, HE, SE> NetworkBehavior<BE, HE, SE> for VisualizationMasterBehaviour<HE, SE>
where
    BE: From<VisualizationMasterBehaviourEvent> + TryInto<VisualizationMasterBehaviourEvent> + Send + Sync + 'static,
    HE: From<VisualizationMasterHandlerEvent> + TryInto<VisualizationMasterHandlerEvent> + Send + Sync + 'static,
    SE: Send + Sync + 'static,
{
    fn service_id(&self) -> u8 {
        return VISUALIZATION_MASTER_SERVICE;
    }

    fn on_started(&mut self, ctx: &BehaviorContext, now_ms: u64) {}

    fn on_awake(&mut self, ctx: &BehaviorContext, now_ms: u64) {}

    fn on_tick(&mut self, ctx: &BehaviorContext, now_ms: u64, interval_ms: u64) {}

    fn on_local_msg(&mut self, ctx: &BehaviorContext, now_ms: u64, msg: TransportMsg) {
        if let Ok(payload) = msg.get_payload_bincode::<VisualizationAgentMsg>() {
            self.logic.process_agent_msg(payload);
        }
    }

    fn on_handler_event(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId, event: BE) {
        let msg: Result<VisualizationMasterBehaviourEvent, _> = event.try_into();
        match msg {
            Ok(msg) => match msg {
                VisualizationMasterBehaviourEvent::OnMsg(payload) => self.logic.process_agent_msg(payload),
            },
            Err(_e) => {}
        }
    }

    fn on_sdk_msg(&mut self, ctx: &BehaviorContext, now_ms: u64, from_service: u8, event: SE) {}

    fn check_incoming_connection(&mut self, ctx: &BehaviorContext, now_ms: u64, node: NodeId, conn_id: atm0s_sdn_identity::ConnId) -> Result<(), ConnectionRejectReason> {
        Ok(())
    }

    fn on_incoming_connection_connected(&mut self, ctx: &BehaviorContext, now_ms: u64, conn: std::sync::Arc<dyn ConnectionSender>) -> Option<Box<dyn ConnectionHandler<BE, HE>>> {
        Some(Box::new(VisualizationMasterHandler::new()))
    }

    fn on_incoming_connection_disconnected(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId) {}

    fn check_outgoing_connection(&mut self, ctx: &BehaviorContext, now_ms: u64, node: NodeId, conn_id: atm0s_sdn_identity::ConnId) -> Result<(), ConnectionRejectReason> {
        Ok(())
    }

    fn on_outgoing_connection_connected(&mut self, ctx: &BehaviorContext, now_ms: u64, conn: std::sync::Arc<dyn ConnectionSender>) -> Option<Box<dyn ConnectionHandler<BE, HE>>> {
        Some(Box::new(VisualizationMasterHandler::new()))
    }

    fn on_outgoing_connection_disconnected(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId) {}

    fn on_outgoing_connection_error(&mut self, ctx: &BehaviorContext, now_ms: u64, node_id: NodeId, conn_id: atm0s_sdn_identity::ConnId, err: &OutgoingConnectionError) {}

    fn on_stopped(&mut self, ctx: &BehaviorContext, now_ms: u64) {}

    fn pop_action(&mut self) -> Option<NetworkBehaviorAction<HE, SE>> {
        self.queue_action.pop_front()
    }
}
