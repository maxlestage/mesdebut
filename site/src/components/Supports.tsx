import { SUPPORTS } from '../donnees.ts'

export default function Supports() {
  return (
    <section className="section">
      <h2>Sur deux écrans</h2>
      <p className="intro">Le même contenu, la même progression, de la poche au poignet.</p>
      <ul className="supports">
        {SUPPORTS.map(s => (
          <li key={s.nom}>
            <span className="support-emoji" aria-hidden="true">{s.emoji}</span>
            <div>
              <h3>{s.nom}</h3>
              <p>{s.detail}</p>
              <p className="support-note">{s.note}</p>
            </div>
          </li>
        ))}
      </ul>
    </section>
  )
}
