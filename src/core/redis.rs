use deadpool_redis::cluster::{Config, Pool, Runtime};

use crate::cli::CliOptions;

pub async fn create_redis_conection_pool(opts: &CliOptions) -> Pool {
    let template = match opts.redis_username.is_empty() && opts.redis_password.is_empty() {
        true => format!("redis://{0}:{1}@", opts.redis_username, opts.redis_password),
        false => "redis://".to_string(),
    };

    let redis_nodes = opts
        .redis_nodes
        .split(',')
        .filter(|address| !address.is_empty())
        .map(|address| template.clone() + address)
        .collect::<Vec<String>>();

    let cfg = Config::from_urls(redis_nodes);
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}
