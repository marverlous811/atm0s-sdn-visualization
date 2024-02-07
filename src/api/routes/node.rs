use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{api::http_server::AppState, NetworkNodeData};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NetworkGraphNode {
    pub nodes: Vec<NetworkNodeData>,
}

pub async fn get_network_graph(state: web::Data<AppState>) -> impl Responder {
    let nodes = state.sdk.clone().get_nodes();
    HttpResponse::Ok().json(NetworkGraphNode { nodes })
}
