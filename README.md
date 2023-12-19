# session-manager
[Experimental] Session manager for games

### Running local dev environment
- Start up the entire dev stack by the docker-compose command:
```
docker-compose -f .\docker-compose.dev.yaml up -d
```

- Initialize a redis cluster (when haven't done it before):
    -  Connect to the redis_1 container
        ```
        docker-compose -f .\docker-compose.dev.yaml exec redis_1 bash
        ```
    - Run the redis-cli command to initialize a cluster
        ```
        echo "yes" | redis-cli --cluster create 173.18.0.2:6379 173.18.0.3:6379 173.18.0.4:6379 173.18.0.5:6379 173.18.0.6:6379 173.18.0.7:6379 --cluster-replicas 1
        ```

- Connect to the app container
```
docker-compose -f .\docker-compose.dev.yaml exec app bash
```

- Run the application by the following command
```
cargo run -- --host=0.0.0.0 --redis-cluster-urls=redis_1:6379,redis_2:6379,redis_3:6379,redis_4:6379,redis_5:6379,redis_6:6379
```