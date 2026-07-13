import { endSummary } from '../questions.js'

export default function End({ result, onReplay, onMenu }) {
  const { stars, msg } = endSummary(result.score, result.total)
  return (
    <>
      <h1>Quiz terminé !</h1>
      <div className="stars">{stars}</div>
      <div className="final-score">{result.score} / {result.total} bonnes réponses</div>
      <p className="final-msg">{msg}</p>
      <div className="choice-list">
        <button className="big-btn" onClick={onReplay}>🔁 Rejouer</button>
        <button className="big-btn secondary" onClick={onMenu}>🏠 Menu principal</button>
      </div>
    </>
  )
}
