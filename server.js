import express from 'express'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const dist = path.join(__dirname, 'dist')
const site = path.join(__dirname, 'site', 'dist')
const app = express()

// Le site de présentation, build séparé, servi sous /presentation. Monté avant
// l'application pour que l'attrape-tout de fin ne le recouvre pas.
app.use('/presentation', express.static(site, {
  maxAge: '1y',
  setHeaders: (res, filePath) => {
    if (/\.html$/.test(filePath)) res.set('Cache-Control', 'no-cache')
  },
}))
app.get(['/presentation', '/presentation/*'], (req, res) => {
  res.set('Cache-Control', 'no-cache')
  res.sendFile(path.join(site, 'index.html'))
})

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
app.listen(port, () => console.log(`ReNeuro en écoute sur le port ${port}`))
