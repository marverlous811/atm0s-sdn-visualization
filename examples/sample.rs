// use std::sync::Arc;

// use async_std::process;
// use atm0s_sdn::{
//     compose_transport, convert_enum, KeyValueBehavior, KeyValueBehaviorEvent, KeyValueHandlerEvent, KeyValueSdk, KeyValueSdkEvent, LayersSpreadRouterSyncBehavior, LayersSpreadRouterSyncBehaviorEvent,
//     LayersSpreadRouterSyncHandlerEvent, ManualBehavior, ManualBehaviorConf, ManualBehaviorEvent, ManualHandlerEvent, NetworkPlane, NetworkPlaneConfig, NodeAddr, NodeAddrBuilder, NodeId, SharedRouter,
//     SystemTimer, TcpTransport, UdpTransport,
// };
// use atm0s_sdn_utils::hashmap::HashMap;
// use atm0s_sdn_visualization::{
//     VisualizationAgentBehaviour, VisualizationAgentBehaviourConf, VisualizationAgentBehaviourEvent, VisualizationAgentHandlerEvent, VisualizationMasterBehaviour, VisualizationMasterBehaviourEvent,
//     VisualizationMasterHandlerEvent, VisualizationMasterSdk,
// };
// use clap::{Arg, ArgMatches, Command};
// use reedline_repl_rs::{Error, Repl};

// #[derive(convert_enum::From, convert_enum::TryInto)]
// enum NodeBehaviorEvent {
//     Manual(ManualBehaviorEvent),
//     LayersSpreadRouterSync(LayersSpreadRouterSyncBehaviorEvent),
//     KeyValue(KeyValueBehaviorEvent),
//     VisualizationAgent(VisualizationAgentBehaviourEvent),
//     VisualizationMaster(VisualizationMasterBehaviourEvent),
// }

// #[derive(convert_enum::From, convert_enum::TryInto)]
// enum NodeHandleEvent {
//     Manual(ManualHandlerEvent),
//     LayersSpreadRouterSync(LayersSpreadRouterSyncHandlerEvent),
//     KeyValue(KeyValueHandlerEvent),
//     VisualizationAgent(VisualizationAgentHandlerEvent),
//     VisualizationMaster(VisualizationMasterHandlerEvent),
// }

// #[derive(convert_enum::From, convert_enum::TryInto)]
// enum NodeSdkEvent {
//     KeyValue(KeyValueSdkEvent),
// }

// struct NodeContext {
//     addr: NodeAddr,
//     master_sdk: Option<VisualizationMasterSdk>,
// }

// struct Context {
//     nodes: HashMap<NodeId, NodeContext>,
// }

// compose_transport!(UdpTcpTransport, udp: UdpTransport, tcp: TcpTransport);

// async fn spawn_node(node_id: NodeId, is_master: bool) -> NodeContext {
//     let mut node_addr_builder = NodeAddrBuilder::new(node_id);
//     let secure = Arc::new(atm0s_sdn::StaticKeySecure::new("secure-token"));
//     let udp_socket = UdpTransport::prepare(50000 + node_id as u16, &mut node_addr_builder).await;
//     let tcp_listener = TcpTransport::prepare(40000 + node_id as u16, &mut node_addr_builder).await;
//     let udp_transport = UdpTransport::new(node_addr_builder.addr(), udp_socket, secure.clone());
//     let tcp_transport = TcpTransport::new(node_addr_builder.addr(), tcp_listener, secure);
//     let transport = UdpTcpTransport::new(udp_transport, tcp_transport);
//     let node_addr = node_addr_builder.addr();

//     let mut retval = NodeContext {
//         addr: node_addr.clone(),
//         master_sdk: None,
//     };

//     let timer = Arc::new(SystemTimer());
//     let router = SharedRouter::new(node_id);
//     let manual = ManualBehavior::new(ManualBehaviorConf {
//         node_id: node_id,
//         node_addr: node_addr.clone(),
//         seeds: vec![],
//         local_tags: vec![],
//         connect_tags: vec![],
//     });
//     let spreads_layer_router = LayersSpreadRouterSyncBehavior::new(router.clone());

//     let key_value_sdk = KeyValueSdk::new();
//     let key_value = KeyValueBehavior::new(node_id, 1000, Some(Box::new(key_value_sdk.clone())));

