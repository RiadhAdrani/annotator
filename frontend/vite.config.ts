import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import unocss from '@unocss/vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), unocss({})],
  define: {
    'import.meta.env.API_URL': JSON.stringify(process.env.API_URL),
  },
});
