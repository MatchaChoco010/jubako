import { defineConfig } from 'vite'

export default defineConfig({
  base: "./",
  clearScreen: false,
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    sourcemap: true,
  },
})
