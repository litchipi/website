#!/bin/sh

ROOT_DIR=`realpath "$(dirname $0)/.."`

codespell -w --interactive 3 -I $ROOT_DIR/.ignored_words $ROOT_DIR/posts/
