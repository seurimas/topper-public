# Topper Aetolia Learner

Svelte + TypeScript frontend with a custom Rust/WASM backend for fast combat-plan simulation experiments.

## Requirements

- Node.js 20+
- Rust toolchain (`rustup`)
- `wasm-pack` (`cargo install wasm-pack`)

## Local Development

```bash
npm install
npm run dev
```

The `dev` script builds the WASM package first, then runs Vite.

## Release Build

```bash
npm run build
```

This runs an optimized WASM build and bundles the frontend.

## Project Layout

- `src-wasm/`: Rust crate compiled to WebAssembly
- `src/`: Svelte + TypeScript UI and orchestration
- `docs/`: planning and design docs
