import express from 'express'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const site = path.join(__dirname, 'site', 'dist')   // site de présentation → /
const appli = path.join(__dirname, 'dist')          // application (PWA)     → /app
const app = express()

// L'index, le manifeste et le service worker doivent toujours être revalidés,
// sinon les mises à jour de la PWA ne seraient jamais récupérées.
const entetes = (res, filePath) => {
  if (/\.(html|webmanifest)$/.test(filePath)
      || /(?:^|\/)(sw|registerSW|workbox-[^/]+)\.js$/.test(filePath)) {
    res.set('Cache-Control', 'no-cache')
  }
}

// L'application a déménagé de / vers /app : on ne casse pas les liens existants.
app.get(['/presentation', '/presentation/*'], (_req, res) => res.redirect(301, '/'))

// ── L'application, sous /app ────────────────────────────────────────────────
app.use('/app', express.static(appli, { maxAge: '1y', setHeaders: entetes }))
app.get(['/app', '/app/*'], (_req, res) => {
  res.set('Cache-Control', 'no-cache')
  res.sendFile(path.join(appli, 'index.html'))
})

// ── Le site de présentation, à la racine ────────────────────────────────────
// Sert aussi /sw.js, le service worker d'extinction qui retire celui que
// l'application avait laissé à la racine avant son déménagement.
app.use(express.static(site, { maxAge: '1y', setHeaders: entetes }))

// Toute autre URL renvoie le site.
app.use((_req, res) => {
  res.set('Cache-Control', 'no-cache')
  res.sendFile(path.join(site, 'index.html'))
})

const port = process.env.PORT || 3000
app.listen(port, () => console.log(`ReNeuro en écoute sur le port ${port}`))
