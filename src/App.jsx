import { useState } from 'react'
import Menu from './components/Menu.jsx'
import Category from './components/Category.jsx'
import Learn from './components/Learn.jsx'
import Levels from './components/Levels.jsx'
import Quiz from './components/Quiz.jsx'
import End from './components/End.jsx'
import { CATEGORIES } from './questions.js'

export default function App() {
  const [screen, setScreen] = useState('menu')
  const [category, setCategory] = useState(null)
  const [level, setLevel] = useState(1)
  const [result, setResult] = useState(null)
  // change de clé à chaque partie pour remonter un Quiz tout neuf
  const [quizKey, setQuizKey] = useState(0)

  const goMenu = () => setScreen('menu')

  const openCategory = (cat) => {
    setCategory(cat)
    setScreen(CATEGORIES[cat].hasLearn ? 'category' : 'levels')
  }

  // depuis la révision ou l'écran de catégorie : passer par les niveaux si la
  // catégorie en a, sinon lancer directement le quiz
  const goToQuiz = () => {
    if (CATEGORIES[category].hasLevels) setScreen('levels')
    else startQuiz()
  }

  const startQuiz = (lvl = level) => {
    setLevel(lvl)
    setQuizKey(k => k + 1)
    setScreen('quiz')
  }

  const finishQuiz = (score, total) => {
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
        {screen === 'end' && (
          <End result={result} onReplay={() => startQuiz()} onMenu={goMenu} />
        )}
      </div>
    </div>
  )
}
