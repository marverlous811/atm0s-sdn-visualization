// use atm0s_sdn::compose_transport;
// use atm0s_sdn::KeyValueBehavior;
// use atm0s_sdn::KeyValueBehaviorEvent;
// use atm0s_sdn::KeyValueHandlerEvent;
// use atm0s_sdn::KeyValueSdk;
// use atm0s_sdn::KeyValueSdkEvent;
// use atm0s_sdn::NodeAddr;
// use atm0s_sdn::SharedRouter;
// use atm0s_sdn::SystemTimer;
// use atm0s_sdn::TcpTransport;
// use atm0s_sdn::{convert_enum, NetworkPlane, NetworkPlaneConfig};
// use atm0s_sdn::{LayersSpreadRouterSyncBehavior, LayersSpreadRouterSyncBehaviorEvent, LayersSpreadRouterSyncHandlerEvent};
// use atm0s_sdn::{ManualBehavior, ManualBehaviorConf, ManualBehaviorEvent, ManualHandlerEvent};
// use atm0s_sdn::{NodeAddrBuilder, UdpTransport};
// use atm0s_sdn_visualization::Server;
// use atm0s_sdn_visualization::ServerConf;
// use atm0s_sdn_visualization::VisualizationAgentBehaviour;
// use atm0s_sdn_visualization::VisualizationAgentBehaviourConf;
// use atm0s_sdn_visualization::VisualizationAgentBehaviourEvent;
// use atm0s_sdn_visualization::VisualizationAgentHandlerEvent;
// use atm0s_sdn_visualization::VisualizationMasterBehaviour;
// use atm0s_sdn_visualization::VisualizationMasterBehaviourEvent;
// use atm0s_sdn_visualization::VisualizationMasterHandlerEvent;
// use atm0s_sdn_visualization::VisualizationMasterSdk;
// use clap::Arg;
// use clap::ArgAction;
// use clap::ArgMatches;
// use clap::{arg, Parser};
// use reedline_repl_rs::{clap::Command, Error, Repl};
// use std::sync::Arc;

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

// compose_transport!(UdpTcpTransport, udp: UdpTransport, tcp: TcpTransport);

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(env, long, short, action=ArgAction::SetTrue)]
//     is_master: bool,

//     #[arg(env, long)]
//     node_id: u32,

//     /// Neighbors
//     #[arg(env, long)]
//     seeds: Vec<NodeAddr>,
// }

// struct Context {
//     node_id: u32,
//     node_addr: NodeAddr,
//     router: SharedRouter,
//     key_value_sdk: KeyValueSdk,
//     visualization_sdk: Option<VisualizationMasterSdk>,
// }

// async fn connect(args: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     let sdk = context.key_value_sdk.clone();
//     let addr = args.get_one::<String>("addr").unwrap();
//     sdk.hset(context.node_id as u64, 1, addr.as_bytes().to_vec().into(), None);
//     Ok(None)
// }

// fn print_route_table(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     context.router.print_dump();
//     Ok(None)
// }

// fn print_node_info(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     println!("current node id: {}, addr: {}", context.node_id, context.node_addr);
//     Ok(None)
// }

// async fn print_dump_graph(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
//     match context.visualization_sdk.clone() {
//         Some(sdk) => {
//             sdk.dump_graph();
//         }
//         None => {}
//     }
//     Ok(None)
// }

// async fn master_process(args: Args) -> (NetworkPlane<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>, Repl<Context, Error>, Option<Server>) {
//     let mut node_addr_builder = NodeAddrBuilder::new(args.node_id);
//     // Create transport layer
//     // The port number is 50000 + node_id
//     let secure = Arc::new(atm0s_sdn::StaticKeySecure::new("secure-token"));
//     let udp_socket = UdpTransport::prepare(50000 + args.node_id as u16, &mut node_addr_builder).await;
//     let tcp_listener = TcpTransport::prepare(40000 + args.node_id as u16, &mut node_addr_builder).await;
//     let udp_transport = UdpTransport::new(node_addr_builder.addr(), udp_socket, secure.clone());
//     let tcp_transport = TcpTransport::new(node_addr_builder.addr(), tcp_listener, secure);
//     let transport = UdpTcpTransport::new(udp_transport, tcp_transport);
//     let node_addr = node_addr_builder.addr();
//     println!("Listenning on addr {}", node_addr);

//     let timer = Arc::new(SystemTimer());
//     let router = SharedRouter::new(args.node_id);
//     let manual = ManualBehavior::new(ManualBehaviorConf {
//         node_id: args.node_id,
//         node_addr: node_addr.clone(),
//         seeds: args.seeds.clone(),
//         local_tags: vec![],
//         connect_tags: vec![],
//     });
//     let spreads_layer_router = LayersSpreadRouterSyncBehavior::new(router.clone());

//     let key_value_sdk = KeyValueSdk::new();
//     let key_value = KeyValueBehavior::new(args.node_id, 1000, Some(Box::new(key_value_sdk.clone())));

//     let visualization_agent = VisualizationAgentBehaviour::new(VisualizationAgentBehaviourConf {
//         node_id: args.node_id,
//         node_addr: node_addr.clone(),
//     });
//     let (master_behaviour, master_sdk) = VisualizationMasterBehaviour::new();

