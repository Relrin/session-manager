use redis::Cmd;
use deadpool_redis::cluster::Pool as RedisClusterConnectionPool;
use tonic::{Request, Response, Status};

use crate::proto::{
    session_manager_server::SessionManager,
    SetSessionRequest, SetSessionResponse,
    GetSessionRequest, GetSessionResponse,
    ResetSessionRequest, ResetSessionResponse
};
use crate::core::error::Error;
use crate::core::models::Session;

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
        let request_body = request.into_inner();
        let session = Session::new(&request_body.host, request_body.port, &request_body.metadata);
        let session_json = serde_json::to_string(&session).map_err(|err| Error::from(err))?;

        let entries = request_body.player_ids
            .iter()
            .map(|player_id| (player_id, &session_json))
            .collect::<Vec<_>>();

        let mut redis_connection = self.redis.get().await.map_err(|err| Error::from(err))?;
        Cmd::mset(&entries).query_async::<_, ()>(&mut redis_connection).await.map_err(|err| Error::from(err))?;

        Ok(Response::new(SetSessionResponse{}))
    }

    async fn get_session(&self, request: Request<GetSessionRequest>) -> Result<Response<GetSessionResponse>, Status> {
        todo!()
    }

    async fn reset_session(&self, request: Request<ResetSessionRequest>) -> Result<Response<ResetSessionResponse>, Status> {
        todo!()
    }
 }