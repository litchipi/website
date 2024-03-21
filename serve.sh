#!/bin/sh

CONFIG_FILE=$(realpath ./config.toml)
ROOT_DIR=$(realpath ../../web/ecoweb)

FEATURES_LIST=(dev)
# FEATURES_LIST=(rss humans-txt webring local_storage add-endpoint save-data githook minify)
FEATURES=$(IFS=, ; echo "${FEATURES_LIST[*]}")

cd "$ROOT_DIR"

clear
rm -rf ./target/assets
cargo run --features "${FEATURES}" -- -c $CONFIG_FILE
