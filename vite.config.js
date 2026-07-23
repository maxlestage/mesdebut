import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
  plugins: [
    react(),
    VitePWA({
      registerType: 'autoUpdate',
      includeAssets: ['apple-touch-icon.png'],
      manifest: {
        name: 'Mes Débuts AVC — Apprendre en s\'amusant',
        short_name: 'Mes Débuts AVC',
        description: 'Quiz pour apprendre les jours, les mois, et le calcul : addition, soustraction, multiplication et division.',
        lang: 'fr',
        start_url: '/',
        display: 'standalone',
        orientation: 'portrait',
        theme_color: '#667eea',
        background_color: '#667eea',
        icons: [
          { src: 'pwa-192.png', sizes: '192x192', type: 'image/png' },
          { src: 'pwa-512.png', sizes: '512x512', type: 'image/png' },
          { src: 'pwa-maskable-512.png', sizes: '512x512', type: 'image/png', purpose: 'maskable' },
        ],
      },
      workbox: {
        // tout le build est mis en cache : l'appli fonctionne hors ligne
        globPatterns: ['**/*.{js,css,html,png,svg,ico}'],
      },
    }),
  ],
})
