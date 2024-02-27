use atm0s_sdn::KeyValueBehavior;
use atm0s_sdn::KeyValueBehaviorEvent;
use atm0s_sdn::KeyValueHandlerEvent;
use atm0s_sdn::KeyValueSdk;
use atm0s_sdn::KeyValueSdkEvent;
use atm0s_sdn::NodeAddr;
use atm0s_sdn::SharedRouter;
use atm0s_sdn::SystemTimer;
use atm0s_sdn::{convert_enum, NetworkPlane, NetworkPlaneConfig};
use atm0s_sdn::{LayersSpreadRouterSyncBehavior, LayersSpreadRouterSyncBehaviorEvent, LayersSpreadRouterSyncHandlerEvent};
use atm0s_sdn::{ManualBehavior, ManualBehaviorConf, ManualBehaviorEvent, ManualHandlerEvent};
use atm0s_sdn::{NodeAddrBuilder, UdpTransport};
use atm0s_sdn_visualization::build_visualization_route;
use atm0s_sdn_visualization::SdnMonitorController;
use atm0s_sdn_visualization::VisualizationAgentBehaviour;
use atm0s_sdn_visualization::VisualizationAgentBehaviourConf;
use atm0s_sdn_visualization::VisualizationAgentBehaviourEvent;
use atm0s_sdn_visualization::VisualizationAgentHandlerEvent;
use atm0s_sdn_visualization::VisualizationMasterBehaviour;
use atm0s_sdn_visualization::VisualizationMasterBehaviourEvent;
use atm0s_sdn_visualization::VisualizationMasterHandlerEvent;
use clap::ArgAction;
use clap::ArgMatches;
use clap::{arg, Parser};
use poem::listener::TcpListener;
use poem::middleware::Tracing;
use poem::EndpointExt;
use poem::Route;
use poem::Server;
use reedline_repl_rs::{clap::Command, Error, Repl};
use std::sync::Arc;

#[derive(convert_enum::From, convert_enum::TryInto)]
enum NodeBehaviorEvent {
    Manual(ManualBehaviorEvent),
    LayersSpreadRouterSync(LayersSpreadRouterSyncBehaviorEvent),
    KeyValue(KeyValueBehaviorEvent),
    VisualizationAgent(VisualizationAgentBehaviourEvent),
    VisualizationMaster(VisualizationMasterBehaviourEvent),
}

#[derive(convert_enum::From, convert_enum::TryInto)]
enum NodeHandleEvent {
    Manual(ManualHandlerEvent),
    LayersSpreadRouterSync(LayersSpreadRouterSyncHandlerEvent),
    KeyValue(KeyValueHandlerEvent),
    VisualizationAgent(VisualizationAgentHandlerEvent),
    VisualizationMaster(VisualizationMasterHandlerEvent),
}

#[derive(convert_enum::From, convert_enum::TryInto)]
enum NodeSdkEvent {
    KeyValue(KeyValueSdkEvent),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(env, long, short, action=ArgAction::SetTrue)]
    is_master: bool,

    #[arg(env, long)]
    node_id: u32,

    /// Neighbors
    #[arg(env, long)]
    seeds: Vec<NodeAddr>,
}

struct Context {
    node_id: u32,
    addr: NodeAddr,
    router: SharedRouter,
    sdn_controller: Option<SdnMonitorController>,
}

fn print_route_table(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
    context.router.print_dump();
    Ok(None)
}

fn print_node_info(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
    println!("current node id: {}, addr: {}", context.node_id, context.addr);
    Ok(None)
}

async fn generate_sdn_plane(args: Args, controller: Option<SdnMonitorController>) -> (NetworkPlane<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>, SharedRouter, NodeAddr) {
    let mut node_addr_builder = NodeAddrBuilder::new(args.node_id);
    // Create transport layer
    // The port number is 50000 + node_id
    let secure = Arc::new(atm0s_sdn::StaticKeySecure::new("secure-token"));
    let socket = UdpTransport::prepare(50000 + args.node_id as u16, &mut node_addr_builder).await;
    let transport = UdpTransport::new(node_addr_builder.addr(), socket, secure);
    let node_addr = node_addr_builder.addr();
    println!("Listenning on addr {}", node_addr);

    let timer = Arc::new(SystemTimer());
    let router = SharedRouter::new(args.node_id);
    let manual = ManualBehavior::new(ManualBehaviorConf {
        node_id: args.node_id,
        node_addr: node_addr.clone(),
        seeds: args.seeds.clone(),
        local_tags: vec![],
        connect_tags: vec![],
    });
    let spreads_layer_router = LayersSpreadRouterSyncBehavior::new(router.clone());

    let key_value_sdk = KeyValueSdk::new();
    let key_value = KeyValueBehavior::new(args.node_id, 1000, Some(Box::new(key_value_sdk.clone())));

    let visualization_agent = VisualizationAgentBehaviour::new(VisualizationAgentBehaviourConf {
        node_id: args.node_id,
        node_addr: node_addr.clone(),
    });

    let plan_cfg = match controller {
        Some(controller) => {
            let (visualization_master, _) = VisualizationMasterBehaviour::new(controller.clone());
            NetworkPlaneConfig {
                router: Arc::new(router.clone()),
                node_id: args.node_id,
                tick_ms: 1000,
                behaviors: vec![
                    Box::new(manual),
                    Box::new(spreads_layer_router),
                    Box::new(key_value),
                    Box::new(visualization_agent),
                    Box::new(visualization_master),
                ],
                transport: Box::new(transport),
                timer,
            }
        }
        None => NetworkPlaneConfig {
            router: Arc::new(router.clone()),
            node_id: args.node_id,
            tick_ms: 1000,
            behaviors: vec![Box::new(manual), Box::new(spreads_layer_router), Box::new(key_value), Box::new(visualization_agent)],
            transport: Box::new(transport),
            timer,
        },
    };
    let plane = NetworkPlane::<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>::new(plan_cfg);

    return (plane, router, node_addr);
}

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Error).format_timestamp_millis().init();
    let args = Args::parse();
    println!("args is master {}", args.is_master);
    let node_id = args.node_id;
    let is_master = args.is_master;

    let (route, controller) = if is_master {
        let (route, controller) = build_visualization_route();
        (Some(route), Some(controller))
    } else {
        (None, None)
    };

    let (mut plane, router, addr) = generate_sdn_plane(args, controller.clone()).await;
    let _ = tokio::task::spawn(async move {
        plane.started();
        while let Ok(_) = plane.recv().await {}
        plane.stopped();
    });

    let context = Context {
        node_id,
        addr,
        router,
        sdn_controller: controller,
    };

    match route {
        Some(route) => {
            let _ = tokio::task::spawn(async move {
                let app = Route::new().nest("/", route).with(Tracing);
                Server::new(TcpListener::bind("0.0.0.0:8080")).run(app).await;
            });
        }
        None => {}
    };

    let mut repl = Repl::new(context)
        .with_name("Sample visualization")
        .with_command(Command::new("info").about("Get Node Info"), print_node_info)
        .with_command(Command::new("router").about("Print router table"), print_route_table);
    let _ = repl.run_async().await;
}
