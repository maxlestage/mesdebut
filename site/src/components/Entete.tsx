import { OFFRE } from '../donnees.ts'

export default function Entete() {
  return (
    <header className="entete">
      <img className="logo" src="./icon.svg" alt="" width={96} height={96} />
      <h1>ReNeuro</h1>
      <p className="accroche">Réapprendre les bases, en s'amusant.</p>
      <p className="sous-accroche">
        Les jours, l'heure, le calcul, les couleurs, organiser sa journée&nbsp;: seize thèmes,
        en questions courtes. Sur l'iPhone et l'Apple&nbsp;Watch.
      </p>
      {/* Pas d'accès direct à l'application depuis le site : l'offre est payante.
          Le seul geste possible est de parcourir les thèmes, d'où le bouton plein. */}
      <p className="pastille-store">
        {OFFRE.disponibilite} · <strong>{OFFRE.prix} {OFFRE.periode}</strong>
      </p>
      <a className="bouton" href="#themes">Voir les thèmes</a>
    </header>
  )
}
