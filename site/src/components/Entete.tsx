export default function Entete() {
  return (
    <header className="entete">
      <img className="logo" src="./icon.svg" alt="" width={96} height={96} />
      <h1>ReNeuro</h1>
      <p className="accroche">Réapprendre les bases, en s'amusant.</p>
      <p className="sous-accroche">
        Les jours, l'heure, le calcul, les couleurs, organiser sa journée&nbsp;: seize thèmes,
        en questions courtes. Sur le web, l'iPhone et l'Apple&nbsp;Watch.
      </p>
      <div className="boutons">
        {/* Le site occupe la racine ; l'application est servie sous /app. */}
        <a className="bouton" href="/app/">Ouvrir l'application</a>
        <a className="bouton bouton-clair" href="#themes">Voir les thèmes</a>
      </div>
    </header>
  )
}
