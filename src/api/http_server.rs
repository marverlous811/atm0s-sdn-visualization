use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::VisualizationMasterSdk;

use super::routes::get_network_graph;

pub struct ServerConf {
    pub port: u16,
}

pub struct Server {
    conf: ServerConf,
    sdk: VisualizationMasterSdk,
}

pub struct AppState {
    pub sdk: VisualizationMasterSdk,
}

impl AppState {
    pub fn new(sdk: VisualizationMasterSdk) -> Self {
        Self { sdk }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self { sdk: self.sdk.clone() }
    }
}

#[derive(Deserialize, Serialize)]
pub struct HealthCheckResponse {
    version: String,
}

#[get("/heathcheck")]
async fn healthcheck() -> impl Responder {
    web::Json(HealthCheckResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

impl Server {
    pub fn new(conf: ServerConf, sdk: VisualizationMasterSdk) -> Self {
        Self { conf, sdk }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let app_state = AppState::new(self.sdk.clone());
        HttpServer::new(move || {
            let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header().send_wildcard();
            App::new()
                .wrap(actix_web::middleware::Logger::default())
                .wrap(cors)
                .app_data(web::Data::new(app_state.clone()))
                .service(healthcheck)
                .service(web::resource("/nodes").route(web::get().to(get_network_graph)))
        })
        .bind(("0.0.0.0", self.conf.port))
        .unwrap()
        .run()
        .await
    }
}
