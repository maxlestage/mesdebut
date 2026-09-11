import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// Site statique : pas de serveur, il se dépose tel quel sur n'importe quel hébergeur.
export default defineConfig({
  plugins: [react()],
  base: './',
})
