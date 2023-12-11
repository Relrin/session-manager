use deadpool_redis_cluster::Pool as RedisClusterConnectionPool;
use tonic::{Request, Response, Status};

use crate::proto::{
    session_manager_server::SessionManager,
    SetSessionRequest, SetSessionResponse,
    GetSessionRequest, GetSessionResponse,
    ResetSessionRequest, ResetSessionResponse
};
use crate::core::error::Error;

pub struct SessionManagerService {
    redis: RedisClusterConnectionPool,
}

impl SessionManagerService {
    pub fn new(redis: RedisClusterConnectionPool) -> Self {
        Self { redis }
    }
}


#[tonic::async_trait]
impl SessionManager for SessionManagerService {
     async fn set_session(&self, request: Request<SetSessionRequest>) -> Result<Response<SetSessionResponse>, Status> {
         let redis_connection = self.redis.get().await.map_err(|err| Error::from(err))?;
         todo!()
     }

     async fn get_session(&self, request: Request<GetSessionRequest>) -> Result<Response<GetSessionResponse>, Status> {
         todo!()
     }

     async fn reset_session(&self, request: Request<ResetSessionRequest>) -> Result<Response<ResetSessionResponse>, Status> {
         todo!()
     }
 }