// @ts-check
import { defineConfig } from 'astro/config';

import react from '@astrojs/react';
import tailwindcss from '@tailwindcss/vite';
import mdx from '@astrojs/mdx';

import vue from '@astrojs/vue';
import node from '@astrojs/node';

// https://astro.build/config
export default defineConfig({
  adapter: node({
    mode: 'standalone',
  }),
  integrations: [react(), mdx(), vue()],

  server: {
    host: '0.0.0.0', // listen on all interfaces
    port: 7004,
  },

  prefetch: true,
  output: "server",

  vite: {
    plugins: [tailwindcss()]
  }
});