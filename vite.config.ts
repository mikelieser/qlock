/// <reference types="vitest/config" />
import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';

export default defineConfig({
  // Relative asset paths, so the build also runs from file:// (screensaver bundle).
  base: './',
  plugins: [solid()],
  test: {
    environment: 'node',
  },
});
