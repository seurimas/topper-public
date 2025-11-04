#!/bin/sh
cd "$(dirname "$0")"/functions/share-log/share-log-wasm ; wasm-pack build --release --target deno ; cd -