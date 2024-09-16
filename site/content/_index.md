+++
title = ''
date = 2024-09-16T21:25:22+08:00
draft = true
+++
Welcome to the first web 3.0 OCI/docker container registry.

## FAQ

1. Why Rust? Why not python or ruby? It would have been faster
The most popular and advanced CloudNative container registry is written in Rust and while this is a standalone project, it makes sense to eventually turn this into an extension for harbor.
2. Why do we use a wrapper for the walrus CLI?
There is currently proper API or sdk for walrus. I started looking at the sui sdk, but trying to integrate that does not make sense with the lack of documentation.
3. Why SQLite instead of Move smart contracts?
Time and money. In the spirit of hackathons this was coded in a weekend.

# Getting started

    cp .env.example .env
    make start-bg

    cd example
    docker build . -t localhost:8090/nginx2
    docker push localhost:8090/nginx2

## Development


    cargo install cargo-watch
    cargo install diesel_cli --no-default-features --features sqlite
    diesel migration run
    cargo watch -i '*.sqlite*' -x run 

If you're developing locally you need to add the following to `/etc/docker/daemon.json`

      { "insecure-registries": [ "localhost:8090" ] }

Inside the examples directory you can then run:

        docker build . -t localhost:8090/nginx2
        docker push localhost:8090/nginx2

## Add migrations
    cargo install diesel_cli --no-default-features --features sqlite

## Security nightmares
Since there is no auth, you can override other peoples images or create funky stuff if you write to the same 

## TODO
- [X] Make sure the same layer is only uploaded once
- [X] Check checksum of blobs, on uploads and return already uploaded when it exists
- [ ] Use Move smart contract
