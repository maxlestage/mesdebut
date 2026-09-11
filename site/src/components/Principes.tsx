import { PRINCIPES } from '../donnees.ts'

export default function Principes() {
  return (
    <section className="section section-sombre">
      <h2>Comment c'est construit</h2>
      <p className="intro">
        Six principes issus des sciences cognitives, qui décident de la forme des exercices.
      </p>
      <ul className="principes">
        {PRINCIPES.map(p => (
          <li key={p.titre}>
            <h3>{p.titre}</h3>
            <p>{p.texte}</p>
          </li>
        ))}
      </ul>
    </section>
  )
}
