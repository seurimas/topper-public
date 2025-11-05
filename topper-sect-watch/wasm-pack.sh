#!/bin/sh
cd "$(dirname "$0")"/supabase/functions/share-log/share-log-wasm ; wasm-pack build --release --target deno ; cd -
cd "$(dirname "$0")"/src-wasm ; wasm-pack build --release --target web ; rm pkg/.gitignore ; cd -