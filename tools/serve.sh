#!/bin/sh

set -e

clear

CONFIG_FILE=$(realpath `dirname $0`/../config.toml)
ROOT_DIR=$(realpath `dirname $0`/../../../web/ecoweb)

echo "$CONFIG_FILE $ROOT_DIR"

FEATURES_LIST=(dev minify)
# FEATURES_LIST=(rss humans-txt webring local_storage add-endpoint save-data githook minify)
FEATURES=$(IFS=, ; echo "${FEATURES_LIST[*]}")

cd "$ROOT_DIR"

rm -rf ./target/assets
cargo run "$@" --features "${FEATURES}" -- -c $CONFIG_FILE
# cargo run --release -- -c $CONFIG_FILE
