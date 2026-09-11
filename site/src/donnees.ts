// Contenu du site : les thèmes présentés, les supports, les principes.
//
// Cette liste reprend les catégories de l'application iOS (ios/), qui reste la
// seule implémentation. Les deux sont écrites séparément : en ajouter une ici
// suppose de la retrouver là-bas.

export type Categorie = {
  readonly key: string
  readonly emoji: string
  readonly label: string
  readonly title: string
  /** Dégradé repris de l'application, pour que les couleurs se correspondent. */
  readonly degrade: readonly [string, string]
  readonly niveaux?: readonly string[]
}

export const CATEGORIES: readonly Categorie[] = [
  { key: 'melange', emoji: '🧠', label: 'Mélange', title: 'Quiz mélangé', degrade: ['#26a69a', '#00695c'] },
  { key: 'jours', emoji: '📅', label: 'Les jours', title: 'Les jours de la semaine', degrade: ['#f6a821', '#f68f21'] },
  { key: 'mois', emoji: '🗓️', label: 'Les mois', title: "Les mois de l'année", degrade: ['#21b3f6', '#2179f6'] },
  { key: 'saisons', emoji: '🍂', label: 'Les saisons', title: 'Les saisons', degrade: ['#8bc34a', '#5a9216'] },
  { key: 'alphabet', emoji: '🔤', label: "L'alphabet", title: "L'alphabet", degrade: ['#26c6da', '#0097a7'] },
  {
    key: 'heure', emoji: '🕐', label: "L'heure", title: "Lire l'heure", degrade: ['#ab47bc', '#6a1b9a'],
    niveaux: ['Heures pleines', 'Quarts et demies', 'De 5 en 5', 'Minute par minute'],
  },
  {
    key: 'planifier', emoji: '📋', label: 'Planifier', title: 'Apprendre à planifier', degrade: ['#26c6da', '#00838f'],
    niveaux: ['Les étapes', "L'ordre", 'Le temps'],
  },
  { key: 'couleurs', emoji: '🎨', label: 'Les couleurs', title: 'Les couleurs', degrade: ['#ff8a65', '#d84315'] },
  { key: 'formes', emoji: '📐', label: 'Les formes', title: 'Les formes géométriques', degrade: ['#78909c', '#455a64'] },
  { key: 'chiffres', emoji: '🧮', label: 'Les chiffres', title: 'Les chiffres de 0 à 9', degrade: ['#ffca28', '#f57f17'] },
  { key: 'cinquante', emoji: '🔟', label: "Jusqu'à 50", title: "Les nombres jusqu'à 50", degrade: ['#ec407a', '#ad1457'] },
  {
    key: 'nombres', emoji: '🔢', label: 'Les nombres', title: 'Les nombres en lettres', degrade: ['#5c6bc0', '#3949ab'],
    niveaux: ['Facile', 'Moyen', 'Difficile'],
  },
  {
    key: 'addition', emoji: '➕', label: 'Addition', title: 'Addition', degrade: ['#4cd964', '#2eb350'],
    niveaux: ['Facile', 'Moyen', 'Difficile'],
  },
  {
    key: 'soustraction', emoji: '➖', label: 'Soustraction', title: 'Soustraction', degrade: ['#ff6b6b', '#e04545'],
    niveaux: ['Facile', 'Moyen', 'Difficile'],
  },
  {
    key: 'multiplication', emoji: '✖️', label: 'Multiplication', title: 'Multiplication', degrade: ['#a76bff', '#7d45e0'],
    niveaux: ['Facile', 'Moyen', 'Difficile'],
  },
  {
    key: 'division', emoji: '➗', label: 'Division', title: 'Division', degrade: ['#ff6bd5', '#e045a8'],
    niveaux: ['Facile', 'Moyen', 'Difficile'],
  },
]

export type Support = {
  readonly emoji: string
  readonly nom: string
  readonly detail: string
  readonly note: string
}

export const SUPPORTS: readonly Support[] = [
  {
    emoji: '🌐', nom: 'Web',
    detail: "Rien à installer : ça s'ouvre dans le navigateur.",
    note: "Installable à l'écran d'accueil, et ça marche sans connexion.",
  },
  {
    emoji: '📱', nom: 'iPhone',
    detail: 'Application native, écrite en Swift.',
    note: "Le quiz en cours s'affiche sur l'écran verrouillé et la Dynamic Island.",
  },
  {
    emoji: '⌚', nom: 'Apple Watch',
    detail: 'Les mêmes exercices, au poignet.',
    note: "Pratique pour l'heure : apprendre à lire une montre sur une montre.",
  },
]

export type Principe = {
  readonly titre: string
  readonly texte: string
}

export const PRINCIPES: readonly Principe[] = [
  {
    titre: 'On répond, on ne relit pas',
    texte: "Se tester ancre mieux que relire. Chaque thème est une suite de questions, pas une leçon.",
  },
  {
    titre: 'La réponse tout de suite',
    texte: "Juste ou faux, on le sait immédiatement, et la bonne réponse s'affiche.",
  },
  {
    titre: 'Ce qui est raté revient',
    texte: "Une question manquée revient 3 questions plus loin, puis 5 après. D'une session à l'autre, l'app se souvient de ce qui résiste.",
  },
  {
    titre: 'Les thèmes sont mélangés',
    texte: "Le mode Mélange alterne les sujets plutôt que de les réviser en blocs : il faut à chaque fois reconnaître de quoi il s'agit.",
  },
  {
    titre: 'Une image avec chaque mot',
    texte: 'Des billes à compter, des horloges à aiguilles, des formes et des couleurs dessinées.',
  },
  {
    titre: 'On encourage, on ne sanctionne pas',
    texte: 'Des étoiles, des félicitations, et jamais de reproche sur une erreur.',
  },
]

export type Offre = {
  readonly prix: string
  readonly periode: string
  readonly inclus: readonly string[]
  readonly disponibilite: string
}

export const OFFRE: Offre = {
  prix: '4,99 €',
  periode: 'par mois',
  inclus: [
    'Les seize thèmes, sans exception',
    'Le web, l\'iPhone et l\'Apple Watch',
    'La progression retenue d\'une séance à l\'autre',
    'Le planificateur de journée et son suivi',
    'Les mises à jour et les thèmes à venir',
  ],
  disponibilite: "Bientôt sur l'App Store",
}
