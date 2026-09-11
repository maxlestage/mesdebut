import { CATEGORIES } from '../donnees.ts'

export default function Categories() {
  const avecNiveaux = CATEGORIES.filter(c => c.niveaux)

  return (
    <section id="themes" className="section">
      <h2>Seize thèmes</h2>
      <p className="intro">
        Chaque thème se révise d'abord, puis se travaille en quiz de dix questions.
      </p>

      <ul className="grille">
        {CATEGORIES.map(c => (
          <li key={c.key}>
            <article
              className="carte-theme"
              style={{ background: `linear-gradient(135deg, ${c.degrade[0]}, ${c.degrade[1]})` }}
            >
              <span className="emoji" aria-hidden="true">{c.emoji}</span>
              <span className="nom">{c.label}</span>
            </article>
          </li>
        ))}
      </ul>

      <p className="note">
        {avecNiveaux.length} thèmes ont plusieurs niveaux. L'heure en a quatre, des heures pleines
        à la minute près — soit 720 horaires différents à lire sur une horloge à aiguilles.
      </p>
    </section>
  )
}
