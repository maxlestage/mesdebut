import { JOURS, MOIS, cap } from '../questions.js'

export default function Learn({ category, onQuiz, onBack }) {
  const isJours = category === 'jours'
  const list = isJours ? JOURS : MOIS
  return (
    <>
      <h1>{isJours ? '📖 Les 7 jours de la semaine' : "📖 Les 12 mois de l'année"}</h1>
      <ul className="learn-list">
        {list.map((item, i) => (
          <li key={item}>
            <span className="num">{i + 1}</span>
            {cap(item)}
          </li>
        ))}
      </ul>
      <button className="big-btn" onClick={onQuiz}>🎯 Je suis prêt·e, quiz !</button>
      <button className="back-link" onClick={onBack}>← Retour au menu</button>
    </>
  )
}
