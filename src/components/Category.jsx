import { CATEGORIES } from '../questions.js'

export default function Category({ category, onLearn, onQuiz, onBack }) {
  const cat = CATEGORIES[category]
  return (
    <>
      <h1>{cat.emoji} {cat.title}</h1>
      <div className="choice-list">
        <button className="big-btn" onClick={onLearn}>📖 Réviser d'abord</button>
        <button className="big-btn" onClick={onQuiz}>🎯 Faire le quiz</button>
      </div>
      <button className="back-link" onClick={onBack}>← Retour au menu</button>
    </>
  )
}
