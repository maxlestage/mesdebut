// Fabrique toutes les icônes de l'application à partir de icons/icon.svg.
//
//   npm install playwright-core
//   node icons/generate.mjs
//
// Le vectoriel est rasterisé par Chromium : net à toutes les tailles, là où un
// émoji agrandi est flou (Noto Color Emoji est une police bitmap plafonnée à
// 136 px). Les PNG sortent en RGB sans canal alpha — Apple refuse les icônes
// d'application transparentes.
import { chromium } from 'playwright-core'
import { copyFileSync, readFileSync } from 'node:fs'

const SVG = new URL('./icon.svg', import.meta.url)

// L'échelle dépend de la sévérité du masque de chaque plateforme.
const CIBLES = [
  { fichier: 'ios/ReNeuro/ReNeuro/Assets.xcassets/AppIcon.appiconset/icon-1024.png',
    taille: 1024, echelle: 1.16, note: 'iPhone — masque en squircle' },
  { fichier: 'ios/ReNeuro/ReNeuroWatch/Assets.xcassets/AppIcon.appiconset/icon-1024.png',
    taille: 1024, echelle: 1, note: 'Apple Watch — masque en cercle' },
  { fichier: 'public/apple-touch-icon.png',
    taille: 180, echelle: 1.16, note: 'écran d\'accueil iOS' },
  { fichier: 'public/pwa-192.png',
    taille: 192, echelle: 1.16, note: 'PWA' },
  { fichier: 'public/pwa-512.png',
    taille: 512, echelle: 1.16, note: 'PWA' },
  { fichier: 'public/pwa-maskable-512.png',
    taille: 512, echelle: 1, note: 'PWA maskable — zone sûre = cercle central' },
]

const svg = readFileSync(SVG, 'utf8')
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH || undefined })

for (const { fichier, taille, echelle, note } of CIBLES) {
  const page = await browser.newPage({
    viewport: { width: taille, height: taille }, deviceScaleFactor: 1,
  })
  await page.setContent(`<style>
      html,body{margin:0;padding:0;overflow:hidden}
      svg{display:block;width:${taille}px;height:${taille}px}
      #toque{transform-origin:512px 512px;transform:scale(${echelle})}
    </style>${svg}`)
  // omitBackground: false => la page est opaque, le PNG sort sans canal alpha
  await page.screenshot({ path: fichier, omitBackground: false })
  await page.close()
  console.log(`${taille.toString().padStart(4)}px  ×${echelle}  ${fichier}  (${note})`)
}

// Le favicon reste vectoriel : le navigateur le met à l'échelle lui-même.
copyFileSync(SVG, 'public/favicon.svg')
console.log('   svg        public/favicon.svg')

await browser.close()
