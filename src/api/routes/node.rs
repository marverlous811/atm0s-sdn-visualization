use std::future::Future;

use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::api::http_server::AppState;

#[derive(Deserialize, Serialize, Clone)]
pub struct NetworkGraphNode {
    pub id: String,
    pub name: String,
    pub group: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NetworkGraphLink {
    pub source: String,
    pub target: String,
    pub value: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NetworkGraph {
    pub nodes: Vec<NetworkGraphNode>,
    pub links: Vec<NetworkGraphLink>,
}

pub async fn get_network_graph(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let nodes = state.sdk.clone().get_nodes();
    let mut graph_nodes = Vec::<NetworkGraphNode>::new();
    let mut graph_links = Vec::<NetworkGraphLink>::new();
    //TODO: optimize by using graph data structure
    for node in nodes.iter() {
        for trans in node.transports.iter() {
            graph_nodes.push(NetworkGraphNode {
                id: trans.addr.clone(),
                name: trans.addr.clone(),
                group: node.node_id.to_string(),
            });
            for conn in trans.connections.iter() {
                graph_links.push(NetworkGraphLink {
                    source: trans.addr.clone(),
                    target: conn.addr.clone(),
                    value: conn.direction as u32,
                })
            }
        }
    }

    web::Json(NetworkGraph {
        nodes: graph_nodes,
        links: graph_links,
    })
}
