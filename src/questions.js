export const JOURS = ['lundi', 'mardi', 'mercredi', 'jeudi', 'vendredi', 'samedi', 'dimanche']
export const MOIS = ['janvier', 'février', 'mars', 'avril', 'mai', 'juin',
  'juillet', 'août', 'septembre', 'octobre', 'novembre', 'décembre']
export const ALPHABET = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')
export const SAISONS = ['printemps', 'été', 'automne', 'hiver']
export const NB_QUESTIONS = 10

// article devant chaque saison ("après l'été", "après le printemps")
const SAISON_ARTICLE = { printemps: 'le printemps', 'été': "l'été", automne: "l'automne", hiver: "l'hiver" }

// mois clairement dans une seule saison (on évite les mois de changement de saison)
const MOIS_SAISON = {
  janvier: 'hiver', février: 'hiver',
  avril: 'printemps', mai: 'printemps',
  juillet: 'été', août: 'été',
  octobre: 'automne', novembre: 'automne',
}

// hasLearn : propose un écran de révision ; hasLevels : propose un choix de niveau.
// Les deux peuvent coexister (ex. « Les nombres » : réviser puis choisir un niveau).
export const CATEGORIES = {
  // Mode entrelacé (interleaving) : mélange les thèmes pour renforcer la mémorisation.
  // Ni révision ni niveau : on va droit au quiz.
  melange: { emoji: '🧠', label: 'Mélange', title: 'Quiz mélangé' },
  jours: { emoji: '📅', label: 'Les jours', title: 'Les jours de la semaine', hasLearn: true },
  mois: { emoji: '🗓️', label: 'Les mois', title: "Les mois de l'année", hasLearn: true },
  saisons: { emoji: '🍂', label: 'Les saisons', title: 'Les saisons', hasLearn: true },
  alphabet: { emoji: '🔤', label: "L'alphabet", title: "L'alphabet", hasLearn: true },
  couleurs: { emoji: '🎨', label: 'Les couleurs', title: 'Les couleurs', hasLearn: true },
  formes: { emoji: '📐', label: 'Les formes', title: 'Les formes géométriques', hasLearn: true },
  chiffres: { emoji: '🧮', label: 'Les chiffres', title: 'Les chiffres de 0 à 9', hasLearn: true },
  cinquante: { emoji: '🔟', label: "Jusqu'à 50", title: "Les nombres jusqu'à 50", hasLearn: true },
  nombres: { emoji: '🔢', label: 'Les nombres', title: 'Les nombres en lettres', hasLearn: true, hasLevels: true },
  addition: { emoji: '➕', label: 'Addition', title: 'Addition', hasLevels: true },
  soustraction: { emoji: '➖', label: 'Soustraction', title: 'Soustraction', hasLevels: true },
  multiplication: { emoji: '✖️', label: 'Multiplication', title: 'Multiplication', hasLevels: true },
  division: { emoji: '➗', label: 'Division', title: 'Division', hasLevels: true },
}

export const LEVELS = [
  { id: 1, emoji: '🌱', label: 'Facile' },
  { id: 2, emoji: '🌿', label: 'Moyen' },
  { id: 3, emoji: '🌳', label: 'Difficile' },
]

// contenu des écrans de révision
// couleurs : nom + code pour la pastille
export const COULEURS = [
  { name: 'rouge', hex: '#e53935' },
  { name: 'bleu', hex: '#1e88e5' },
  { name: 'jaune', hex: '#fdd835' },
  { name: 'vert', hex: '#43a047' },
  { name: 'orange', hex: '#fb8c00' },
  { name: 'violet', hex: '#8e24aa' },
  { name: 'rose', hex: '#f06292' },
  { name: 'marron', hex: '#795548' },
  { name: 'gris', hex: '#9e9e9e' },
  { name: 'noir', hex: '#212121' },
  { name: 'blanc', hex: '#ffffff' },
]

// mélanges de peinture classiques
const MELANGES = [
  { a: 'bleu', b: 'jaune', donne: 'vert' },
  { a: 'rouge', b: 'jaune', donne: 'orange' },
  { a: 'rouge', b: 'bleu', donne: 'violet' },
  { a: 'rouge', b: 'blanc', donne: 'rose' },
  { a: 'noir', b: 'blanc', donne: 'gris' },
]

