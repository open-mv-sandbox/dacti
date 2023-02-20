#!/bin/bash
set -e

cargo doc --workspace --exclude ptero-pack-cli --open
