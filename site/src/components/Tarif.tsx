import { OFFRE } from '../donnees.ts'

export default function Tarif() {
  return (
    <section id="tarif" className="section">
      <h2>Le tarif</h2>
      <div className="carte-tarif">
        <p className="tarif-prix">
          <strong>{OFFRE.prix}</strong>
          <span className="tarif-periode">{OFFRE.periode}</span>
        </p>
        <ul className="tarif-inclus">
          {OFFRE.inclus.map(ligne => (
            <li key={ligne}>{ligne}</li>
          ))}
        </ul>
        <p className="tarif-dispo">{OFFRE.disponibilite}</p>
      </div>
    </section>
  )
}
