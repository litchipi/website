#!/bin/sh

set -e

clear

CONFIG_FILE=$(realpath `dirname $0`/../config.toml)
ROOT_DIR=$(realpath `dirname $0`/../../../web/ecoweb)

echo "$CONFIG_FILE $ROOT_DIR"

FEATURES_LIST=(dev storage-local minify)
FEATURES=$(IFS=, ; echo "${FEATURES_LIST[*]}")

cd "$ROOT_DIR"

rm -rf ./target/assets
cargo run "$@" --no-default-features --features "${FEATURES}" -- -c $CONFIG_FILE
# cargo run --release -- -c $CONFIG_FILE