// objets choisis pour que l'accord du nom de couleur reste correct
const OBJETS_COULEUR = [
  { objet: 'un citron', couleur: 'jaune' },
  { objet: 'une banane', couleur: 'jaune' },
  { objet: 'le soleil', couleur: 'jaune' },
  { objet: 'le ciel', couleur: 'bleu' },
  { objet: 'un sapin', couleur: 'vert' },
  { objet: 'une tomate', couleur: 'rouge' },
  { objet: 'une fraise', couleur: 'rouge' },
  { objet: 'une carotte', couleur: 'orange' },
  { objet: 'un cochon', couleur: 'rose' },
  { objet: 'le chocolat', couleur: 'marron' },
  { objet: 'un éléphant', couleur: 'gris' },
  { objet: 'un corbeau', couleur: 'noir' },
  { objet: 'le lait', couleur: 'blanc' },
]

// nombres en toutes lettres (les 10 premiers servent aussi aux chiffres)
const UNITES = ['zéro', 'un', 'deux', 'trois', 'quatre', 'cinq', 'six', 'sept', 'huit', 'neuf',
  'dix', 'onze', 'douze', 'treize', 'quatorze', 'quinze', 'seize', 'dix-sept', 'dix-huit', 'dix-neuf']
const DIZAINES = { 20: 'vingt', 30: 'trente', 40: 'quarante', 50: 'cinquante', 60: 'soixante', 80: 'quatre-vingt' }

// formes géométriques : nom + petite description pour la révision
export const FORMES = [
  { name: 'cercle', sub: 'parfaitement rond' },
  { name: 'carré', sub: '4 côtés égaux' },
  { name: 'triangle', sub: '3 côtés' },
  { name: 'rectangle', sub: '4 côtés, 2 longs et 2 courts' },
  { name: 'losange', sub: '4 côtés égaux, penché' },
  { name: 'ovale', sub: 'comme un œuf' },
  { name: 'étoile', sub: '5 branches' },
  { name: 'cœur', sub: 'comme dans « je t\'aime »' },
  { name: 'hexagone', sub: '6 côtés' },
  { name: 'octogone', sub: '8 côtés' },
]

// nombre de côtés (formes où la question a un sens)
const FORME_COTES = { triangle: 3, carré: 4, rectangle: 4, losange: 4, hexagone: 6, octogone: 8 }
// nombres de côtés qui désignent une seule forme (pour la question inverse)
const COTES_UNIQUES = { 3: 'triangle', 6: 'hexagone', 8: 'octogone' }

// objets du quotidien à la forme non ambiguë (complément prêt à insérer)
const OBJETS_FORME = [
  { objet: "d'une pièce de monnaie", forme: 'cercle' },
  { objet: "d'un ballon", forme: 'cercle' },
  { objet: "d'un œuf", forme: 'ovale' },
  { objet: "d'une part de pizza", forme: 'triangle' },
  { objet: "d'un panneau stop", forme: 'octogone' },
  { objet: "d'une porte", forme: 'rectangle' },
  { objet: 'des alvéoles des abeilles', forme: 'hexagone' },
]

// items { num, label } pour tous les nombres de a à b (cap et numberToWords sont hissés)
function numberRangeItems(a, b) {
  return Array.from({ length: b - a + 1 }, (_, i) => a + i)
    .map(n => ({ num: n, label: cap(numberToWords(n)) }))
}

