import { useEffect, useMemo, useRef, useState } from 'react'
import { buildQuestions, pickPraise, NB_QUESTIONS } from '../questions.js'

export default function Quiz({ category, level, onFinish, onQuit }) {
  const questions = useMemo(() => buildQuestions(category, level), [category, level])
  const [index, setIndex] = useState(0)
  const [score, setScore] = useState(0)
  const [selected, setSelected] = useState(null) // option choisie, null = pas encore répondu
  const [feedback, setFeedback] = useState(null)
  const timerRef = useRef(null)

  useEffect(() => () => clearTimeout(timerRef.current), [])

  const question = questions[index]

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
    }

    timerRef.current = setTimeout(() => {
      if (index + 1 < questions.length) {
        setIndex(index + 1)
        setSelected(null)
        setFeedback(null)
      } else {
        onFinish(nextScore, questions.length)
      }
    }, good ? 900 : 1800)
  }

  const answerClass = (option) => {
    if (selected === null) return 'answer-btn'
    if (option === question.answer) return 'answer-btn correct'
    if (option === selected) return 'answer-btn wrong'
    return 'answer-btn'
  }

  return (
    <>
      <div className="progress-bar">
        <div className="progress-fill" style={{ width: `${(index / NB_QUESTIONS) * 100}%` }} />
      </div>
      <div className="quiz-info">
        <span>Question {index + 1} / {NB_QUESTIONS}</span>
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
