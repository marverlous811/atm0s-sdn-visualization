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
use atm0s_sdn_visualization::VisualizationBehavior;
use atm0s_sdn_visualization::VisualizationBehaviorEvent;
use atm0s_sdn_visualization::VisualizationHandlerEvent;
use atm0s_sdn_visualization::VisualizationSdk;
use atm0s_sdn_visualization::VisualziationConf;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::{arg, Parser};
use reedline_repl_rs::{clap::Command, Error, Repl};
use std::sync::Arc;

#[derive(convert_enum::From, convert_enum::TryInto)]
enum NodeBehaviorEvent {
    Manual(ManualBehaviorEvent),
    LayersSpreadRouterSync(LayersSpreadRouterSyncBehaviorEvent),
    KeyValue(KeyValueBehaviorEvent),
    Visualization(VisualizationBehaviorEvent),
}

#[derive(convert_enum::From, convert_enum::TryInto)]
enum NodeHandleEvent {
    Manual(ManualHandlerEvent),
    LayersSpreadRouterSync(LayersSpreadRouterSyncHandlerEvent),
    KeyValue(KeyValueHandlerEvent),
    Visualization(VisualizationHandlerEvent),
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
    node_addr: NodeAddr,
    router: SharedRouter,
    key_value_sdk: KeyValueSdk,
    visualization_sdk: VisualizationSdk,
}

async fn connect(args: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
    let sdk = context.key_value_sdk.clone();
    let addr = args.get_one::<String>("addr").unwrap();
    sdk.hset(context.node_id as u64, 1, addr.as_bytes().to_vec().into(), None);
    Ok(None)
}

fn print_route_table(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
    context.router.print_dump();
    Ok(None)
}

fn print_node_info(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
    println!("current node id: {}, addr: {}", context.node_id, context.node_addr);
    Ok(None)
}

async fn print_dump_graph(_: ArgMatches, context: &mut Context) -> Result<Option<String>, Error> {
    context.visualization_sdk.clone().dump_graph();
    Ok(None)
}

#[async_std::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Error).format_timestamp_millis().init();
    let args = Args::parse();
    let mut node_addr_builder = NodeAddrBuilder::new(args.node_id);
    println!("args is master {}", args.is_master);

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
        node_addr,
        seeds: args.seeds.clone(),
        local_tags: vec![],
        connect_tags: vec![],
    });
    let spreads_layer_router = LayersSpreadRouterSyncBehavior::new(router.clone());

    let key_value_sdk = KeyValueSdk::new();
    let key_value = KeyValueBehavior::new(args.node_id, 1000, Some(Box::new(key_value_sdk.clone())));

    let (visualization, visualization_sdk) = VisualizationBehavior::new(VisualziationConf {
        node_id: args.node_id,
        node_addr: node_addr_builder.addr(),
        is_master: args.is_master,
    });

    let plan_cfg = NetworkPlaneConfig {
        router: Arc::new(router.clone()),
        node_id: args.node_id,
        tick_ms: 1000,
        behaviors: vec![Box::new(manual), Box::new(spreads_layer_router), Box::new(key_value), Box::new(visualization)],
        transport: Box::new(transport),
        timer,
    };
    let mut plane = NetworkPlane::<NodeBehaviorEvent, NodeHandleEvent, NodeSdkEvent>::new(plan_cfg);
    let _ = async_std::task::spawn(async move {
        plane.started();
        while let Ok(_) = plane.recv().await {}
        plane.stopped();
    });

    let context = Context {
        node_id: args.node_id,
        node_addr: node_addr_builder.addr(),
        router: router.clone(),
        key_value_sdk: key_value_sdk.clone(),
        visualization_sdk: visualization_sdk,
    };

    let mut repl = Repl::new(context)
        .with_name("Sample visualization")
        .with_command_async(Command::new("connect").arg(Arg::new("addr").required(true)).about("Connect to node addr"), |args, context| {
            Box::pin(connect(args, context))
        })
        .with_command_async(Command::new("graph").about("Print dump graph"), |args, context| Box::pin(print_dump_graph(args, context)))
        .with_command(Command::new("info").about("Get Node Info"), print_node_info)
        .with_command(Command::new("router").about("Print router table"), print_route_table);
    let _ = repl.run_async().await;
}