export const LEARN_DATA = {
  jours: { title: '📖 Les 7 jours de la semaine', items: JOURS.map(j => ({ label: cap(j) })) },
  mois: { title: "📖 Les 12 mois de l'année", items: MOIS.map(m => ({ label: cap(m) })) },
  saisons: {
    title: '📖 Les 4 saisons',
    items: [
      { label: '🌸 Printemps', sub: 'mars, avril, mai' },
      { label: '☀️ Été', sub: 'juin, juillet, août' },
      { label: '🍂 Automne', sub: 'septembre, octobre, novembre' },
      { label: '❄️ Hiver', sub: 'décembre, janvier, février' },
    ],
  },
  alphabet: { title: '📖 Les 26 lettres de l\'alphabet', grid: ALPHABET },
  couleurs: {
    title: '📖 Les 11 couleurs',
    items: COULEURS.map(c => ({ label: cap(c.name), color: c.hex })),
  },
  formes: {
    title: '📖 Les 10 formes géométriques',
    items: FORMES.map(f => ({ label: cap(f.name), sub: f.sub, shape: f.name })),
  },
  chiffres: {
    title: '📖 Les chiffres de 0 à 9',
    items: Array.from({ length: 10 }, (_, i) => ({ label: cap(UNITES[i]), num: i, marbles: i })),
  },
  cinquante: {
    title: '📖 Les dizaines jusqu\'à 50',
    intro: 'Une rangée de 10 billes = 1 dizaine',
    items: [10, 20, 30, 40, 50].map(n => ({
      label: cap(numberToWords(n)),
      num: n,
      marbles: n,
      perRow: 10,
      colorByRow: true,
      sub: `${n / 10} dizaine${n > 10 ? 's' : ''}`,
    })),
  },
  nombres: {
    title: '📖 Les nombres en lettres',
    intro: 'Appuie sur une dizaine pour dérouler tous les nombres',
    // accordéon : une section par dizaine, à dérouler pour voir chaque nombre
    groups: [
      ...Array.from({ length: 10 }, (_, d) => ({
        label: `De ${d * 10} à ${d * 10 + 9}`,
        items: numberRangeItems(d * 10, d * 10 + 9),
      })),
      { label: 'Cent', items: numberRangeItems(100, 100) },
    ],
  },
}

function rand(min, max) { return Math.floor(Math.random() * (max - min + 1)) + min }
function pick(arr) { return arr[rand(0, arr.length - 1)] }

export function shuffle(arr) {
  const a = arr.slice()
  for (let i = a.length - 1; i > 0; i--) {
    const j = rand(0, i)
    ;[a[i], a[j]] = [a[j], a[i]]
  }
  return a
}

export function cap(s) { return s.charAt(0).toUpperCase() + s.slice(1) }
function ordinal(n, feminine) { return n === 1 ? (feminine ? '1re' : '1er') : n + 'e' }

// renvoie `count` éléments de la liste différents de l'index donné
function distinctFrom(list, excludeIndex, count) {
  const others = list.filter((_, i) => i !== excludeIndex)
  return shuffle(others).slice(0, count)
}

/*
 * Questions sur une liste ordonnée (jours, mois, saisons, alphabet).
 * labels :
 *   unit / unitPlural  : "jour" / "jours"
 *   feminine           : true pour "lettre", "saison" (Quelle… / la 1re…)
 *   container          : "la semaine", "l'année", "l'alphabet"
 *   container2         : "une semaine" (pour "Combien de … dans …")
 *   cyclic             : true si le dernier est suivi du premier
 *   display            : fonction d'affichage d'un élément dans la question
 */
function makeSequenceQuestion(list, labels) {
  const type = pick(labels.types || ['apres', 'avant', 'position', 'compte', 'premier', 'dernier'])
  const wrap = i => ((i % list.length) + list.length) % list.length
  const quel = labels.feminine ? 'Quelle' : 'Quel'
  const show = labels.display || (x => x)
  const fmt = cap

  if (type === 'apres') {
    // sans cycle (alphabet), on ne pose pas la question sur le dernier élément
    const i = labels.cyclic ? rand(0, list.length - 1) : rand(0, list.length - 2)
    return {
      q: `${quel} ${labels.unit} vient juste après ${show(list[i])} ?`,
      answer: fmt(list[wrap(i + 1)]),
      choices: distinctFrom(list, wrap(i + 1), 3).map(fmt),
    }
  }
  if (type === 'avant') {
    const i = labels.cyclic ? rand(0, list.length - 1) : rand(1, list.length - 1)
    return {
      q: `${quel} ${labels.unit} vient juste avant ${show(list[i])} ?`,
      answer: fmt(list[wrap(i - 1)]),
      choices: distinctFrom(list, wrap(i - 1), 3).map(fmt),
    }
  }
  if (type === 'position') {
    const i = rand(0, list.length - 1)
    const article = labels.feminine ? 'la' : 'le'
    return {
      q: `${quel} est ${article} ${ordinal(i + 1, labels.feminine)} ${labels.unit} de ${labels.container} ?`,
      answer: fmt(list[i]),
      choices: distinctFrom(list, i, 3).map(fmt),
    }
  }
  if (type === 'compte') {
    const n = list.length
    return {
      q: `Combien y a-t-il de ${labels.unitPlural} dans ${labels.container2} ?`,
      answer: String(n),
      choices: shuffle([n - 2, n - 1, n + 1, n + 2]).slice(0, 3).map(String),
    }
  }
  if (type === 'premier') {
    const first = labels.feminine ? 'la première' : 'le premier'
    return {
      q: `${quel} est ${first} ${labels.unit} de ${labels.container} ?`,
      answer: fmt(list[0]),
      choices: distinctFrom(list, 0, 3).map(fmt),
    }
  }
  const last = labels.feminine ? 'la dernière' : 'le dernier'
  return {
    q: `${quel} est ${last} ${labels.unit} de ${labels.container} ?`,
    answer: fmt(list[list.length - 1]),
    choices: distinctFrom(list, list.length - 1, 3).map(fmt),
  }
}

