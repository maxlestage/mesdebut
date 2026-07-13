import { CATEGORIES, LEVELS } from '../questions.js'

export default function Levels({ category, onSelect, onBack }) {
  const cat = CATEGORIES[category]
  return (
    <>
      <h1>{cat.emoji} {cat.title}</h1>
      <p className="subtitle">Choisis ton niveau</p>
      <div className="choice-list">
        {LEVELS.map(lvl => (
          <button key={lvl.id} className="big-btn" onClick={() => onSelect(lvl.id)}>
            {lvl.emoji} {lvl.label}
          </button>
        ))}
      </div>
      <button className="back-link" onClick={onBack}>← Retour au menu</button>
    </>
  )
}
