use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "session-manager",
    version = "0.1.0",
    about = "Session manager microservice"
)]
pub struct CliOptions {
    #[structopt(
        short = "h",
        long = "host",
        help = "The used IP for a server",
        default_value = "127.0.0.1"
    )]
    pub host: String,
    #[structopt(
        short = "p",
        long = "port",
        help = "The listened port",
        default_value = "8000"
    )]
    pub port: u16,

    #[structopt(
        long = "redis-cluster-urls",
        help = "Redis cluster URLs",
        default_value = "127.0.0.1:6379",
        env = "REDIS_CLUSTER_URLS"
    )]
    pub redis_nodes: String,

    #[structopt(
        long = "redis-username",
        help = "Redis username",
        default_value = "redis",
        env = "REDIS_USERNAME"
    )]
    pub redis_username: String,

    #[structopt(
        long = "redis-password",
        help = "Redis user's password",
        default_value = "password",
        env = "REDIS_PASSWORD"
    )]
    pub redis_password: String,
}
