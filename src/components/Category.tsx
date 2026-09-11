import { CATEGORIES } from '../questions.ts'
import type { Category, CategoryKey } from '../questions.ts'

type Props = {
  category: CategoryKey
  onLearn: () => void
  onQuiz: () => void
  /** Troisième choix, propre à « Planifier » : le planificateur de journée. */
  onPlan?: (() => void) | null
  onBack: () => void
}

export default function Category({ category, onLearn, onQuiz, onPlan, onBack }: Props) {
  const cat: Category = CATEGORIES[category]
  return (
    <>
      <h1>{cat.emoji} {cat.title}</h1>
      <div className="choice-list">
        <button className="big-btn" onClick={onLearn}>📖 Réviser d'abord</button>
        <button className="big-btn" onClick={onQuiz}>🎯 Faire le quiz</button>
        {onPlan && <button className="big-btn" onClick={onPlan}>🗓️ Ma journée</button>}
      </div>
      <button className="back-link" onClick={onBack}>← Retour au menu</button>
    </>
  )
}
