import { CATEGORIES, levelsFor } from '../questions.ts'
import type { Category, CategoryKey } from '../questions.ts'

type Props = {
  category: CategoryKey
  onSelect: (level: number) => void
  onBack: () => void
}

export default function Levels({ category, onSelect, onBack }: Props) {
  const cat: Category = CATEGORIES[category]
  return (
    <>
      <h1>{cat.emoji} {cat.title}</h1>
      <p className="subtitle">Choisis ton niveau</p>
      <div className="choice-list">
        {levelsFor(category).map(lvl => (
          <button key={lvl.id} className="big-btn" onClick={() => onSelect(lvl.id)}>
            {lvl.emoji} {lvl.label}
          </button>
        ))}
      </div>
      <button className="back-link" onClick={onBack}>← Retour au menu</button>
    </>
  )
}