//     let plan_cfg = NetworkPlaneConfig {
//         router: Arc::new(router.clone()),
//         node_id: args.node_id,
//         tick_ms: 1000,
//         behaviors: vec![
//             Box::new(manual),
//             Box::new(spreads_layer_router),
//             Box::new(key_value),
//             Box::new(visualization_agent),
//             Box::new(master_behaviour),
//         ],
//         transport: Box::new(transport),
//         timer,
//     };
//     let plane = NetworkPlane::<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>::new(plan_cfg);

//     let context = Context {
//         node_id: args.node_id,
//         node_addr: node_addr_builder.addr(),
//         router: router.clone(),
//         key_value_sdk: key_value_sdk.clone(),
//         visualization_sdk: Some(master_sdk.clone()),
//     };

//     let server = Server::new(ServerConf { port: 8080 }, master_sdk.clone());

//     let repl = Repl::new(context)
//         .with_name("[Master Node] Sample visualization")
//         .with_command_async(Command::new("connect").arg(Arg::new("addr").required(true)).about("Connect to node addr"), |args, context| {
//             Box::pin(connect(args, context))
//         })
//         .with_command_async(Command::new("graph").about("Print dump graph"), |args, context| Box::pin(print_dump_graph(args, context)))
//         .with_command(Command::new("info").about("Get Node Info"), print_node_info)
//         .with_command(Command::new("router").about("Print router table"), print_route_table);
//     return (plane, repl, Some(server));
// }

// async fn normal_process(args: Args) -> (NetworkPlane<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>, Repl<Context, Error>, Option<Server>) {
//     let mut node_addr_builder = NodeAddrBuilder::new(args.node_id);
//     // Create transport layer
//     // The port number is 50000 + node_id
//     let secure = Arc::new(atm0s_sdn::StaticKeySecure::new("secure-token"));
//     let udp_socket = UdpTransport::prepare(50000 + args.node_id as u16, &mut node_addr_builder).await;
//     let tcp_listener = TcpTransport::prepare(40000 + args.node_id as u16, &mut node_addr_builder).await;
//     let udp_transport = UdpTransport::new(node_addr_builder.addr(), udp_socket, secure.clone());
//     let tcp_transport = TcpTransport::new(node_addr_builder.addr(), tcp_listener, secure);
//     let transport = UdpTcpTransport::new(udp_transport, tcp_transport);
//     let node_addr = node_addr_builder.addr();
//     println!("Listenning on addr {}", node_addr);

//     let timer = Arc::new(SystemTimer());
//     let router = SharedRouter::new(args.node_id);
//     let manual = ManualBehavior::new(ManualBehaviorConf {
//         node_id: args.node_id,
//         node_addr: node_addr.clone(),
//         seeds: args.seeds.clone(),
//         local_tags: vec![],
//         connect_tags: vec![],
//     });
//     let spreads_layer_router = LayersSpreadRouterSyncBehavior::new(router.clone());

//     let key_value_sdk = KeyValueSdk::new();
//     let key_value = KeyValueBehavior::new(args.node_id, 1000, Some(Box::new(key_value_sdk.clone())));

//     let visualization_agent = VisualizationAgentBehaviour::new(VisualizationAgentBehaviourConf {
//         node_id: args.node_id,
//         node_addr: node_addr.clone(),
//     });

//     let plan_cfg = NetworkPlaneConfig {
//         router: Arc::new(router.clone()),
//         node_id: args.node_id,
//         tick_ms: 1000,
//         behaviors: vec![Box::new(manual), Box::new(spreads_layer_router), Box::new(key_value), Box::new(visualization_agent)],
//         transport: Box::new(transport),
//         timer,
//     };
//     let plane = NetworkPlane::<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>::new(plan_cfg);

//     let context = Context {
//         node_id: args.node_id,
//         node_addr: node_addr_builder.addr(),
//         router: router.clone(),
//         key_value_sdk: key_value_sdk.clone(),
//         visualization_sdk: None,
//     };

//     let repl = Repl::new(context)
//         .with_name("[Agent Node] Sample visualization")
//         .with_command_async(Command::new("connect").arg(Arg::new("addr").required(true)).about("Connect to node addr"), |args, context| {
//             Box::pin(connect(args, context))
//         })
//         .with_command(Command::new("info").about("Get Node Info"), print_node_info)
//         .with_command(Command::new("router").about("Print router table"), print_route_table);
//     return (plane, repl, None);
// }

// #[async_std::main]
// async fn main() {
//     env_logger::builder().filter_level(log::LevelFilter::Error).format_timestamp_millis().init();
//     let args = Args::parse();
//     println!("args is master {}", args.is_master);

//     let (mut plane, mut repl, server) = if args.is_master {
//         master_process(args).await
//     } else {
//         normal_process(args).await
//     };

//     let _ = async_std::task::spawn(async move {
//         plane.started();
//         while let Ok(_) = plane.recv().await {}
//         plane.stopped();
//     });
//     match server {
//         Some(mut server) => {
//             let _ = async_std::task::spawn(async move { actix_web::rt::System::new().block_on(async move { server.run().await }) });
//         }
//         None => {}
//     }
//     let _ = repl.run_async().await;
// }

fn main() {}
