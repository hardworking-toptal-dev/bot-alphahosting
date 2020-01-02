#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )"

docker run --name build_certbot-alfahosting \
    -v "$WORKDIR:/root/src" \
    --rm -i -t -w=/root/src rust:1-stretch \
    cargo build --release --target=x86_64-unknown-linux-gnu

docker build -t certbot-alfahosting "$WORKDIR"
