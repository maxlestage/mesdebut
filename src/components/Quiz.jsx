import { useEffect, useRef, useState } from 'react'
import { buildQuestions, pickPraise, shuffle } from '../questions.js'
import Shape from './Shape.jsx'
import Marbles from './Marbles.jsx'

// identifiant stable d'une question (même clé que la déduplication)
const qid = (q) => q.key || q.q

export default function Quiz({ category, level, onFinish, onQuit }) {
  // la file de questions est dynamique : une question ratée y est ré-insérée
  const [questions, setQuestions] = useState(() => buildQuestions(category, level))
  const [index, setIndex] = useState(0)
  const [score, setScore] = useState(0)
  const [selected, setSelected] = useState(null) // option choisie, null = pas encore répondu
  const [feedback, setFeedback] = useState(null)
  const timerRef = useRef(null)
  // longueur à jour, lisible depuis le setTimeout (évite une valeur périmée)
  const questionsRef = useRef(questions)
  useEffect(() => { questionsRef.current = questions }, [questions])
  // questions ayant déjà programmé leurs reprises (une seule fois chacune)
  const scheduledRef = useRef(new Set())

  useEffect(() => () => clearTimeout(timerRef.current), [])

  const question = questions[index]

  // en cas d'erreur : la question revient 3 questions plus loin, puis encore 5 après
  const scheduleRetries = (q, atIndex) => {
    const id = qid(q)
    if (scheduledRef.current.has(id)) return // déjà reprogrammée : on évite l'empilement
    scheduledRef.current.add(id)
    setQuestions(prev => {
      const next = prev.slice()
      const makeCopy = () => ({ ...q, options: shuffle([q.answer, ...q.choices]) })
      // positions finales voulues : 3 questions plus loin, puis encore 5 après (donc +8).
      // On insère dans l'ordre croissant : la 1re insertion décale la 2e cible d'un cran,
      // ce qui fait justement retomber la reprise à 5 questions après la première.
      for (const t of [atIndex + 3, atIndex + 8]) {
        next.splice(Math.min(t, next.length), 0, makeCopy())
      }
      return next
    })
  }

  const selectAnswer = (option) => {
    if (selected !== null) return
    setSelected(option)

    const good = option === question.answer
    const nextScore = good ? score + 1 : score
    if (good) {
      setScore(nextScore)
      setFeedback({ good: true, text: pickPraise() })
    } else {
      setFeedback({ good: false, text: `❌ La bonne réponse était : ${question.answer}` })
      scheduleRetries(question, index) // on la reverra plus loin
    }

    timerRef.current = setTimeout(() => {
      const total = questionsRef.current.length
      if (index + 1 < total) {
        setIndex(index + 1)
        setSelected(null)
        setFeedback(null)
      } else {
        onFinish(nextScore, total)
      }
    }, good ? 900 : 1800)
  }

  const answerClass = (option) => {
    if (selected === null) return 'answer-btn'
    if (option === question.answer) return 'answer-btn correct'
    if (option === selected) return 'answer-btn wrong'
    return 'answer-btn'
  }

  const total = questions.length

  return (
    <>
      <div className="progress-bar">
        <div className="progress-fill" style={{ width: `${(index / total) * 100}%` }} />
      </div>
      <div className="quiz-info">
        <span>Question {index + 1} / {total}</span>
        <span>⭐ {score}</span>
      </div>
      <div className="question">
        <span>
          {question.q}
          {question.swatch && (
            <span
              className={`swatch${question.swatch === '#ffffff' ? ' swatch-light' : ''}`}
              style={{ background: question.swatch }}
            />
          )}
          {question.shape && (
            <span className="shape-figure">
              <Shape name={question.shape} size={80} />
            </span>
          )}
          {question.marbles != null && (
            <span className="shape-figure">
              <Marbles
                count={question.marbles}
                perRow={question.perRow || 5}
                colorByRow={!!question.colorByRow}
                size={question.marbles > 10 ? 22 : 28}
              />
            </span>
          )}
        </span>
      </div>
      <div className="answers">
        {question.options.map(option => (
          <button
            key={option}
            className={answerClass(option)}
            disabled={selected !== null}
            onClick={() => selectAnswer(option)}
          >
            {option}
          </button>
        ))}
      </div>
      <div className={`feedback ${feedback ? (feedback.good ? 'good' : 'bad') : ''}`}>
        {feedback?.text}
      </div>
      <button className="back-link" onClick={onQuit}>← Abandonner</button>
    </>
  )
}
