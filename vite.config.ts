import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000,
    open: true,
  },
  build: {
    outDir: 'build',
    sourcemap: false,
    minify: 'esbuild',
    // SEO-optimized build settings
    cssCodeSplit: true, // Separate CSS files
    reportCompressedSize: true, // Log gzipped bundle size
    rollupOptions: {
      output: {
        // Optimized chunk splitting for better caching
        manualChunks: {
          vendor: ['react', 'react-dom', 'react-router-dom', 'react-helmet-async'],
          animations: ['framer-motion'],
          icons: ['lucide-react'],
          query: ['@tanstack/react-query'],
        },
        // Optimize chunk names for SEO
        entryFileNames: 'js/[name]-[hash].js',
        chunkFileNames: 'js/[name]-[hash].js',
        assetFileNames: (assetInfo) => {
          const fileName = assetInfo.names?.[0] || '';
          const info = fileName.split('.');
          const ext = info[info.length - 1];
          if (/png|jpe?g|gif|tiff|bmp|ico/i.test(ext)) {
            return `images/[name]-[hash][extname]`;
          } else if (/woff|woff2|ttf|otf|eot/i.test(ext)) {
            return `fonts/[name]-[hash][extname]`;
          } else if (ext === 'css') {
            return `css/[name]-[hash][extname]`;
          }
          return `[name]-[hash][extname]`;
        },
      },
    },
    // Performance optimization
    terserOptions: {
      compress: {
        drop_console: true, // Remove console logs in production
      },
    },
  },
});
