// Rasterise un SVG d'icône en PNG 1024×1024 sans canal alpha (Apple le refuse).
//   npm install playwright-core
//   node ios/icons/render.mjs ios/icons/watch-icon.svg \
//        ios/ReNeuro/ReNeuroWatch/Assets.xcassets/AppIcon.appiconset/icon-1024.png
import { chromium } from 'playwright-core'
import { readFileSync } from 'node:fs'

const [, , source, destination] = process.argv
if (!source || !destination) {
  console.error('usage : node render.mjs <source.svg> <destination.png>')
  process.exit(1)
}

const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH || undefined })
const page = await browser.newPage({ viewport: { width: 1024, height: 1024 }, deviceScaleFactor: 1 })
await page.setContent(
  `<style>html,body{margin:0;padding:0;overflow:hidden}</style>${readFileSync(source, 'utf8')}`
)
// omitBackground: false => la page est opaque, le PNG sort en RGB sans alpha
await page.screenshot({ path: destination, omitBackground: false })
await browser.close()
console.log('écrit :', destination)
