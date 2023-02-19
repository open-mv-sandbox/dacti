#!/bin/bash
set -e

cargo build

export PATH="$PATH:./target/debug"
export PACK="./packages/dacti-example-web/public/viewer-builtins.dacti-pack"

ptero-pack create --package $PACK
ptero-pack add --package $PACK --input ./data/shader.wgsl --uuid bacc2ba1-8dc7-4d54-a7a4-cdad4d893a1b
