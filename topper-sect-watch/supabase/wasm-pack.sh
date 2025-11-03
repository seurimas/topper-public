#!/bin/sh
cd "$(dirname "$0")"/functions/share-log/src-wasm ; wasm-pack build --release --target deno ; cd -