use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::proto::GetSessionResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    host: String,
    port: u32,
    metadata: HashMap<String, String>,
}

impl Session {
    pub fn new(host: &String, port: u32, metadata: &HashMap<String, String>) -> Self {
        Self {
            host: host.clone(),
            port,
            metadata: metadata.clone(),
        }
    }
}

impl From<Session> for GetSessionResponse {
    fn from(session: Session) -> Self {
        GetSessionResponse {
            host: session.host,
            port: session.port,
            metadata: session.metadata,
        }
    }
}