//     let visualization_agent = VisualizationAgentBehaviour::new(VisualizationAgentBehaviourConf {
//         node_id: node_id,
//         node_addr: node_addr.clone(),
//     });

//     let plan_cfg = if is_master == true {
//         let (master_behaviour, master_sdk) = VisualizationMasterBehaviour::new();
//         retval.master_sdk = Some(master_sdk.clone());
//         NetworkPlaneConfig {
//             router: Arc::new(router.clone()),
//             node_id: node_id,
//             tick_ms: 1000,
//             behaviors: vec![
//                 Box::new(manual),
//                 Box::new(spreads_layer_router),
//                 Box::new(key_value),
//                 Box::new(visualization_agent),
//                 Box::new(master_behaviour),
//             ],
//             transport: Box::new(transport),
//             timer,
//         }
//     } else {
//         NetworkPlaneConfig {
//             router: Arc::new(router.clone()),
//             node_id: node_id,
//             tick_ms: 1000,
//             behaviors: vec![Box::new(manual), Box::new(spreads_layer_router), Box::new(key_value), Box::new(visualization_agent)],
//             transport: Box::new(transport),
//             timer,
//         }
//     };
//     let mut plane = NetworkPlane::<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>::new(plan_cfg);

//     let _ = async_std::task::spawn(async move {
//         plane.started();
//         while let Ok(_) = plane.recv().await {}
//         plane.stopped();
//     });
//     retval
// }

// async fn create_node(args: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     let node_id_str = args.get_one::<String>("node_id").unwrap();
//     let node_id: NodeId = match node_id_str.trim().parse() {
//         Ok(num) => num,
//         _ => 0,
//     };
//     let is_master_str = args.get_one::<String>("is_master").unwrap();
//     let is_master = match is_master_str.trim().parse::<u8>() {
//         Ok(num) => {
//             if num == 1 {
//                 true
//             } else {
//                 false
//             }
//         }
//         _ => false,
//     };

//     match context.nodes.contains_key(&node_id) {
//         true => {
//             println!("node {} is existed", node_id);
//         }
//         false => {
//             println!("create new node node_id: {}, is_master {}", node_id, is_master);
//             let node_ctx = spawn_node(node_id, is_master).await;
//             context.nodes.insert(node_id, node_ctx);
//         }
//     }
//     Ok(None)
// }

// async fn remove_node(args: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     let node_id_str = args.get_one::<String>("node_id").unwrap();
//     let node_id: NodeId = match node_id_str.trim().parse() {
//         Ok(num) => num,
//         _ => 0,
//     };

//     // match context.nodes.get_mut(&node_id) {
//     //     Some(node) => {
//     //         match node.task {
//     //             Some(task) => {
//     //                 task.cancel();
//     //             }
//     //             None => {}
//     //         };
//     //     }
//     //     None => {}
//     // }

//     Ok(None)
// }

// async fn master_commands(args: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     let node_id_str = args.get_one::<String>("node_id").unwrap();
//     let node_id: NodeId = match node_id_str.trim().parse() {
//         Ok(num) => num,
//         _ => 0,
//     };

//     match context.nodes.get_mut(&node_id) {
//         Some(node) => match node.master_sdk.clone() {
//             Some(sdk) => {
//                 sdk.dump_graph();
//             }
//             None => {
//                 println!("node {} is not master node", node_id);
//             }
//         },
//         None => {
//             println!("node {} not found...", node_id);
//         }
//     }

//     Ok(None)
// }

// fn exit(args: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     process::exit(0);
// }

// #[async_std::main]
// async fn main() {
//     let ctx = Context { nodes: HashMap::new() };
//     let mut repl = Repl::new(ctx)
//         .with_name("Visualization Sample Cli Demo")
//         .with_command_async(
//             Command::new("create_node").arg(Arg::new("node_id").required(true)).arg(Arg::new("is_master")).about("Create node "),
//             |args, context| Box::pin(create_node(args, context)),
//         )
//         .with_command_async(Command::new("cmd").arg(Arg::new("node_id").required(true)).about("master controller command"), |args, context| {
//             Box::pin(master_commands(args, context))
//         })
//         .with_command(Command::new("exit").about("kill process"), exit);

//     let _ = repl.run_async().await;
// }

fn main() {}
