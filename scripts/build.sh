#!/bin/bash

# Make sure we always run from the root
SCRIPTS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPTS_DIR/.."

# Build canister
export RUSTFLAGS="--remap-path-prefix $(readlink -f $(dirname ${0}))=/build --remap-path-prefix ${CARGO_HOME}=/cargo"
cargo rustc -p icp_hello_world_rust_backend --crate-type=cdylib --locked --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/icp_hello_world_rust_backend.wasm icp_hello_world_rust_backend.wasm
