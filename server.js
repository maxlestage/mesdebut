// Serveur de production, sans dépendance : seuls les modules de Node.
//
// Il sert deux builds distincts — le site de présentation à la racine, et
// l'application (PWA) sous /app — avec les en-têtes de cache qui conviennent à
// chacun.

import { createReadStream } from 'node:fs'
import { stat } from 'node:fs/promises'
import { createServer } from 'node:http'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const SITE = path.join(__dirname, 'site', 'dist') // site de présentation → /
const APPLI = path.join(__dirname, 'dist')        // application (PWA)     → /app

const TYPES = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.webmanifest': 'application/manifest+json; charset=utf-8',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.webp': 'image/webp',
  '.ico': 'image/x-icon',
  '.woff2': 'font/woff2',
  '.txt': 'text/plain; charset=utf-8',
}

const AN = 60 * 60 * 24 * 365

/**
 * L'index, le manifeste et le service worker doivent toujours être revalidés,
 * sinon les mises à jour de la PWA ne seraient jamais récupérées. Le reste porte
 * une empreinte dans son nom : il peut être gardé un an.
 */
function cacheDe(fichier) {
  const revalider = /\.(html|webmanifest)$/.test(fichier)
    || /(?:^|[\\/])(sw|registerSW|workbox-[^\\/]+)\.js$/.test(fichier)
  return revalider ? 'no-cache' : `public, max-age=${AN}`
}

/**
 * Le chemin demandé, ramené à un fichier réel sous `racine`.
 * Renvoie null si le chemin s'en échappe ou ne désigne pas un fichier : c'est
 * la protection contre les remontées d'arborescence (« ../ », chemins encodés),
 * dont on ne bénéficie plus automatiquement sans intergiciel.
 */
async function fichierSous(racine, cheminUrl) {
  let relatif
  try {
    relatif = decodeURIComponent(cheminUrl)
  } catch {
    return null // séquence d'échappement invalide
  }
  if (relatif.includes('\0')) return null

  const cible = path.resolve(racine, '.' + path.posix.normalize(relatif))
  if (cible !== racine && !cible.startsWith(racine + path.sep)) return null

  try {
    const infos = await stat(cible)
    return infos.isFile() ? { chemin: cible, infos } : null
  } catch {
    return null
  }
}

function envoie(req, res, chemin, infos, code = 200) {
  const etag = `W/"${infos.size.toString(16)}-${infos.mtimeMs.toString(16)}"`
  const entetes = {
    'Content-Type': TYPES[path.extname(chemin).toLowerCase()] ?? 'application/octet-stream',
    'Cache-Control': cacheDe(chemin),
    'Content-Length': infos.size,
    ETag: etag,
    'Last-Modified': infos.mtime.toUTCString(),
  }
  // revalidation bon marché : le navigateur garde sa copie
  if (req.headers['if-none-match'] === etag) {
    res.writeHead(304, { ETag: etag, 'Cache-Control': entetes['Cache-Control'] })
    return res.end()
  }
  res.writeHead(code, entetes)
  if (req.method === 'HEAD') return res.end()
  createReadStream(chemin).pipe(res)
}

async function envoieIndex(req, res, racine) {
  const index = await fichierSous(racine, '/index.html')
  if (!index) {
    res.writeHead(500, { 'Content-Type': 'text/plain; charset=utf-8' })
    return res.end("Build absent : lancer « npm run build » et « npm run build:site ».")
  }
  res.setHeader('Cache-Control', 'no-cache')
  envoie(req, res, index.chemin, index.infos)
}

const serveur = createServer(async (req, res) => {
  if (req.method !== 'GET' && req.method !== 'HEAD') {
    res.writeHead(405, { Allow: 'GET, HEAD', 'Content-Type': 'text/plain; charset=utf-8' })
    return res.end('Méthode non autorisée')
  }

  const url = new URL(req.url, 'http://localhost')
  const chemin = url.pathname

  // L'application a déménagé de / vers /app : on ne casse pas les liens existants.
  if (chemin === '/presentation' || chemin.startsWith('/presentation/')) {
    res.writeHead(301, { Location: '/' })
    return res.end()
  }

  // ── L'application, sous /app ──────────────────────────────────────────────
  if (chemin === '/app') {
    res.writeHead(301, { Location: '/app/' })
    return res.end()
  }
  if (chemin.startsWith('/app/')) {
    const trouve = await fichierSous(APPLI, chemin.slice('/app'.length))
    if (trouve) return envoie(req, res, trouve.chemin, trouve.infos)
    return envoieIndex(req, res, APPLI) // application d'une seule page
  }

  // ── Le site de présentation, à la racine ──────────────────────────────────
  // Sert aussi /sw.js, le service worker d'extinction qui retire celui que
  // l'application avait laissé à la racine avant son déménagement.
  const trouve = await fichierSous(SITE, chemin)
  if (trouve) return envoie(req, res, trouve.chemin, trouve.infos)
  return envoieIndex(req, res, SITE)
})

serveur.on('clientError', (_err, socket) => {
  if (socket.writable) socket.end('HTTP/1.1 400 Bad Request\r\n\r\n')
})

const port = process.env.PORT || 3000
serveur.listen(port, () => console.log(`ReNeuro en écoute sur le port ${port}`))