// question d'association mois → saison
function makeMoisSaisonQuestion() {
  const mois = pick(Object.keys(MOIS_SAISON))
  const saison = MOIS_SAISON[mois]
  return {
    q: `En quelle saison est le mois de ${mois} ?`,
    answer: cap(saison),
    choices: SAISONS.filter(s => s !== saison).map(cap),
  }
}

// ---------- couleurs ----------
function otherColorNames(exclude, count) {
  return shuffle(COULEURS.filter(c => !exclude.includes(c.name))).slice(0, count).map(c => cap(c.name))
}

function makeCouleursQuestion() {
  const type = pick(['pastille', 'pastille', 'melange', 'objet']) // la reconnaissance visuelle revient plus souvent
  if (type === 'pastille') {
    const c = pick(COULEURS)
    return {
      q: 'Quelle est cette couleur ?',
      key: `pastille:${c.name}`, // le texte est identique pour toutes : on déduplique sur la couleur
      swatch: c.hex,
      answer: cap(c.name),
      choices: otherColorNames([c.name], 3),
    }
  }
  if (type === 'melange') {
    const m = pick(MELANGES)
    return {
      q: `Quelle couleur obtient-on en mélangeant du ${m.a} et du ${m.b} ?`,
      answer: cap(m.donne),
      choices: otherColorNames([m.a, m.b, m.donne], 3),
    }
  }
  const o = pick(OBJETS_COULEUR)
  return {
    q: `De quelle couleur est ${o.objet} ?`,
    answer: cap(o.couleur),
    choices: otherColorNames([o.couleur], 3),
  }
}

// ---------- formes géométriques ----------
function otherShapeNames(exclude, count) {
  return shuffle(FORMES.filter(f => !exclude.includes(f.name))).slice(0, count).map(f => cap(f.name))
}

function makeFormesQuestion() {
  const type = pick(['visuelle', 'visuelle', 'cotes', 'inverse', 'objet']) // la reconnaissance visuelle revient plus souvent
  if (type === 'visuelle') {
    const f = pick(FORMES)
    return {
      q: 'Quelle est cette forme ?',
      key: `forme:${f.name}`, // le texte est identique pour toutes : on déduplique sur la forme
      shape: f.name,
      answer: cap(f.name),
      choices: otherShapeNames([f.name], 3),
    }
  }
  if (type === 'cotes') {
    // une fois sur six, la question sur les branches de l'étoile
    if (rand(0, 5) === 0) {
      return {
        q: 'Combien de branches a une étoile ?',
        answer: '5',
        choices: shuffle(['3', '4', '6', '7']).slice(0, 3),
      }
    }
    const name = pick(Object.keys(FORME_COTES))
    const n = FORME_COTES[name]
    const distractors = new Set()
    let d = 3
    while (distractors.size < 3) { if (d !== n) distractors.add(String(d)); d++ }
    return {
      q: `Combien de côtés a un ${name} ?`,
      shape: name,
      answer: String(n),
      choices: shuffle([...distractors]).slice(0, 3),
    }
  }
  if (type === 'inverse') {
    const n = pick(Object.keys(COTES_UNIQUES))
    const name = COTES_UNIQUES[n]
    return {
      q: `Quelle forme a ${n} côtés ?`,
      answer: cap(name),
      choices: otherShapeNames([name], 3),
    }
  }
  const o = pick(OBJETS_FORME)
  return {
    q: `Quelle est la forme ${o.objet} ?`,
    answer: cap(o.forme),
    choices: otherShapeNames([o.forme], 3),
  }
}

