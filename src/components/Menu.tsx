import { CATEGORIES } from '../questions.ts'
import type { CategoryKey } from '../questions.ts'

type Props = { onSelect: (key: CategoryKey) => void }

export default function Menu({ onSelect }: Props) {
  return (
    <>
      <h1>🎓 ReNeuro</h1>
      <p className="subtitle">Choisis ce que tu veux apprendre !</p>
      <div className="menu-grid">
        {Object.entries(CATEGORIES).map(([key, cat]) => (
          <button key={key} className={`menu-btn c-${key}`} onClick={() => onSelect(key as CategoryKey)}>
            <span className="emoji">{cat.emoji}</span>
            {cat.label}
          </button>
        ))}
      </div>
      <p className="credits">Conçu et développé par Maxime Nathan Lestage</p>
    </>
  )
}
