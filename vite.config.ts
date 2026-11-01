/// <reference types="vitest/config" />
import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import { viteSingleFile } from 'vite-plugin-singlefile';

export default defineConfig(({ mode }) => ({
  // Relative asset paths, so the build also runs from file://.
  base: './',
  // `--mode saver`: one self-contained HTML file for the macOS screensaver, since module
  // scripts and fonts can't be fetched from file:// inside its web view.
  plugins: mode === 'saver' ? [solid(), viteSingleFile()] : [solid()],
  build: mode === 'saver' ? { outDir: 'saver/build/web', assetsInlineLimit: Infinity } : {},
  test: {
    environment: 'node',
  },
}));
