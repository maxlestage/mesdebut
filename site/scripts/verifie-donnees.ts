// Le site réécrit la liste des thèmes pour rester un projet autonome et typé.
// Ce contrôle la compare à la source de vérité, src/questions.js de l'app, pour
// qu'un thème ajouté ou renommé ne passe pas inaperçu ici.
import { CATEGORIES as SITE } from '../src/donnees.ts'
import { CATEGORIES as APP, levelsFor } from '../../src/questions.ts'

const erreurs = []
const clesApp = Object.keys(APP)
const clesSite = SITE.map(c => c.key)

if (clesApp.join(',') !== clesSite.join(',')) {
  erreurs.push(`Thèmes différents (ordre compris) :\n    app  : ${clesApp.join(' ')}\n    site : ${clesSite.join(' ')}`)
}

for (const c of SITE) {
  const ref = APP[c.key]
  if (!ref) { erreurs.push(`« ${c.key} » n'existe pas dans l'app`); continue }
  if (ref.emoji !== c.emoji) erreurs.push(`${c.key}.emoji : app ${ref.emoji} ≠ site ${c.emoji}`)
  if (ref.label !== c.label) erreurs.push(`${c.key}.label : app « ${ref.label} » ≠ site « ${c.label} »`)
  if (ref.title !== c.title) erreurs.push(`${c.key}.title : app « ${ref.title} » ≠ site « ${c.title} »`)

  const niveauxApp = ref.hasLevels ? levelsFor(c.key).map(n => n.label) : []
  const niveauxSite = c.niveaux ?? []
  if (niveauxApp.join('|') !== niveauxSite.join('|')) {
    erreurs.push(`${c.key}.niveaux : app [${niveauxApp}] ≠ site [${niveauxSite}]`)
  }
}

console.log(`${clesSite.length} thèmes comparés au moteur de l'application`)
if (erreurs.length) {
  console.error(`\n❌ ${erreurs.length} divergence(s) :`)
  for (const e of erreurs) console.error('  -', e)
  process.exit(1)
}
console.log('✅ le site est à jour avec l\'application')
