mod api;
mod cli;
mod multiplex_service;
mod core;

use axum::{routing::get, Router};
use log::info;
use structopt::StructOpt;

use crate::api::connection_state::ConnectionStateService;
use crate::api::k8s::healthcheck;
use crate::cli::CliOptions;
use crate::core::redis::create_redis_conection_pool;
use crate::multiplex_service::MultiplexService;

mod proto {
    tonic::include_proto!("connection_state");
    pub(crate) const CONNECTION_STATE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("connection_state_descriptor");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let opts = CliOptions::from_args();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let redis_connection_pool = create_redis_conection_pool(&opts).await;

    // build the rest service
    let rest = Router::new().route("/health", get(healthcheck));

    // build the grpc service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::CONNECTION_STATE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let grpc = tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(proto::connection_state_manager_server::ConnectionStateManagerServer::new(
            ConnectionStateService::new(redis_connection_pool),
        ))
        .into_service();

    // combine them into one service
    let service = MultiplexService::new(rest, grpc);

    info!("Listening {0}:{1}...", opts.host, opts.port);
    let addr = format!("{}:{}", opts.host, opts.port);
    let socket = &addr.parse().unwrap();
    hyper::Server::bind(socket)
        .serve(tower::make::Shared::new(service))
        .await
        .unwrap();

    Ok(())
}