use deadpool_redis::cluster::Pool as RedisClusterConnectionPool;
use redis::{RedisResult};
use tonic::{Request, Response, Status};

use crate::core::error::Error;
use crate::core::models::Session;
use crate::proto::{
    session_manager_server::SessionManager, GetSessionRequest, GetSessionResponse,
    ResetSessionRequest, ResetSessionResponse, SetSessionRequest, SetSessionResponse,
};

const SESSION_TTL: i64 = 1800;

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
    async fn set_session(
        &self,
        request: Request<SetSessionRequest>,
    ) -> Result<Response<SetSessionResponse>, Status> {
        let request_body = request.into_inner();
        let session = Session::new(
            &request_body.host,
            request_body.port,
            &request_body.metadata,
        );
        let session_json = serde_json::to_string(&session).map_err(|err| Error::from(err))?;

        let entries_mset = request_body
            .player_ids
            .iter()
            .map(|player_id| (player_id, &session_json))
            .collect::<Vec<_>>();

        let mut pipeline = redis::pipe();
        pipeline.mset(&entries_mset).ignore();
        for player_id in request_body.player_ids {
            pipeline.expire(player_id, SESSION_TTL).ignore();
        }

        let mut redis_connection = self.redis.get().await.map_err(|err| Error::from(err))?;
        pipeline
            .query_async::<_, ()>(&mut redis_connection)
            .await
            .map_err(|err| Error::from(err))?;

        Ok(Response::new(SetSessionResponse {}))
    }

    async fn get_session(
        &self,
        request: Request<GetSessionRequest>,
    ) -> Result<Response<GetSessionResponse>, Status> {
        let player_id = request.into_inner().player_id;

        let mut redis_connection = self.redis.get().await.map_err(|err| Error::from(err))?;

        let redis_result: RedisResult<Vec<String>> = redis::pipe()
            .cmd("READONLY")
            .ignore()
            .cmd("GET")
            .arg(&player_id)
            .query_async(&mut redis_connection)
            .await;

        if redis_result.is_err() {
            return Err(Status::not_found("Session not found"))
        }

        let serialized_session = redis_result.unwrap();
        let session: Session = serde_json::from_str(&serialized_session[0]).map_err(|err| Error::from(err))?;

        Ok(Response::new(GetSessionResponse::from(session)))
    }

    async fn reset_session(
        &self,
        request: Request<ResetSessionRequest>,
    ) -> Result<Response<ResetSessionResponse>, Status> {
        let request_body = request.into_inner();

        let mut pipeline = redis::pipe();
        for player_id in request_body.player_ids {
            pipeline.del(player_id).ignore();
        }

        let mut redis_connection = self.redis.get().await.map_err(|err| Error::from(err))?;
        pipeline
            .query_async::<_, ()>(&mut redis_connection)
            .await
            .map_err(|err| Error::from(err))?;

        Ok(Response::new(ResetSessionResponse {}))
    }
}