// ---------- chiffres de 0 à 9, avec des billes à compter ----------
function digitDistractors(answer, min, max) {
  const others = []
  for (let i = min; i <= max; i++) if (i !== answer) others.push(String(i))
  return shuffle(others).slice(0, 3)
}

// 3 noms de chiffres (en lettres) différents de n, parmi 0 à 9
function digitWordDistractors(n) {
  const others = []
  for (let i = 0; i <= 9; i++) if (i !== n) others.push(cap(UNITES[i]))
  return shuffle(others).slice(0, 3)
}

function makeChiffresQuestion() {
  // compter des billes revient plus souvent ; le chiffre et son écriture se répondent dans les deux sens
  const type = pick(['compter', 'compter', 'ecrire', 'lire', 'apres', 'avant'])
  if (type === 'compter') {
    const n = rand(1, 9)
    return {
      q: 'Combien de billes comptes-tu ?',
      key: `billes:${n}`, // le texte est identique : on déduplique sur le nombre de billes
      marbles: n,
      answer: String(n),
      choices: digitDistractors(n, Math.max(0, n - 3), Math.min(9, n + 3)),
    }
  }
  if (type === 'ecrire') { // chiffre → lettres
    const n = rand(0, 9)
    return {
      q: `Comment s'écrit le chiffre ${n} ?`,
      answer: cap(UNITES[n]),
      choices: digitWordDistractors(n),
    }
  }
  if (type === 'lire') { // lettres → chiffre
    const n = rand(0, 9)
    return {
      q: `Quel chiffre s'écrit « ${UNITES[n]} » ?`,
      answer: String(n),
      choices: digitDistractors(n, 0, 9),
    }
  }
  if (type === 'apres') {
    const i = rand(0, 8)
    return {
      q: `Quel chiffre vient juste après ${i} ?`,
      answer: String(i + 1),
      choices: digitDistractors(i + 1, 0, 9),
    }
  }
  const i = rand(1, 9)
  return {
    q: `Quel chiffre vient juste avant ${i} ?`,
    answer: String(i - 1),
    choices: digitDistractors(i - 1, 0, 9),
  }
}

// ---------- nombres jusqu'à 50, dizaines et unités ----------
// 3 nombres proches et distincts dans [min, max] (dont les pièges ±10 et ±1)
function nearbyNumbers(n, min, max) {
  const candidates = shuffle([n - 10, n + 10, n - 1, n + 1, n - 2, n + 2, n - 11, n + 11])
    .filter(c => c >= min && c <= max && c !== n)
  const set = new Set(candidates.slice(0, 3).map(String))
  let extra = min
  while (set.size < 3) { if (extra !== n) set.add(String(extra)); extra++ }
  return [...set]
}

function makeCinquanteQuestion() {
  const type = pick(['compter', 'compter', 'dizaines', 'unites', 'composer', 'suite'])

  if (type === 'compter') {
    const n = rand(11, 50)
    return {
      q: 'Combien de billes comptes-tu ?',
      key: `billes50:${n}`, // le texte est identique : on déduplique sur le nombre
      marbles: n,
      perRow: 10,
      colorByRow: true,
      answer: String(n),
      choices: nearbyNumbers(n, 11, 50),
    }
  }
  if (type === 'dizaines') {
    const n = rand(11, 50)
    const d = Math.floor(n / 10)
    return {
      q: `Dans le nombre ${n}, combien y a-t-il de dizaines ?`,
      answer: String(d),
      choices: digitDistractors(d, 0, 5),
    }
  }
  if (type === 'unites') {
    const n = rand(11, 49)
    const u = n % 10
    return {
      q: `Dans le nombre ${n}, combien y a-t-il d'unités ?`,
      answer: String(u),
      choices: digitDistractors(u, 0, 9),
    }
  }
  if (type === 'composer') {
    const d = rand(1, 4)
    const u = rand(1, 9)
    const n = 10 * d + u
    const inversion = 10 * u + d // le piège classique : dizaines et unités inversées
    const candidates = [inversion, n - 1, n + 1, 10 * d, n + 10, n - 10]
      .filter(c => c >= 1 && c <= 50 && c !== n)
    const set = new Set(candidates.map(String))
    let extra = n + 2
    while (set.size < 3) { set.add(String(extra)); extra++ }
    return {
      q: `Avec ${d} dizaine${d > 1 ? 's' : ''} et ${u} unité${u > 1 ? 's' : ''}, quel nombre écris-tu ?`,
      answer: String(n),
      choices: shuffle([...set]).slice(0, 3),
    }
  }
  // suite : le nombre d'avant ou d'après, avec les passages de dizaine
  if (rand(0, 1)) {
    const n = rand(10, 49)
    return {
      q: `Quel nombre vient juste après ${n} ?`,
      answer: String(n + 1),
      choices: nearbyNumbers(n + 1, 0, 51),
    }
  }
  const n = rand(11, 50)
  return {
    q: `Quel nombre vient juste avant ${n} ?`,
    answer: String(n - 1),
    choices: nearbyNumbers(n - 1, 0, 50),
  }
}

