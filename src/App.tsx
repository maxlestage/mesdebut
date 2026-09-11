import { useState } from 'react'
import Menu from './components/Menu.tsx'
import Category from './components/Category.tsx'
import Learn from './components/Learn.tsx'
import Levels from './components/Levels.tsx'
import Quiz from './components/Quiz.tsx'
import End from './components/End.tsx'
import Planner from './components/Planner.tsx'
import { CATEGORIES } from './questions.ts'
// le composant Category porte déjà ce nom : on renomme le type à l'import
import type { Category as MetaCategorie, CategoryKey } from './questions.ts'

/** Les écrans de l'application, dans l'ordre d'un parcours type. */
type Screen = 'menu' | 'category' | 'learn' | 'levels' | 'quiz' | 'planner' | 'end'

type Resultat = { score: number; total: number }

export default function App() {
  const [screen, setScreen] = useState<Screen>('menu')
  const [category, setCategory] = useState<CategoryKey>('melange')
  const [level, setLevel] = useState(1)
  const [result, setResult] = useState<Resultat>({ score: 0, total: 0 })
  // change de clé à chaque partie pour remonter un Quiz tout neuf
  const [quizKey, setQuizKey] = useState(0)

  const goMenu = () => setScreen('menu')

  const openCategory = (cat: CategoryKey) => {
    setCategory(cat)
    const c: MetaCategorie = CATEGORIES[cat]
    if (c.hasLearn) setScreen('category')
    else if (c.hasLevels) setScreen('levels')
    else startQuiz() // ni révision ni niveau (ex. Mélange) → quiz direct
  }

  // depuis la révision ou l'écran de catégorie : passer par les niveaux si la
  // catégorie en a, sinon lancer directement le quiz
  const goToQuiz = () => {
    const courante: MetaCategorie = CATEGORIES[category]
    if (courante.hasLevels) setScreen('levels')
    else startQuiz()
  }

  const startQuiz = (lvl = level) => {
    setLevel(lvl)
    setQuizKey(k => k + 1)
    setScreen('quiz')
  }

  const finishQuiz = (score: number, total: number) => {
    setResult({ score, total })
    setScreen('end')
  }

  return (
    <div className="app">
      <div className="card">
        {screen === 'menu' && <Menu onSelect={openCategory} />}
        {screen === 'category' && (
          <Category
            category={category}
            onLearn={() => setScreen('learn')}
            onQuiz={goToQuiz}
            onPlan={category === 'planifier' ? () => setScreen('planner') : null}
            onBack={goMenu}
          />
        )}
        {screen === 'learn' && (
          <Learn category={category} onQuiz={goToQuiz} onBack={goMenu} />
        )}
        {screen === 'levels' && (
          <Levels category={category} onSelect={startQuiz} onBack={goMenu} />
        )}
        {screen === 'quiz' && (
          <Quiz
            key={quizKey}
            category={category}
            level={level}
            onFinish={finishQuiz}
            onQuit={goMenu}
          />
        )}
        {screen === 'planner' && <Planner onBack={goMenu} />}
        {screen === 'end' && (
          <End result={result} onReplay={() => startQuiz()} onMenu={goMenu} />
        )}
      </div>
    </div>
  )
}
