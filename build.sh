#!/bin/sh

TARGET=$(uname -m)-unknown-linux-musl
cargo build --release --target=${TARGET}
mkdir -p ./dist/
cp -v ./target/${TARGET}/debug/parrot-node ./dist/
