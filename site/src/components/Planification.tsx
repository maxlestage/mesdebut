export default function Planification() {
  return (
    <section className="section section-mise-en-avant">
      <span className="badge">📋 Planifier</span>
      <h2>Organiser sa journée, pas seulement la raconter</h2>
      <p>
        Onze activités du quotidien sont décomposées en étapes&nbsp;: se faire un café, faire une
        lessive, prendre le bus. On apprend à les remettre dans l'ordre, à repérer l'étape qui
        manque, à estimer le temps qu'elles prennent.
      </p>
      <p>
        Puis on passe à sa vraie journée. <strong>Ma journée</strong> laisse écrire ses tâches, les
        ordonner, les cocher. Chacune démarre quand la précédente finit, donc l'heure de chaque
        tâche s'affiche&nbsp;: on voit tout de suite quand la journée déborde.
      </p>
      <div className="exemple">
        <div className="exemple-ligne">
          <span className="exemple-heure">9 h 00</span>
          <span className="exemple-tache fait">Prendre son médicament</span>
          <span className="exemple-duree juste">2 → 2 min</span>
        </div>
        <div className="exemple-ligne">
          <span className="exemple-heure">9 h 02</span>
          <span className="exemple-tache">Se faire un café</span>
          <span className="exemple-duree ecart">5 → 9 min</span>
        </div>
        <div className="exemple-ligne">
          <span className="exemple-heure">9 h 11</span>
          <span className="exemple-tache">Faire les courses</span>
          <span className="exemple-duree">45 min</span>
        </div>
        <p className="exemple-bilan">
          Sur 5 jours&nbsp;: +18&nbsp;% d'écart en moyenne (contre +45&nbsp;% avant) — tu progresses&nbsp;! 🌱
        </p>
      </div>
      <p>
        Un chronomètre compare le temps prévu au temps réellement passé, et un suivi sur dix jours
        montre si l'écart se resserre. C'est cet écart-là qu'on travaille.
      </p>
    </section>
  )
}
