#!/bin/bash
set -e

cargo build

export PATH="$PATH:./target/debug"
export PACK="./packages/dacti-example-web/public/viewer-builtins.dacti-pack"

ptero-pack create --package $PACK
ptero-pack add --package $PACK --file ./data/shader.wgsl
