export const JOURS = ['lundi', 'mardi', 'mercredi', 'jeudi', 'vendredi', 'samedi', 'dimanche']
export const MOIS = ['janvier', 'février', 'mars', 'avril', 'mai', 'juin',
  'juillet', 'août', 'septembre', 'octobre', 'novembre', 'décembre']
export const NB_QUESTIONS = 10

export const CATEGORIES = {
  jours: { emoji: '📅', label: 'Les jours', title: 'Les jours de la semaine', kind: 'sequence' },
  mois: { emoji: '🗓️', label: 'Les mois', title: "Les mois de l'année", kind: 'sequence' },
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
function ordinal(n) { return n === 1 ? '1er' : n + 'e' }

// renvoie `count` éléments de la liste différents de l'index donné
function distinctFrom(list, excludeIndex, count) {
  const others = list.filter((_, i) => i !== excludeIndex)
  return shuffle(others).slice(0, count)
}

function makeSequenceQuestion(list, labels) {
  const type = pick(['apres', 'avant', 'position', 'compte', 'premier', 'dernier'])
  const wrap = i => ((i % list.length) + list.length) % list.length

  if (type === 'apres') {
    const i = rand(0, list.length - 1)
    return {
      q: `Quel ${labels.unit} vient juste après ${list[i]} ?`,
      answer: cap(list[wrap(i + 1)]),
      choices: distinctFrom(list, wrap(i + 1), 3).map(cap),
    }
  }
  if (type === 'avant') {
    const i = rand(0, list.length - 1)
    return {
      q: `Quel ${labels.unit} vient juste avant ${list[i]} ?`,
      answer: cap(list[wrap(i - 1)]),
      choices: distinctFrom(list, wrap(i - 1), 3).map(cap),
    }
  }
  if (type === 'position') {
    const i = rand(0, list.length - 1)
    return {
      q: `Quel est le ${ordinal(i + 1)} ${labels.unit} de ${labels.container} ?`,
      answer: cap(list[i]),
      choices: distinctFrom(list, i, 3).map(cap),
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
    return {
      q: `Quel est le premier ${labels.unit} de ${labels.container} ?`,
      answer: cap(list[0]),
      choices: distinctFrom(list, 0, 3).map(cap),
    }
  }
  return {
    q: `Quel est le dernier ${labels.unit} de ${labels.container} ?`,
    answer: cap(list[list.length - 1]),
    choices: distinctFrom(list, list.length - 1, 3).map(cap),
  }
}

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

export function buildQuestions(category, level) {
  const qs = []
  const seen = new Set()
  let guard = 0
  while (qs.length < NB_QUESTIONS && guard < 300) {
    guard++
    let q
    if (category === 'jours') {
      q = makeSequenceQuestion(JOURS, {
        unit: 'jour', unitPlural: 'jours',
        container: 'la semaine', container2: 'une semaine',
      })
    } else if (category === 'mois') {
      q = makeSequenceQuestion(MOIS, {
        unit: 'mois', unitPlural: 'mois',
        container: "l'année", container2: 'une année',
      })
    } else {
      q = makeMathQuestion(category, level)
    }
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
