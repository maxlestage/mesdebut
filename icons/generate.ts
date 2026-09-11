// Fabrique toutes les icônes de l'application à partir de icons/icon.svg.
//
//   cd icons && npm install && npm run generate
//
// Le vectoriel est rasterisé par Chromium : net à toutes les tailles, là où un
// émoji agrandi est flou (Noto Color Emoji est une police bitmap plafonnée à
// 136 px). Les PNG sortent en RGB sans canal alpha — Apple refuse les icônes
// d'application transparentes.
import { chromium } from 'playwright-core'
import { copyFileSync, readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'

const SVG = new URL('./icon.svg', import.meta.url)
// Les sorties sont repérées depuis la racine du dépôt, pas depuis le dossier
// courant : le script se lance donc d'où l'on veut.
const depuisRacine = (chemin: string) => fileURLToPath(new URL('../' + chemin, import.meta.url))

// L'échelle dépend de la sévérité du masque de chaque plateforme.
type Cible = {
  readonly fichier: string
  readonly taille: number
  /** Agrandissement du dessin, selon la sévérité du masque de la plateforme. */
  readonly echelle: number
  readonly note: string
}

const CIBLES: readonly Cible[] = [
  { fichier: 'ios/ReNeuro/ReNeuro/Assets.xcassets/AppIcon.appiconset/icon-1024.png',
    taille: 1024, echelle: 1.16, note: 'iPhone — masque en squircle' },
  { fichier: 'ios/ReNeuro/ReNeuroWatch/Assets.xcassets/AppIcon.appiconset/icon-1024.png',
    taille: 1024, echelle: 1, note: 'Apple Watch — masque en cercle' },
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
  await page.screenshot({ path: depuisRacine(fichier), omitBackground: false })
  await page.close()
  console.log(`${taille.toString().padStart(4)}px  ×${echelle}  ${fichier}  (${note})`)
}

// Le site utilise directement le vectoriel : le navigateur le met à l'échelle.
copyFileSync(SVG, depuisRacine('site/public/icon.svg'))
console.log('   svg        site/public/icon.svg')

await browser.close()
