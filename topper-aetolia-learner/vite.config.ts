import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { resolve } from 'node:path';

export default defineConfig({
  plugins: [svelte()],
  server: {
    fs: {
      allow: [resolve('.'), resolve('src-wasm/pkg')]
    }
  }
});
