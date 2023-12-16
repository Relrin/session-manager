use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    host: String,
    port: u32,
    metadata: HashMap<String, String>,
}

impl Session {
    pub fn new(host: &String, port: u32, metadata: &HashMap<String, String>) -> Self {
        return Session {
            host: host.clone(),
            port,
            metadata: metadata.clone()
        }
    }
}
