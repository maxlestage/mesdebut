import { useState } from 'react'
import { LEARN_DATA } from '../questions.js'
import Shape from './Shape.jsx'
import Marbles from './Marbles.jsx'

export default function Learn({ category, onQuiz, onBack }) {
  const data = LEARN_DATA[category]
  // pour les accordéons : la première section est ouverte au départ
  const [open, setOpen] = useState(() => new Set(data.groups ? [0] : []))
  const toggle = (i) => setOpen(prev => {
    const next = new Set(prev)
    next.has(i) ? next.delete(i) : next.add(i)
    return next
  })

  return (
    <>
      <h1>{data.title}</h1>
      {data.intro && <p className="subtitle">{data.intro}</p>}

      {data.groups ? (
        <div className="accordion">
          {data.groups.map((g, gi) => {
            const isOpen = open.has(gi)
            return (
              <div key={g.label} className="accordion-group">
                <button
                  className="accordion-head"
                  onClick={() => toggle(gi)}
                  aria-expanded={isOpen}
                >
                  <span>{g.label}</span>
                  <span className="chevron">{isOpen ? '▾' : '▸'}</span>
                </button>
                {isOpen && (
                  <ul className="learn-list accordion-body">
                    {g.items.map(item => (
                      <li key={item.num}>
                        <span className="num">{item.num}</span>
                        <span>{item.label}</span>
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            )
          })}
        </div>
      ) : data.grid ? (
        <div className="letter-grid">
          {data.grid.map(letter => <span key={letter} className="letter">{letter}</span>)}
        </div>
      ) : (
        <ul className="learn-list">
          {data.items.map((item, i) => (
            <li key={item.label}>
              {item.color ? (
                <span
                  className={`color-dot${item.color === '#ffffff' ? ' swatch-light' : ''}`}
                  style={{ background: item.color }}
                />
              ) : item.shape ? (
                <Shape name={item.shape} size={30} />
              ) : (
                <span className="num">{item.num != null ? item.num : i + 1}</span>
              )}
              <span>
                {item.label}
                {item.sub && <small className="learn-sub">{item.sub}</small>}
                {item.marbles != null && (
                  <small className="learn-sub">
                    <Marbles
                      count={item.marbles}
                      size={item.perRow === 10 ? 14 : 16}
                      perRow={item.perRow || 5}
                      colorByRow={!!item.colorByRow}
                    />
                  </small>
                )}
              </span>
            </li>
          ))}
        </ul>
      )}

      <button className="big-btn" onClick={onQuiz}>🎯 Je suis prêt·e, quiz !</button>
      <button className="back-link" onClick={onBack}>← Retour au menu</button>
    </>
  )
}
