#!/bin/sh

ip addr
ls -lh /app/

/app/parrot-node --webui-ip=0.0.0.0 --verbose --config-file=/app/parrot-node.toml