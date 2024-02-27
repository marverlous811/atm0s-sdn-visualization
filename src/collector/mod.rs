mod controller;
mod storage;

pub use controller::SdnMonitorController;
use poem::{
    get, handler,
    web::{Data, Json},
    EndpointExt, Route,
};

#[cfg(not(feature = "embed"))]
use poem::endpoint::StaticFilesEndpoint;

#[cfg(feature = "embed")]
use poem::endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint};

use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
pub use storage::{NodeConnectionData, NodeData};

#[cfg(feature = "embed")]
#[derive(RustEmbed)]
#[folder = "public"]
pub struct Files;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NetworkGraphNode {
    pub nodes: Vec<NodeData>,
}

#[handler]
fn fetch_all_nodes(Data(controller): Data<&SdnMonitorController>) -> Json<NetworkGraphNode> {
    let nodes = controller.get_nodes();
    let data = NetworkGraphNode { nodes };
    Json(data)
}

pub fn build_visualization_route() -> (Route, SdnMonitorController) {
    let controller = SdnMonitorController::new();
    let route = Route::new().at("/api/nodes", get(fetch_all_nodes.data(controller.clone())));

    #[cfg(not(feature = "embed"))]
    let route = route.nest("/", StaticFilesEndpoint::new("./public/").show_files_listing());

    #[cfg(feature = "embed")]
    let route = route.at("/", EmbeddedFileEndpoint::<Files>::new("index.html"));
    #[cfg(feature = "embed")]
    let route = route.nest("/", EmbeddedFilesEndpoint::<Files>::new());

    (route, controller)
}
