# Walrus Container Registry

Welcome to the frist web 3.0 OCI/docker container registry.

## FAQ

1. Why Rust? Why not python or ruby? It would have been faster
The most popular and advanced CloudNative container registry is written in Rust and while this is a standalone project, it makes sense to eventually turn this into an extension for harbor.
2. Why do we use a wrapper for the walrus CLI?
There is currently proper API or sdk for walrus. I started looking at the sui sdk, but trying to integrate that does not make sense with the lack of documentation.
3. Why SQLite instead of Move smart contracts?
Time and money. In the spirit of hackathons this was coded in a weekend.

## Getting started

    cp .env.example .env
    cargo run

    cd example
    docker build . -t localhost:8090/nginx2
    docker push localhost:8090/nginx2

## Development
    cargo install cargo-watch
    cargo watch -x run

## Add migrations
    cargo install diesel_cli --no-default-features --features sqlite
## TODO
- [ ] Make sure the same layer is only uploaded once
- [ ] Check checksum of blobs, on uploads and return already uploaded when it exists