// ---------- nombres en lettres ----------
export function numberToWords(n) {
  if (n < 20) return UNITES[n]
  if (n === 100) return 'cent'
  let dizaine = Math.floor(n / 10) * 10
  let unite = n - dizaine
  // 70–79 et 90–99 se disent "soixante-dix…" et "quatre-vingt-dix…"
  if (dizaine === 70 || dizaine === 90) { dizaine -= 10; unite += 10 }
  if (unite === 0) return dizaine === 80 ? 'quatre-vingts' : DIZAINES[dizaine]
  if ((unite === 1 || unite === 11) && dizaine !== 80) return `${DIZAINES[dizaine]} et ${UNITES[unite]}`
  return `${DIZAINES[dizaine]}-${UNITES[unite]}`
}

function makeNombresQuestion(level) {
  const [min, max] = [[0, 16], [17, 69], [60, 100]][level - 1]
  const n = rand(min, max)
  // 3 nombres proches mais différents, dans la plage du niveau
  const distractors = new Set()
  let guard = 0
  while (distractors.size < 3 && guard < 100) {
    guard++
    const d = Math.min(Math.max(n + (rand(0, 1) ? rand(1, 6) : -rand(1, 6)), min), max)
    if (d !== n) distractors.add(d)
  }
  let extra = min
  while (distractors.size < 3) { if (extra !== n) distractors.add(extra); extra++ }
  const nums = [...distractors]

  if (rand(0, 1)) {
    return {
      q: `Comment s'écrit le nombre ${n} ?`,
      answer: numberToWords(n),
      choices: nums.map(numberToWords),
    }
  }
  return {
    q: `Quel nombre s'écrit « ${numberToWords(n)} » ?`,
    answer: String(n),
    choices: nums.map(String),
  }
}

// ---------- calcul ----------
// 3 mauvaises réponses proches du résultat, uniques et positives
function numberDistractors(answer, count) {
  const set = new Set()
  let guard = 0
  while (set.size < count && guard < 200) {
    guard++
    const delta = rand(1, Math.max(3, Math.round(Math.abs(answer) * 0.3)))
    const candidate = answer + (Math.random() < 0.5 ? -delta : delta)
    if (candidate >= 0 && candidate !== answer) set.add(candidate)
  }
  // filet de sécurité si le hasard n'a pas suffi
  let extra = answer + count + 1
  while (set.size < count) set.add(extra++)
  return [...set]
}

function makeMathQuestion(op, level) {
  let a, b, result, symbol

  if (op === 'addition') {
    const max = [10, 20, 100][level - 1]
    a = rand(1, max); b = rand(1, max)
    result = a + b; symbol = '+'
  } else if (op === 'soustraction') {
    const max = [10, 20, 100][level - 1]
    a = rand(1, max); b = rand(1, max)
    if (b > a) [a, b] = [b, a] // jamais de résultat négatif
    result = a - b; symbol = '−'
  } else if (op === 'multiplication') {
    const max = [5, 10, 12][level - 1]
    a = rand(1, max); b = rand(1, 10)
    result = a * b; symbol = '×'
  } else { // division exacte
    const max = [5, 10, 12][level - 1]
    b = rand(2, max)
    result = rand(1, max)
    a = b * result; symbol = '÷'
  }

  return {
    q: `${a} ${symbol} ${b} = ?`,
    answer: String(result),
    choices: numberDistractors(result, 3).map(String),
  }
}

