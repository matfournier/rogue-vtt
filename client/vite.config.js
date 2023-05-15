import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  base: './',
  esbuild: {
    loader: 'tsx',
  },
  server: {
    port: 3000,
  },
});