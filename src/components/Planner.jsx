import { useEffect, useMemo, useState } from 'react'
import { ROUTINES, formatHeure, ajouteMinutes } from '../questions.js'

const CLE = 'reneuro-journee'
const DUREES = [5, 10, 15, 20, 30, 45, 60]

const aujourdhui = () => new Date().toISOString().slice(0, 10)

const dateLisible = () =>
  new Date().toLocaleDateString('fr-FR', { weekday: 'long', day: 'numeric', month: 'long' })

/** Journée vide, commencée à 9 h. */
const journeeVide = () => ({ date: aujourdhui(), debut: { h: 9, m: 0 }, taches: [] })

/**
 * On repart d'une journée vide quand la date a changé : planifier, c'est
 * planifier aujourd'hui. Un stockage illisible ne doit jamais bloquer l'écran.
 */
function chargeJournee() {
  try {
    const brut = localStorage.getItem(CLE)
    if (!brut) return journeeVide()
    const j = JSON.parse(brut)
    if (j?.date !== aujourdhui() || !Array.isArray(j.taches)) return journeeVide()
    return { ...journeeVide(), ...j }
  } catch {
    return journeeVide()
  }
}

export default function Planner({ onBack }) {
  const [journee, setJournee] = useState(chargeJournee)
  const [texte, setTexte] = useState('')
  const [duree, setDuree] = useState(15)
  const [ouverte, setOuverte] = useState(null) // tâche dont on déroule les étapes
  const [, setTic] = useState(0) // force le réaffichage du chrono en cours

  useEffect(() => {
    try { localStorage.setItem(CLE, JSON.stringify(journee)) } catch { /* stockage indisponible */ }
  }, [journee])

  // Le chrono d'une tâche en cours doit avancer sous les yeux : on ne fait
  // tourner l'horloge que s'il y a effectivement quelque chose à afficher.
  const enCours = journee.taches.find(t => t.demarreA)
  useEffect(() => {
    if (!enCours) return
    const id = setInterval(() => setTic(n => n + 1), 1000)
    return () => clearInterval(id)
  }, [!!enCours])

  const { taches, debut } = journee
  const majTaches = fn => setJournee(j => ({ ...j, taches: fn(j.taches) }))

  // Chaque tâche démarre quand la précédente se termine : c'est ce calcul qui
  // rend le plan concret, et qui montre tout de suite si la journée déborde.
  const horaires = useMemo(() => {
    let h = debut.h, m = debut.m
    return taches.map(t => {
      const debutTache = formatHeure(h, m)
      ;[h, m] = ajouteMinutes(h, m, t.duree)
      return debutTache
    })
  }, [taches, debut])

  const total = taches.reduce((s, t) => s + t.duree, 0)
  // tâches dont on connaît le temps réellement passé
  const mesurees = taches.filter(t => t.reel != null)
  const prevuMesure = mesurees.reduce((s, t) => s + t.duree, 0)
  const reelMesure = mesurees.reduce((s, t) => s + t.reel, 0)
  const bilanEstimation = (() => {
    if (!mesurees.length) return ''
    const debut = `Sur ${mesurees.length} tâche${mesurees.length > 1 ? 's' : ''} chronométrée${mesurees.length > 1 ? 's' : ''} : `
      + `${prevuMesure} min prévues, ${reelMesure} min réelles`
    const ecart = reelMesure - prevuMesure
    if (Math.abs(ecart) <= Math.max(3, prevuMesure * 0.15)) {
      return `${debut} — tes estimations sont justes ! 🎯`
    }
    if (ecart > 0) return `${debut} — tu as tendance à sous-estimer, c'est très courant. 🌱`
    return `${debut} — tu prends moins de temps que prévu.`
  })()
  const faites = taches.filter(t => t.fait).length
  const [fh, fm] = ajouteMinutes(debut.h, debut.m, total)

  const ajoute = (label, dureeTache, routine = null) => {
    const propre = label.trim()
    if (!propre) return
    majTaches(ts => [...ts, {
      id: `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
      label: propre, duree: dureeTache, fait: false, routine,
      demarreA: null, reel: null,
    }])
  }

  const ajouteEcrite = () => { ajoute(texte, duree); setTexte('') }

  /** Minutes écoulées depuis le démarrage, arrondies à la minute la plus proche. */
  const ecoulees = t => Math.max(0, Math.round((Date.now() - t.demarreA) / 60000))

  /**
   * Démarre une tâche — et arrête celle qui tournait : on fait une chose à la
   * fois, c'est justement ce qu'on apprend ici.
   */
  const demarre = id => majTaches(ts => ts.map(t => {
    if (t.id === id) return { ...t, demarreA: Date.now() }
    return t.demarreA ? { ...t, demarreA: null, reel: ecoulees(t) } : t
  }))

  const arrete = id => majTaches(ts => ts.map(t => (
    t.id === id && t.demarreA ? { ...t, demarreA: null, reel: ecoulees(t) } : t
  )))

  // Cocher une tâche en cours arrête aussi son chrono et enregistre le temps mis.
  const bascule = id => majTaches(ts => ts.map(t => {
    if (t.id !== id) return t
    if (!t.fait && t.demarreA) return { ...t, fait: true, demarreA: null, reel: ecoulees(t) }
    return { ...t, fait: !t.fait }
  }))
  const supprime = id => majTaches(ts => ts.filter(t => t.id !== id))

  const deplace = (i, sens) => majTaches(ts => {
    const j = i + sens
    if (j < 0 || j >= ts.length) return ts
    const copie = ts.slice()
    ;[copie[i], copie[j]] = [copie[j], copie[i]]
    return copie
  })

  /**
   * Vert quand l'estimation était juste (à 20 % près), orange sinon. On ne
   * reproche jamais un dépassement : mal estimer est précisément ce qu'on
   * travaille, et le voir suffit à progresser.
   */
  const ecartClasse = t => {
    const marge = Math.max(2, t.duree * 0.2)
    return Math.abs(t.reel - t.duree) <= marge ? 'plan-juste' : 'plan-ecart'
  }

  const changeDebut = e => {
    const [h, m] = e.target.value.split(':').map(Number)
    setJournee(j => ({ ...j, debut: { h, m } }))
  }

  const videJournee = () => {
    if (taches.length && !confirm('Effacer toutes les tâches de la journée ?')) return
    setJournee(journeeVide())
  }

  return (
    <>
      <h1>🗓️ Ma journée</h1>
      <p className="subtitle">{dateLisible()}</p>

      <label className="plan-debut">
        Je commence à
        <input
          type="time"
          value={`${String(debut.h).padStart(2, '0')}:${String(debut.m).padStart(2, '0')}`}
          onChange={changeDebut}
        />
      </label>

      {taches.length === 0 ? (
        <p className="plan-vide">
          Aucune tâche pour l'instant.<br />
          Ajoute ce que tu veux faire aujourd'hui, dans l'ordre.
        </p>
      ) : (
        <ul className="plan-liste">
          {taches.map((t, i) => {
            const routine = ROUTINES.find(r => r.key === t.routine)
            return (
              <li key={t.id} className={t.fait ? 'plan-tache faite' : 'plan-tache'}>
                <div className="plan-ligne">
                  <span className="plan-heure">{horaires[i]}</span>
                  <button
                    className="plan-case"
                    onClick={() => bascule(t.id)}
                    aria-label={t.fait ? 'Marquer comme à faire' : 'Marquer comme faite'}
                  >
                    {t.fait ? '☑' : '☐'}
                  </button>
                  <span className="plan-label">{t.label}</span>
                  <span className="plan-duree">
                    {t.demarreA ? (
                      <span className="plan-chrono">⏱ {ecoulees(t)} min</span>
                    ) : t.reel != null ? (
                      <span className={ecartClasse(t)}>{t.duree} → {t.reel} min</span>
                    ) : (
                      `${t.duree} min`
                    )}
                  </span>
                </div>
                <div className="plan-actions">
                  {!t.fait && (t.demarreA
                    ? <button className="plan-mini en-cours" onClick={() => arrete(t.id)}>■ arrêter</button>
                    : <button className="plan-mini" onClick={() => demarre(t.id)}>▶ démarrer</button>
                  )}
                  {routine && (
                    <button
                      className="plan-mini"
                      onClick={() => setOuverte(ouverte === t.id ? null : t.id)}
                    >
                      {ouverte === t.id ? '▾ étapes' : '▸ étapes'}
                    </button>
                  )}
                  <button className="plan-mini" onClick={() => deplace(i, -1)} disabled={i === 0}>↑</button>
                  <button
                    className="plan-mini"
                    onClick={() => deplace(i, 1)}
                    disabled={i === taches.length - 1}
                  >↓</button>
                  <button className="plan-mini danger" onClick={() => supprime(t.id)}>✕</button>
                </div>
                {routine && ouverte === t.id && (
                  <ol className="plan-etapes">
                    {routine.etapes.map(e => <li key={e}>{e}</li>)}
                  </ol>
                )}
              </li>
            )
          })}
        </ul>
      )}

      {taches.length > 0 && (
        <p className="plan-bilan">
          Fin prévue à <strong>{formatHeure(fh, fm)}</strong> · {total} min au total
          <br />
          <span className={faites === taches.length ? 'plan-fini' : ''}>
            {faites} faite{faites > 1 ? 's' : ''} sur {taches.length}
            {faites === taches.length && ' — journée terminée ! 🎉'}
          </span>
          {mesurees.length > 0 && (
            <>
              <br />
              <span className="plan-estimation">{bilanEstimation}</span>
            </>
          )}
        </p>
      )}

      <div className="plan-ajout">
        <input
          type="text"
          value={texte}
          placeholder="Qu'est-ce que tu veux faire ?"
          onChange={e => setTexte(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && ajouteEcrite()}
        />
        <div className="plan-durees">
          {DUREES.map(d => (
            <button
              key={d}
              className={d === duree ? 'plan-duree-btn on' : 'plan-duree-btn'}
              onClick={() => setDuree(d)}
            >{d} min</button>
          ))}
        </div>
        <button className="big-btn" onClick={ajouteEcrite} disabled={!texte.trim()}>
          ➕ Ajouter à ma journée
        </button>
      </div>

      <p className="plan-titre-connues">ou une activité que tu connais déjà :</p>
      <div className="plan-connues">
        {ROUTINES.map(r => (
          <button key={r.key} className="plan-connue" onClick={() => ajoute(r.label, r.duree, r.key)}>
            <span className="emoji">{r.emoji}</span>
            {r.label}
            <span className="plan-connue-duree">{r.duree} min</span>
          </button>
        ))}
      </div>

      {taches.length > 0 && (
        <button className="big-btn secondary" onClick={videJournee}>🗑️ Vider la journée</button>
      )}
      <button className="back-link" onClick={onBack}>← Retour au menu</button>
    </>
  )
}
