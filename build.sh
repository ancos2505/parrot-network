#!/bin/sh
TARGET=x86_64-unknown-linux-musl
cargo build --target=${TARGET}
mkdir -p ./dist/
cp -v ./target/${TARGET}/debug/parrot-node ./dist/
