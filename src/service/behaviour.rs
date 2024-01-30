use atm0s_sdn_identity::{ConnId, NodeAddr, NodeId};
use atm0s_sdn_network::{
    behaviour::{NetworkBehavior, NetworkBehaviorAction},
    msg::{MsgHeader, TransportMsg},
};
use atm0s_sdn_router::RouteRule;
use atm0s_sdn_utils::vec_dequeue::VecDeque;

use super::{
    handler::VisualizationConnectionHandler,
    logic::VisualizationLogic,
    msg::{VisualizationBehaviorEvent, VisualizationControllAction, VisualizationHandlerEvent, VisualizationMsg, VisualziationConf},
    sdk::VisualizationSdk,
    VISUALIZATION_AGENT_SERVICE, VISUALIZATION_MASTER_SERVICE,
};

pub struct VisualizationBehavior<HE, SE> {
    node_id: NodeId,
    service_id: u8,
    logic: VisualizationLogic,
    queue_action: VecDeque<NetworkBehaviorAction<HE, SE>>,
}

impl<HE, SE> VisualizationBehavior<HE, SE> {
    pub fn new(cfg: VisualziationConf) -> (Self, VisualizationSdk) {
        let service_id = if cfg.is_master {
            VISUALIZATION_MASTER_SERVICE
        } else {
            VISUALIZATION_AGENT_SERVICE
        };
        let logic = VisualizationLogic::new(cfg.clone());
        let sdk = VisualizationSdk::new(logic.clone());

        (
            Self {
                node_id: cfg.node_id,
                service_id,
                logic: logic,
                queue_action: VecDeque::new(),
            },
            sdk,
        )
    }

    fn process_logic_action(&mut self) {
        while let Some(action) = self.logic.pop_action() {
            let msg = match action {
                VisualizationControllAction::NodeStats(id, addr, ts, neighbour_ids) => {
                    let header = MsgHeader::build(self.service_id, VISUALIZATION_MASTER_SERVICE, RouteRule::ToService(VISUALIZATION_MASTER_SERVICE as u32));
                    let payload = VisualizationMsg::NodeStats(id, addr.to_vec(), ts, neighbour_ids);
                    TransportMsg::from_payload_bincode(header, &payload)
                }
            };
            self.queue_action.push_back(NetworkBehaviorAction::ToNet(msg));
        }
    }

    fn process_visalization_msg(&mut self, msg: VisualizationMsg) {
        let action = match msg {
            VisualizationMsg::NodeStats(id, add_vec, ts, neighbour_ids) => {
                if let Some(addr) = NodeAddr::from_vec(&add_vec) {
                    if id != self.node_id {
                        Some(VisualizationControllAction::NodeStats(id, addr, ts, neighbour_ids))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };
        match action {
            Some(action) => self.logic.execute_action(action),
            None => {}
        }
    }
}

impl<BE, HE, SE> NetworkBehavior<BE, HE, SE> for VisualizationBehavior<HE, SE>
where
    BE: From<VisualizationBehaviorEvent> + TryInto<VisualizationBehaviorEvent> + Send + Sync + 'static,
    HE: From<VisualizationHandlerEvent> + TryInto<VisualizationHandlerEvent> + Send + Sync + 'static,
    SE: Send + Sync + 'static,
{
    fn service_id(&self) -> u8 {
        return self.service_id;
    }

    fn on_started(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, now_ms: u64) {
        self.logic.report_stats(now_ms);
    }

    fn on_tick(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, now_ms: u64, _interval_ms: u64) {
        self.process_logic_action();
        self.logic.report_stats(now_ms);
    }

    fn on_awake(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, _now_ms: u64) {}

    fn on_sdk_msg(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, _now_ms: u64, _from_service: u8, _event: SE) {}

    fn check_incoming_connection(
        &mut self,
        _ctx: &atm0s_sdn_network::behaviour::BehaviorContext,
        _now_ms: u64,
        _node: NodeId,
        _conn_id: ConnId,
    ) -> Result<(), atm0s_sdn_network::transport::ConnectionRejectReason> {
        Ok(())
    }

    fn check_outgoing_connection(
        &mut self,
        _ctx: &atm0s_sdn_network::behaviour::BehaviorContext,
        _now_ms: u64,
        _node: NodeId,
        _conn_id: ConnId,
    ) -> Result<(), atm0s_sdn_network::transport::ConnectionRejectReason> {
        Ok(())
    }

    fn on_local_msg(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, _now_ms: u64, _msg: atm0s_sdn_network::msg::TransportMsg) {}

    fn on_incoming_connection_connected(
        &mut self,
        _ctx: &atm0s_sdn_network::behaviour::BehaviorContext,
        now_ms: u64,
        conn: std::sync::Arc<dyn atm0s_sdn_network::transport::ConnectionSender>,
    ) -> Option<Box<dyn atm0s_sdn_network::behaviour::ConnectionHandler<BE, HE>>> {
        self.logic.on_node_connected(conn.remote_node_id(), conn.remote_addr(), now_ms);
        Some(Box::new(VisualizationConnectionHandler::new()))
    }

    fn on_incoming_connection_disconnected(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, now_ms: u64, node_id: NodeId, _conn_id: ConnId) {
        self.logic.on_node_disconnected(node_id, now_ms);
    }

    fn on_outgoing_connection_connected(
        &mut self,
        _ctx: &atm0s_sdn_network::behaviour::BehaviorContext,
        _now_ms: u64,
        _conn: std::sync::Arc<dyn atm0s_sdn_network::transport::ConnectionSender>,
    ) -> Option<Box<dyn atm0s_sdn_network::behaviour::ConnectionHandler<BE, HE>>> {
        Some(Box::new(VisualizationConnectionHandler::new()))
    }

    fn on_outgoing_connection_disconnected(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, _now_ms: u64, _node_id: NodeId, _conn_id: ConnId) {}

    fn on_outgoing_connection_error(
        &mut self,
        _ctx: &atm0s_sdn_network::behaviour::BehaviorContext,
        _now_ms: u64,
        _node_id: NodeId,
        _conn_id: ConnId,
        _err: &atm0s_sdn_network::transport::OutgoingConnectionError,
    ) {
    }

    fn on_handler_event(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, _now_ms: u64, _node_id: NodeId, _conn_id: ConnId, event: BE) {
        let msg: Result<VisualizationBehaviorEvent, _> = event.try_into();
        match msg {
            Ok(msg) => match msg {
                VisualizationBehaviorEvent::OnMsg(payload) => self.process_visalization_msg(payload),
            },
            Err(_e) => {}
        }
    }

    fn on_stopped(&mut self, _ctx: &atm0s_sdn_network::behaviour::BehaviorContext, _now_ms: u64) {}

    fn pop_action(&mut self) -> Option<NetworkBehaviorAction<HE, SE>> {
        self.queue_action.pop_front()
    }
}
