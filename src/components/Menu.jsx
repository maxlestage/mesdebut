import { CATEGORIES } from '../questions.js'

export default function Menu({ onSelect }) {
  return (
    <>
      <h1>🎓 Mes Débuts</h1>
      <p className="subtitle">Choisis ce que tu veux apprendre !</p>
      <div className="menu-grid">
        {Object.entries(CATEGORIES).map(([key, cat]) => (
          <button key={key} className={`menu-btn c-${key}`} onClick={() => onSelect(key)}>
            <span className="emoji">{cat.emoji}</span>
            {cat.label}
          </button>
        ))}
      </div>
    </>
  )
}