// ---------- construction d'un quiz ----------
function makeQuestion(category, level) {
  if (category === 'jours') {
    return makeSequenceQuestion(JOURS, {
      unit: 'jour', unitPlural: 'jours', feminine: false, cyclic: true,
      container: 'la semaine', container2: 'une semaine',
    })
  }
  if (category === 'mois') {
    return makeSequenceQuestion(MOIS, {
      unit: 'mois', unitPlural: 'mois', feminine: false, cyclic: true,
      container: "l'année", container2: 'une année',
    })
  }
  if (category === 'saisons') {
    // moitié de questions d'ordre, moitié d'association mois → saison
    if (rand(0, 1)) return makeMoisSaisonQuestion()
    // pas de question de position : « la 1re saison de l'année » serait ambiguë
    return makeSequenceQuestion(SAISONS, {
      unit: 'saison', unitPlural: 'saisons', feminine: true, cyclic: true,
      container: "l'année", container2: 'une année',
      display: s => SAISON_ARTICLE[s],
      types: ['apres', 'avant', 'compte'],
    })
  }
  if (category === 'alphabet') {
    return makeSequenceQuestion(ALPHABET, {
      unit: 'lettre', unitPlural: 'lettres', feminine: true, cyclic: false,
      container: "l'alphabet", container2: "l'alphabet",
      display: l => `la lettre ${l}`,
    })
  }
  if (category === 'couleurs') return makeCouleursQuestion()
  if (category === 'formes') return makeFormesQuestion()
  if (category === 'chiffres') return makeChiffresQuestion()
  if (category === 'cinquante') return makeCinquanteQuestion()
  if (category === 'nombres') return makeNombresQuestion(level)
  return makeMathQuestion(category, level)
}

// thèmes puisés par le mode Mélange (toutes les catégories réelles, sauf le mélange lui-même)
const MELANGE_SOURCES = ['chiffres', 'cinquante', 'nombres', 'addition', 'soustraction',
  'multiplication', 'division', 'jours', 'mois', 'saisons', 'alphabet', 'couleurs', 'formes']

// Entrelacement : on parcourt les thèmes en rotation pour que deux questions
// voisines viennent presque toujours de thèmes différents.
function buildInterleaved() {
  const sources = shuffle(MELANGE_SOURCES)
  const qs = []
  const seen = new Set()
  let i = 0, guard = 0
  while (qs.length < NB_QUESTIONS && guard < 400) {
    guard++
    const cat = sources[i % sources.length]
    i++
    const level = rand(1, 2) // difficulté douce et variée pour les thèmes à niveaux
    const q = makeQuestion(cat, level)
    const key = `${cat}:${q.key || q.q}`
    if (seen.has(key)) continue // éviter deux fois la même question
    seen.add(key)
    qs.push({ ...q, category: cat, options: shuffle([q.answer, ...q.choices]) })
  }
  return qs
}

export function buildQuestions(category, level) {
  if (category === 'melange') return buildInterleaved()
  const qs = []
  const seen = new Set()
  let guard = 0
  while (qs.length < NB_QUESTIONS && guard < 300) {
    guard++
    const q = makeQuestion(category, level)
    const key = q.key || q.q
    if (seen.has(key)) continue // éviter deux fois la même question
    seen.add(key)
    qs.push({ ...q, options: shuffle([q.answer, ...q.choices]) })
  }
  return qs
}

export function pickPraise() {
  return pick(['✅ Bravo !', '✅ Super !', '✅ Exact !', '✅ Génial !'])
}

export function endSummary(score, total) {
  const ratio = score / total
  if (ratio === 1) return { stars: '⭐⭐⭐⭐⭐', msg: 'Score parfait, tu es un champion ! 🏆' }
  if (ratio >= 0.8) return { stars: '⭐⭐⭐⭐', msg: 'Excellent travail, continue comme ça ! 🎉' }
  if (ratio >= 0.6) return { stars: '⭐⭐⭐', msg: 'Bien joué ! Encore un petit effort ! 💪' }
  if (ratio >= 0.4) return { stars: '⭐⭐', msg: 'Pas mal ! Entraîne-toi encore un peu ! 🙂' }
  return { stars: '⭐', msg: "Courage, réessaie : c'est en s'entraînant qu'on apprend ! 🌱" }
}
