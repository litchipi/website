#!/bin/sh

ROOT_DIR=`realpath "$(dirname $0)/.."`

nix-shell -p codespell --command "codespell -w --interactive 3 -I $ROOT_DIR/.ignored_words $ROOT_DIR/data/blog/ && echo 'OK'"
