// ─────────────────────────────────────────────────────────────────────────────
// Types du moteur
// ─────────────────────────────────────────────────────────────────────────────

/** Clé d'une catégorie : « jours », « heure », « addition »… */
export type CategoryKey = keyof typeof CATEGORIES

export type Level = { id: number; emoji: string; label: string }

export type Category = {
  emoji: string
  label: string
  title: string
  /** Propose un écran de révision. */
  hasLearn?: boolean
  /** Propose un choix de niveau. */
  hasLevels?: boolean
  /** Niveaux propres à la catégorie ; absent = niveaux communs. */
  levels?: Level[]
}

/** Une question posée : l'énoncé, la bonne réponse, les leurres, et de quoi
 *  l'illustrer (pastille de couleur, forme, horloge, billes, étapes). */
export type Question = {
  q: string
  answer: string
  choices: string[]
  /** Les quatre propositions mélangées ; ajoutées par buildQuestions. */
  options?: string[]
  /** Identifiant servant à éviter les doublons dans un même quiz. */
  key?: string
  /** Thème d'origine, en mode Mélange. */
  category?: CategoryKey
  swatch?: string
  shape?: string
  clock?: { hours: number; minutes: number }
  marbles?: number
  perRow?: number
  colorByRow?: boolean
  /** Étapes montrées sous l'énoncé (planification). */
  liste?: string[]
  /** Position du trou dans `liste`, pour « quelle étape manque ? ». */
  trou?: number
  /** Activité d'origine, pour les questions de planification. */
  routine?: Routine
}

/** Une ligne d'un écran de révision. */
export type LearnItem = {
  label: string
  sub?: string
  num?: number
  color?: string
  shape?: string
  marbles?: number
  perRow?: number
  colorByRow?: boolean
  clock?: { hours: number; minutes: number }
}

export type LearnGroup = { label: string; items: LearnItem[] }

/** Un écran de révision : liste simple, grille de lettres, ou accordéon. */
export type LearnContent = {
  title: string
  intro?: string
  items?: LearnItem[]
  grid?: string[]
  groups?: LearnGroup[]
}

export type Routine = {
  key: string
  emoji: string
  label: string
  /** Durée estimée, en minutes. */
  duree: number
  etapes: string[]
}

export type EndSummary = { stars: string; msg: string }

/** Réglages d'une question de séquence (jours, mois, saisons, alphabet). */
export type SeqLabels = {
  unit: string
  unitPlural: string
  feminine?: boolean
  cyclic?: boolean
  container: string
  container2: string
  display?: (x: string) => string
  types?: string[]
}

export const JOURS = ['lundi', 'mardi', 'mercredi', 'jeudi', 'vendredi', 'samedi', 'dimanche']
export const MOIS = ['janvier', 'février', 'mars', 'avril', 'mai', 'juin',
  'juillet', 'août', 'septembre', 'octobre', 'novembre', 'décembre']
export const ALPHABET = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')
export const SAISONS = ['printemps', 'été', 'automne', 'hiver']
export const NB_QUESTIONS = 10

// article devant chaque saison ("après l'été", "après le printemps")
const SAISON_ARTICLE: Record<string, string> = { printemps: 'le printemps', 'été': "l'été", automne: "l'automne", hiver: "l'hiver" }

