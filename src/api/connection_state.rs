use deadpool_redis_cluster::Pool as RedisClusterConnectionPool;
use tonic::{Request, Response, Status};

use crate::proto::{
    connection_state_manager_server::ConnectionStateManager,
    SetConnectionStateRequest, SetConnectionStateResponse,
    GetConnectionStateRequest, GetConnectionStateResponse,
    ResetConnectionStateRequest, ResetConnectionStateResponse
};
use crate::core::error::Error;

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
        let redis_connection = self.redis.get().await.map_err(|err| Error::from(err))?;
        todo!()
    }

    async fn get_connection_state(&self, request: Request<GetConnectionStateRequest>) -> Result<Response<GetConnectionStateResponse>, Status> {
        todo!()
    }

    async fn reset_connection_state(&self, request: Request<ResetConnectionStateRequest>) -> Result<Response<ResetConnectionStateResponse>, Status> {
        todo!()
    }
}