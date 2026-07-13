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

export const CATEGORIES = {
  jours: { emoji: '📅', label: 'Les jours', title: 'Les jours de la semaine', kind: 'sequence' },
  mois: { emoji: '🗓️', label: 'Les mois', title: "Les mois de l'année", kind: 'sequence' },
  saisons: { emoji: '🍂', label: 'Les saisons', title: 'Les saisons', kind: 'sequence' },
  alphabet: { emoji: '🔤', label: "L'alphabet", title: "L'alphabet", kind: 'sequence' },
  nombres: { emoji: '🔢', label: 'Les nombres', title: 'Les nombres en lettres', kind: 'math' },
  addition: { emoji: '➕', label: 'Addition', title: 'Addition', kind: 'math' },
  soustraction: { emoji: '➖', label: 'Soustraction', title: 'Soustraction', kind: 'math' },
  multiplication: { emoji: '✖️', label: 'Multiplication', title: 'Multiplication', kind: 'math' },
  division: { emoji: '➗', label: 'Division', title: 'Division', kind: 'math' },
}

export const LEVELS = [
  { id: 1, emoji: '🌱', label: 'Facile' },
  { id: 2, emoji: '🌿', label: 'Moyen' },
  { id: 3, emoji: '🌳', label: 'Difficile' },
]

// contenu des écrans de révision
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

// ---------- nombres en lettres ----------
const UNITES = ['zéro', 'un', 'deux', 'trois', 'quatre', 'cinq', 'six', 'sept', 'huit', 'neuf',
  'dix', 'onze', 'douze', 'treize', 'quatorze', 'quinze', 'seize', 'dix-sept', 'dix-huit', 'dix-neuf']
const DIZAINES = { 20: 'vingt', 30: 'trente', 40: 'quarante', 50: 'cinquante', 60: 'soixante', 80: 'quatre-vingt' }

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
  if (category === 'nombres') return makeNombresQuestion(level)
  return makeMathQuestion(category, level)
}

export function buildQuestions(category, level) {
  const qs = []
  const seen = new Set()
  let guard = 0
  while (qs.length < NB_QUESTIONS && guard < 300) {
    guard++
    const q = makeQuestion(category, level)
    if (seen.has(q.q)) continue // éviter deux fois la même question
    seen.add(q.q)
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