// mois clairement dans une seule saison (on évite les mois de changement de saison)
const MOIS_SAISON: Record<string, string> = {
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
  heure: {
    emoji: '🕐', label: "L'heure", title: "Lire l'heure", hasLearn: true, hasLevels: true,
    // niveaux propres à l'heure : de l'heure pile à la minute près
    levels: [
      { id: 1, emoji: '🌱', label: 'Heures pleines' },
      { id: 2, emoji: '🌿', label: 'Quarts et demies' },
      { id: 3, emoji: '🌳', label: 'De 5 en 5' },
      { id: 4, emoji: '🏆', label: 'Minute par minute' },
    ],
  },
  planifier: {
    emoji: '📋', label: 'Planifier', title: 'Apprendre à planifier',
    hasLearn: true, hasLevels: true,
    // du plus concret au plus abstrait : les étapes, puis leur ordre, puis le temps
    levels: [
      { id: 1, emoji: '🌱', label: 'Les étapes' },
      { id: 2, emoji: '🌿', label: "L'ordre" },
      { id: 3, emoji: '🌳', label: 'Le temps' },
    ],
  },
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

// Niveaux d'une catégorie : les siens s'ils sont définis, sinon les niveaux communs.
export function levelsFor(category: string): Level[] {
  return (CATEGORIES as Record<string, Category>)[category]?.levels || LEVELS
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
const DIZAINES: Record<number, string> = { 20: 'vingt', 30: 'trente', 40: 'quarante', 50: 'cinquante', 60: 'soixante', 80: 'quatre-vingt' }

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
const FORME_COTES: Record<string, number> = { triangle: 3, carré: 4, rectangle: 4, losange: 4, hexagone: 6, octogone: 8 }
// nombres de côtés qui désignent une seule forme (pour la question inverse)
const COTES_UNIQUES: Record<number, string> = { 3: 'triangle', 6: 'hexagone', 8: 'octogone' }

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

// item de révision pour une horloge : le libellé et le repère de la grande
// aiguille sont déduits de l'horaire, donc toujours cohérents entre eux
function clockLearnItem(h: number, m: number): LearnItem {
  return {
    label: cap(timeToWords(h, m)),
    sub: `la grande aiguille est sur le ${m === 0 ? 12 : m / 5}`,
    clock: { hours: h, minutes: m },
  }
}

// items { num, label } pour tous les nombres de a à b (cap et numberToWords sont hissés)
function numberRangeItems(a: number, b: number): LearnItem[] {
  return Array.from({ length: b - a + 1 }, (_, i) => a + i)
    .map(n => ({ num: n, label: cap(numberToWords(n)) }))
}

// ─────────────────────────────────────────────────────────────────────────────
// Planifier — décomposer une activité en étapes, les ordonner, estimer le temps
// qu'elles prennent. C'est une fonction exécutive, et elle se rééduque.
//
// Les activités sont celles de la vie quotidienne, et leurs étapes s'enchaînent
// vraiment : chaque étape suppose la précédente, pour qu'il n'y ait qu'un seul
// ordre correct. « duree » est une estimation en minutes, volontairement ronde.
export const ROUTINES = [
  { key: 'medicament', emoji: '💊', label: 'Prendre son médicament', duree: 2, etapes: [
    "Regarder l'ordonnance",
    'Sortir la bonne boîte',
    'Prendre le comprimé avec un verre d\'eau',
    'Cocher sur le carnet',
  ] },
  { key: 'dents', emoji: '🦷', label: 'Se brosser les dents', duree: 3, etapes: [
    'Prendre la brosse à dents',
    'Mettre le dentifrice dessus',
    'Se brosser les dents',
    'Se rincer la bouche',
  ] },
  { key: 'cafe', emoji: '☕', label: 'Se faire un café', duree: 5, etapes: [
    "Faire chauffer l'eau",
    'Mettre le café dans la tasse',
    "Verser l'eau chaude dans la tasse",
    'Remuer avec la cuillère',
  ] },
  { key: 'sandwich', emoji: '🥪', label: 'Préparer un sandwich', duree: 5, etapes: [
    'Sortir le pain et le jambon',
    'Couper le pain en deux',
    'Étaler le beurre',
    'Poser le jambon dessus',
    'Refermer le sandwich',
  ] },
  { key: 'lessive', emoji: '🧺', label: 'Faire une lessive', duree: 10, etapes: [
    'Trier le linge sale',
    'Mettre le linge dans la machine',
    'Ajouter la lessive',
    'Lancer la machine',
    'Étendre le linge',
  ] },
  { key: 'habiller', emoji: '👕', label: "S'habiller", duree: 10, etapes: [
    'Choisir ses vêtements',
    'Enlever son pyjama',
    'Mettre son pantalon et son haut',
    'Mettre ses chaussettes',
    'Mettre ses chaussures',
  ] },
  { key: 'pates', emoji: '🍝', label: 'Préparer des pâtes', duree: 15, etapes: [
    "Remplir la casserole d'eau",
    "Faire bouillir l'eau",
    'Verser les pâtes dans l\'eau',
    'Attendre la cuisson',
    'Égoutter les pâtes',
  ] },
  { key: 'douche', emoji: '🚿', label: 'Prendre une douche', duree: 15, etapes: [
    'Préparer sa serviette',
    "Régler la température de l'eau",
    'Se laver',
    'Se sécher avec la serviette',
  ] },
  { key: 'lettre', emoji: '✉️', label: 'Envoyer une lettre', duree: 15, etapes: [
    'Écrire la lettre',
    "Mettre la lettre dans l'enveloppe",
    'Coller le timbre',
    'Poster la lettre dans la boîte',
  ] },
  { key: 'bus', emoji: '🚌', label: 'Prendre le bus', duree: 20, etapes: [
    "Regarder l'horaire du bus",
    "Aller à l'arrêt",
    'Monter dans le bus',
    'Valider son ticket',
    'Descendre au bon arrêt',
  ] },
  { key: 'courses', emoji: '🛒', label: 'Faire les courses', duree: 45, etapes: [
    'Écrire la liste des courses',
    'Aller au magasin',
    'Remplir le panier',
    'Passer à la caisse',
    'Ranger les courses',
  ] },
]

// « 9 h 00 », « 10 h 05 » — la notation d'un agenda, plus lisible ici que les
// lettres, et cohérente avec la catégorie « Lire l'heure ».
export function formatHeure(h: number, m: number): string {
  return `${h} h ${String(m).padStart(2, '0')}`
}

/** Ajoute des minutes à une heure, sur 24 h. */
export function ajouteMinutes(h: number, m: number, minutes: number): [number, number] {
  const total = ((h * 60 + m + minutes) % 1440 + 1440) % 1440
  return [Math.floor(total / 60), total % 60]
}

export const LEARN_DATA: Record<string, LearnContent> = {
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
    items: Array.from({ length: 10 }, (_, i) => ({ label: cap(at(UNITES, i)), num: i, marbles: i })),
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
  heure: {
    title: "📖 Lire l'heure",
    intro: 'La petite aiguille donne les heures, la grande donne les minutes. '
      + 'Entre deux chiffres du cadran, il y a 5 minutes.',
    // des heures variées, du plus simple au plus fin
    items: [
      [1, 0], [6, 0], [10, 0],
      [2, 15], [5, 30], [8, 45],
      [4, 5], [9, 20], [11, 40], [12, 50],
    ].map(([h, m]) => clockLearnItem(h as number, m as number)),
  },
  planifier: {
    title: '📖 Les étapes des activités',
    intro: "Appuie sur une activité pour dérouler ses étapes, dans l'ordre",
    groups: ROUTINES.map(r => ({
      label: `${r.emoji} ${r.label} — ${r.duree} min`,
      items: r.etapes.map((e, i) => ({ label: e, num: i + 1 })),
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


/**
 * Élément d'un tableau dont on sait l'index valide. Préféré à « ! » : si
 * l'hypothèse est fausse un jour, on le sait tout de suite au lieu de propager
 * un undefined silencieux.
 */
function at<T>(arr: readonly T[], i: number): T {
  const v = arr[i]
  if (v === undefined) throw new RangeError(`index ${i} hors du tableau de ${arr.length}`)
  return v
}

function rand(min: number, max: number): number { return Math.floor(Math.random() * (max - min + 1)) + min }
function pick<T>(arr: readonly T[]): T { return at(arr, rand(0, arr.length - 1)) }

export function shuffle<T>(arr: readonly T[]): T[] {
  const a = arr.slice()
  for (let i = a.length - 1; i > 0; i--) {
    const j = rand(0, i)
    ;[a[i], a[j]] = [at(a, j), at(a, i)]
  }
  return a
}

export function cap(s: string): string { return s.charAt(0).toUpperCase() + s.slice(1) }
function ordinal(n: number, feminine?: boolean): string { return n === 1 ? (feminine ? '1re' : '1er') : n + 'e' }

// renvoie `count` éléments de la liste différents de l'index donné
function distinctFrom(list: readonly string[], excludeIndex: number, count: number): string[] {
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
function makeSequenceQuestion(list: readonly string[], labels: SeqLabels): Question {
  const type = pick(labels.types || ['apres', 'avant', 'position', 'compte', 'premier', 'dernier'])
  const wrap = (i: number) => ((i % list.length) + list.length) % list.length
  const quel = labels.feminine ? 'Quelle' : 'Quel'
  const show = labels.display || (x => x)
  const fmt = cap

  if (type === 'apres') {
    // sans cycle (alphabet), on ne pose pas la question sur le dernier élément
    const i = labels.cyclic ? rand(0, list.length - 1) : rand(0, list.length - 2)
    return {
      q: `${quel} ${labels.unit} vient juste après ${show(at(list, i))} ?`,
      answer: fmt(at(list, wrap(i + 1))),
      choices: distinctFrom(list, wrap(i + 1), 3).map(fmt),
    }
  }
  if (type === 'avant') {
    const i = labels.cyclic ? rand(0, list.length - 1) : rand(1, list.length - 1)
    return {
      q: `${quel} ${labels.unit} vient juste avant ${show(at(list, i))} ?`,
      answer: fmt(at(list, wrap(i - 1))),
      choices: distinctFrom(list, wrap(i - 1), 3).map(fmt),
    }
  }
  if (type === 'position') {
    const i = rand(0, list.length - 1)
    const article = labels.feminine ? 'la' : 'le'
    return {
      q: `${quel} est ${article} ${ordinal(i + 1, labels.feminine)} ${labels.unit} de ${labels.container} ?`,
      answer: fmt(at(list, i)),
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
      answer: fmt(at(list, 0)),
      choices: distinctFrom(list, 0, 3).map(fmt),
    }
  }
  const last = labels.feminine ? 'la dernière' : 'le dernier'
  return {
    q: `${quel} est ${last} ${labels.unit} de ${labels.container} ?`,
    answer: fmt(at(list, list.length - 1)),
    choices: distinctFrom(list, list.length - 1, 3).map(fmt),
  }
}

// question d'association mois → saison
function makeMoisSaisonQuestion(): Question {
  const mois = pick(Object.keys(MOIS_SAISON))
  const saison = MOIS_SAISON[mois] ?? ''
  return {
    q: `En quelle saison est le mois de ${mois} ?`,
    answer: cap(saison),
    choices: SAISONS.filter(s => s !== saison).map(cap),
  }
}

// ---------- couleurs ----------
function otherColorNames(exclude: readonly string[], count: number): string[] {
  return shuffle(COULEURS.filter(c => !exclude.includes(c.name))).slice(0, count).map(c => cap(c.name))
}

function makeCouleursQuestion(): Question {
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
function otherShapeNames(exclude: readonly string[], count: number): string[] {
  return shuffle(FORMES.filter(f => !exclude.includes(f.name))).slice(0, count).map(f => cap(f.name))
}

function makeFormesQuestion(): Question {
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
    const distractors = new Set<string>()
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
    const name = COTES_UNIQUES[Number(n)] ?? ''
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
function digitDistractors(answer: number, min: number, max: number): string[] {
  const others = []
  for (let i = min; i <= max; i++) if (i !== answer) others.push(String(i))
  return shuffle(others).slice(0, 3)
}

// 3 noms de chiffres (en lettres) différents de n, parmi 0 à 9
function digitWordDistractors(n: number): string[] {
  const others = []
  for (let i = 0; i <= 9; i++) if (i !== n) others.push(cap(at(UNITES, i)))
  return shuffle(others).slice(0, 3)
}

function makeChiffresQuestion(): Question {
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
      answer: cap(at(UNITES, n)),
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
function nearbyNumbers(n: number, min: number, max: number): string[] {
  const candidates = shuffle([n - 10, n + 10, n - 1, n + 1, n - 2, n + 2, n - 11, n + 11])
    .filter(c => c >= min && c <= max && c !== n)
  const set = new Set(candidates.slice(0, 3).map(String))
  let extra = min
  while (set.size < 3) { if (extra !== n) set.add(String(extra)); extra++ }
  return [...set]
}

function makeCinquanteQuestion(): Question {
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
export function numberToWords(n: number): string {
  if (n < 20) return at(UNITES, n)
  if (n === 100) return 'cent'
  let dizaine = Math.floor(n / 10) * 10
  let unite = n - dizaine
  // 70–79 et 90–99 se disent "soixante-dix…" et "quatre-vingt-dix…"
  if (dizaine === 70 || dizaine === 90) { dizaine -= 10; unite += 10 }
  if (unite === 0) return dizaine === 80 ? 'quatre-vingts' : (DIZAINES[dizaine] ?? '')
  if ((unite === 1 || unite === 11) && dizaine !== 80) return `${DIZAINES[dizaine]} et ${UNITES[unite]}`
  return `${DIZAINES[dizaine]}-${UNITES[unite]}`
}

function makeNombresQuestion(level: number): Question {
  const [min, max] = at([[0, 16], [17, 69], [60, 100]] as const, level - 1)
  const n = rand(min, max)
  // 3 nombres proches mais différents, dans la plage du niveau
  const distractors = new Set<number>()
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
      choices: nums.map(x => numberToWords(x)),
    }
  }
  return {
    q: `Quel nombre s'écrit « ${numberToWords(n)} » ?`,
    answer: String(n),
    choices: nums.map(String),
  }
}


// ---------- lire l'heure sur une horloge à aiguilles ----------
// minutes proposées selon le niveau : heures pleines, puis demies, puis quarts
const HEURE_MINUTES = [
  [0],                                          // 🌱 heures pleines
  [0, 15, 30, 45],                              // 🌿 quarts et demies
  [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55], // 🌳 toutes les 5 minutes
  Array.from({ length: 60 }, (_, i) => i),        // 🏆 minute par minute
]

// « une heure », « trois heures et quart », « quatre heures moins le quart »…
// Les minutes du niveau demandé, bornées : un niveau inattendu retombe sur le plus fin.
function heureMinutes(level: number): number[] {
  return at(HEURE_MINUTES, Math.min(Math.max(level, 1), HEURE_MINUTES.length) - 1)
}

export function timeToWords(h: number, m: number): string {
  const nom = (n: number) => (n === 1 ? 'une heure' : `${numberToWords(n)} heures`)
  // la minute est féminine : « onze heures une », « deux heures trente et une »
  const min = (n: number) => numberToWords(n).replace(/\bun$/, 'une')
  const suivante = h === 12 ? 1 : h + 1
  if (m === 0) return nom(h)
  if (m === 15) return `${nom(h)} et quart`
  if (m === 30) return `${nom(h)} et demie`
  if (m === 45) return `${nom(suivante)} moins le quart`
  // passé la demie, on annonce l'heure suivante : 2 h 40 → « trois heures moins vingt ».
  // Réservé aux minutes rondes : on ne dit pas « moins vingt-neuf ».
  if (m > 30 && (60 - m) % 5 === 0) return `${nom(suivante)} moins ${min(60 - m)}`
  return `${nom(h)} ${min(m)}`
}

// 3 autres horaires du même niveau, formulés en toutes lettres
function otherTimes(h: number, m: number, level: number, count: number): string[] {
  const minutes = heureMinutes(level)
  const answer = timeToWords(h, m)
  const set = new Set<string>()
  let guard = 0
  while (set.size < count && guard < 200) {
    guard++
    // on privilégie les pièges : même heure minutes différentes, ou heure voisine
    const hh = rand(0, 2) === 0 ? h : rand(1, 12)
    const mm = pick(minutes)
    const t = timeToWords(hh, mm)
    if (t !== answer) set.add(t)
  }
  let extra = 1
  while (set.size < count) {
    const t = timeToWords(extra, at(minutes, 0))
    if (t !== answer) set.add(t)
    extra++
  }
  return [...set]
}

// questions sur le rôle des aiguilles, indépendantes du niveau
const HEURE_NOTIONS = [
  { q: 'Quelle aiguille indique les heures ?', answer: 'La petite',
    choices: ['La grande', 'Les deux', 'Aucune'] },
  { q: 'Quelle aiguille indique les minutes ?', answer: 'La grande',
    choices: ['La petite', 'Les deux', 'Aucune'] },
  { q: 'Combien y a-t-il de minutes dans une heure ?', answer: '60',
    choices: ['12', '30', '100'] },
  { q: "Combien y a-t-il d'heures sur le cadran d'une horloge ?", answer: '12',
    choices: ['10', '24', '60'] },
  { q: 'Quand la grande aiguille est sur le 6, il est…', answer: 'et demie',
    choices: ['et quart', 'moins le quart', 'pile'] },
  { q: 'Quand la grande aiguille est sur le 3, il est…', answer: 'et quart',
    choices: ['et demie', 'moins le quart', 'pile'] },
  { q: 'Quand la grande aiguille est sur le 12, il est…', answer: 'pile',
    choices: ['et quart', 'et demie', 'moins le quart'] },
  { q: 'Combien de minutes séparent deux chiffres du cadran ?', answer: '5',
    choices: ['1', '10', '12'] },
  { q: "Sur quel chiffre est la grande aiguille à « et quart » ?", answer: '3',
    choices: ['6', '9', '12'] },
  { q: "Sur quel chiffre est la grande aiguille à « et demie » ?", answer: '6',
    choices: ['3', '9', '12'] },
  { q: "Sur quel chiffre est la grande aiguille à « moins le quart » ?", answer: '9',
    choices: ['3', '6', '12'] },
]

function makeHeureQuestion(level: number): Question {
  // lire l'horloge revient le plus souvent ; on ajoute les notions et une projection
  const type = pick(['lire', 'lire', 'lire', 'notion', 'plus_tard'])

  if (type === 'notion') {
    const n = pick(HEURE_NOTIONS)
    return { q: n.q, answer: n.answer, choices: n.choices }
  }

  const minutes = heureMinutes(level)
  const h = rand(1, 12)
  const m = pick(minutes)

  if (type === 'plus_tard') {
    const suivante = h === 12 ? 1 : h + 1
    return {
      q: "Dans une heure, quelle heure sera-t-il ?",
      key: `heure+1:${h}:${m}`,
      clock: { hours: h, minutes: m },
      answer: timeToWords(suivante, m),
      choices: otherTimes(suivante, m, level, 3),
    }
  }

  return {
    q: 'Quelle heure est-il ?',
    key: `heure:${h}:${m}`, // le texte est identique : on déduplique sur l'horaire
    clock: { hours: h, minutes: m },
    answer: timeToWords(h, m),
    choices: otherTimes(h, m, level, 3),
  }
}

// ---------- calcul ----------
// 3 mauvaises réponses proches du résultat, uniques et positives
function numberDistractors(answer: number, count: number): string[] {
  const set = new Set<number>()
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
  return [...set].map(String)
}

function makeMathQuestion(op: string, level: number): Question {
  let a, b, result, symbol

  if (op === 'addition') {
    const max = at([10, 20, 100] as const, level - 1)
    a = rand(1, max); b = rand(1, max)
    result = a + b; symbol = '+'
  } else if (op === 'soustraction') {
    const max = at([10, 20, 100] as const, level - 1)
    a = rand(1, max); b = rand(1, max)
    if (b > a) [a, b] = [b, a] // jamais de résultat négatif
    result = a - b; symbol = '−'
  } else if (op === 'multiplication') {
    const max = at([5, 10, 12] as const, level - 1)
    a = rand(1, max); b = rand(1, 10)
    result = a * b; symbol = '×'
  } else { // division exacte
    const max = at([5, 10, 12] as const, level - 1)
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
function makeQuestion(category: string, level: number): Question {
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
      display: (s: string) => SAISON_ARTICLE[s] ?? s,
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
  if (category === 'heure') return makeHeureQuestion(level)
  if (category === 'nombres') return makeNombresQuestion(level)
  if (category === 'planifier') return makePlanifierQuestion(level)
  return makeMathQuestion(category, level)
}


// ─── Planifier : génération des questions ────────────────────────────────────

/** Autres étapes de la même activité, comme leurres plausibles. */
function autresEtapes(routine: Routine, exclure: readonly string[], nb: number): string[] {
  const restantes = routine.etapes.filter(e => !exclure.includes(e))
  return shuffle(restantes).slice(0, nb)
}

/** Étapes puisées dans d'autres activités (pour l'intrus et les compléments). */
function etapesAilleurs(routine: Routine, nb: number): string[] {
  const ailleurs = ROUTINES.filter(r => r.key !== routine.key).flatMap(r => r.etapes)
  return shuffle(ailleurs).slice(0, nb)
}

/** Complète une liste de leurres jusqu'à `nb`, sans doublon avec la réponse. */
function completeChoix(choix: string[], reponse: string, routine: Routine, nb: number): string[] {
  const vus = new Set([reponse, ...choix])
  for (const e of etapesAilleurs(routine, 12)) {
    if (choix.length >= nb) break
    if (!vus.has(e)) { choix.push(e); vus.add(e) }
  }
  return choix.slice(0, nb)
}

// 🌱 Les étapes : repérer le début, la fin, et ce qui suit
function planEtapes(): Question {
  const r = pick(ROUTINES)
  const type = pick(['premiere', 'derniere', 'apres'])

  if (type === 'premiere') {
    const reponse = at(r.etapes, 0)
    return {
      q: `« ${r.label} » : par quoi commence-t-on ?`,
      key: `plan:premiere:${r.key}`,
      routine: r,
      answer: reponse,
      choices: completeChoix(autresEtapes(r, [reponse], 3), reponse, r, 3),
    }
  }
  if (type === 'derniere') {
    const reponse = at(r.etapes, r.etapes.length - 1)
    return {
      q: `« ${r.label} » : quelle est la dernière étape ?`,
      key: `plan:derniere:${r.key}`,
      routine: r,
      answer: reponse,
      choices: completeChoix(autresEtapes(r, [reponse], 3), reponse, r, 3),
    }
  }
  // « juste après » : on tire une étape qui en a une suivante
  const i = rand(0, r.etapes.length - 2)
  const reponse = at(r.etapes, i + 1)
  return {
    q: `« ${r.label} » : que fait-on juste après « ${at(r.etapes, i)} » ?`,
    key: `plan:apres:${r.key}:${i}`,
    routine: r,
    answer: reponse,
    choices: completeChoix(autresEtapes(r, [reponse, at(r.etapes, i)], 3), reponse, r, 3),
  }
}

// 🌿 L'ordre : l'étape qui manque, l'intrus, ce qui vient avant
function planOrdre(): Question {
  const r = pick(ROUTINES)
  const type = pick(['manquante', 'intrus', 'avant'])

  if (type === 'manquante') {
    const i = rand(0, r.etapes.length - 1)
    const reponse = at(r.etapes, i)
    const restantes = r.etapes.filter((_, j) => j !== i)
    return {
      q: `« ${r.label} » : quelle étape manque ?`,
      liste: restantes, // affichée dans l'ordre, avec un trou
      trou: i,
      key: `plan:manquante:${r.key}:${i}`,
      routine: r,
      answer: reponse,
      choices: completeChoix([], reponse, r, 3),
    }
  }
  if (type === 'intrus') {
    const intrus = at(etapesAilleurs(r, 1), 0)
    const gardees = shuffle(r.etapes).slice(0, 3)
    return {
      q: `« ${r.label} » : quelle étape n'en fait pas partie ?`,
      liste: shuffle<string>([...gardees, intrus]),
      key: `plan:intrus:${r.key}:${intrus}`,
      routine: r,
      answer: intrus,
      choices: gardees,
    }
  }
  const i = rand(1, r.etapes.length - 1)
  const reponse = at(r.etapes, i - 1)
  return {
    q: `« ${r.label} » : que fait-on juste avant « ${at(r.etapes, i)} » ?`,
    key: `plan:avant:${r.key}:${i}`,
    routine: r,
    answer: reponse,
    choices: completeChoix(autresEtapes(r, [reponse, at(r.etapes, i)], 3), reponse, r, 3),
  }
}

// 🌳 Le temps : estimer une durée, calculer une fin, tenir dans un créneau
function planTemps(): Question {
  const r = pick(ROUTINES)
  const type = pick(['duree', 'fin', 'tient', 'pluslongue'])

  if (type === 'duree') {
    const autres = shuffle([...new Set(ROUTINES.map(x => x.duree))].filter(d => d !== r.duree))
    return {
      q: `Combien de temps prend « ${r.label} » à peu près ?`,
      key: `plan:duree:${r.key}`,
      routine: r,
      answer: `${r.duree} min`,
      choices: autres.slice(0, 3).map(d => `${d} min`),
    }
  }
  if (type === 'fin') {
    const h = rand(8, 17)
    const m = pick([0, 15, 30, 45])
    const [fh, fm] = ajouteMinutes(h, m, r.duree)
    const leurres = new Set()
    for (const ecart of shuffle([-30, -15, -10, 10, 15, 30, 60])) {
      if (leurres.size >= 3) break
      const [lh, lm] = ajouteMinutes(fh, fm, ecart)
      leurres.add(formatHeure(lh, lm))
    }
    return {
      q: `Tu commences « ${r.label} » à ${formatHeure(h, m)}. `
        + `Ça prend ${r.duree} min. À quelle heure as-tu fini ?`,
      key: `plan:fin:${r.key}:${h}:${m}`,
      routine: r,
      answer: formatHeure(fh, fm),
      choices: [...leurres].slice(0, 3) as string[],
    }
  }
  if (type === 'tient') {
    // une seule activité tient dans le créneau, les trois autres sont trop longues
    const creneau = pick([5, 10, 15, 20])
    const tiennent = ROUTINES.filter(x => x.duree <= creneau)
    const trop = ROUTINES.filter(x => x.duree > creneau)
    if (tiennent.length && trop.length >= 3) {
      const bonne = pick(tiennent)
      return {
        q: `Tu as ${creneau} minutes devant toi. Qu'est-ce qui rentre ?`,
        key: `plan:tient:${creneau}:${bonne.key}`,
        routine: bonne,
        answer: bonne.label,
        choices: shuffle(trop).slice(0, 3).map(x => x.label),
      }
    }
  }
  // la plus longue de quatre activités
  const quatre = shuffle(ROUTINES).slice(0, 4)
  const plusLongue = quatre.reduce((a, b) => (b.duree > a.duree ? b : a))
  if (quatre.filter(x => x.duree === plusLongue.duree).length > 1) return planTemps()
  return {
    q: 'Laquelle de ces activités prend le plus de temps ?',
    key: `plan:pluslongue:${quatre.map(x => x.key).sort().join('-')}`,
    routine: plusLongue,
    answer: plusLongue.label,
    choices: quatre.filter(x => x.key !== plusLongue.key).map(x => x.label),
  }
}

function makePlanifierQuestion(level: number): Question {
  if (level === 1) return planEtapes()
  if (level === 2) return planOrdre()
  return planTemps()
}

// thèmes puisés par le mode Mélange (toutes les catégories réelles, sauf le mélange lui-même)
const MELANGE_SOURCES: CategoryKey[] = ['chiffres', 'cinquante', 'nombres', 'addition', 'soustraction',
  'multiplication', 'division', 'jours', 'mois', 'saisons', 'alphabet', 'couleurs', 'formes',
  'heure', 'planifier']

// Entrelacement : on parcourt les thèmes en rotation pour que deux questions
// voisines viennent presque toujours de thèmes différents.
function buildInterleaved(): Question[] {
  const sources = shuffle(MELANGE_SOURCES)
  const qs = []
  const seen = new Set()
  let i = 0, guard = 0
  while (qs.length < NB_QUESTIONS && guard < 400) {
    guard++
    const cat = at(sources, i % sources.length)
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

export function buildQuestions(category: string, level: number): Question[] {
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

export function pickPraise(): string {
  return pick(['✅ Bravo !', '✅ Super !', '✅ Exact !', '✅ Génial !'])
}

export function endSummary(score: number, total: number): EndSummary {
  const ratio = score / total
  if (ratio === 1) return { stars: '⭐⭐⭐⭐⭐', msg: 'Score parfait, tu es un champion ! 🏆' }
  if (ratio >= 0.8) return { stars: '⭐⭐⭐⭐', msg: 'Excellent travail, continue comme ça ! 🎉' }
  if (ratio >= 0.6) return { stars: '⭐⭐⭐', msg: 'Bien joué ! Encore un petit effort ! 💪' }
  if (ratio >= 0.4) return { stars: '⭐⭐', msg: 'Pas mal ! Entraîne-toi encore un peu ! 🙂' }
  return { stars: '⭐', msg: "Courage, réessaie : c'est en s'entraînant qu'on apprend ! 🌱" }
}
