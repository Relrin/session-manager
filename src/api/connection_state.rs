use deadpool_redis_cluster::Pool as RedisClusterConnectionPool;
use tonic::{Request, Response, Status};

use crate::proto::{
    connection_state_manager_server::ConnectionStateManager,
    SetConnectionStateRequest, SetConnectionStateResponse,
    GetConnectionStateRequest, GetConnectionStateResponse,
};

pub struct ConnectionStateService {
    redis: RedisClusterConnectionPool,
}

impl ConnectionStateService {
    pub fn new(redis: RedisClusterConnectionPool) -> Self {
        Self { redis }
    }
}

#[tonic::async_trait]
impl ConnectionStateManager for ConnectionStateService {
    async fn set_connection_state(&self, request: Request<SetConnectionStateRequest>) -> Result<Response<SetConnectionStateResponse>, Status> {
        todo!()
    }

    async fn get_connection_state(&self, request: Request<GetConnectionStateRequest>) -> Result<Response<GetConnectionStateResponse>, Status> {
        todo!()
    }
}