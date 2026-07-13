import express from 'express'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const dist = path.join(__dirname, 'dist')
const app = express()

app.use(express.static(dist, {
  maxAge: '1y',
  setHeaders: (res, filePath) => {
    // l'index, le manifest et le service worker doivent toujours être revalidés,
    // sinon les mises à jour de la PWA ne seraient jamais récupérées
    if (/\.(html|webmanifest)$/.test(filePath) || /(?:^|\/)(sw|registerSW|workbox-[^/]+)\.js$/.test(filePath)) {
      res.set('Cache-Control', 'no-cache')
    }
  },
}))

// toute autre URL renvoie l'application
app.use((req, res) => {
  res.set('Cache-Control', 'no-cache')
  res.sendFile(path.join(dist, 'index.html'))
})

const port = process.env.PORT || 3000
app.listen(port, () => console.log(`Mes Débuts en écoute sur le port ${port}`))
