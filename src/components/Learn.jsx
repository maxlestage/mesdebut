import { LEARN_DATA } from '../questions.js'

export default function Learn({ category, onQuiz, onBack }) {
  const data = LEARN_DATA[category]
  return (
    <>
      <h1>{data.title}</h1>
      {data.grid ? (
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
              ) : (
                <span className="num">{i + 1}</span>
              )}
              <span>
                {item.label}
                {item.sub && <small className="learn-sub">{item.sub}</small>}
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
