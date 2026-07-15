// billes à compter, groupées par rangées pour aider à compter d'un coup d'œil
// (rangées de 5 pour les petits chiffres, rangées de 10 pour travailler les dizaines)
const MARBLE_COLORS = ['#e53935', '#1e88e5', '#fdd835', '#43a047', '#fb8c00',
  '#8e24aa', '#f06292', '#26c6da', '#795548', '#5c6bc0']

export default function Marbles({ count, size = 28, perRow = 5, colorByRow = false }) {
  if (count === 0) {
    return <span className="marbles-empty">(aucune bille)</span>
  }
  const rows = []
  for (let start = 0; start < count; start += perRow) {
    rows.push(Array.from({ length: Math.min(perRow, count - start) }, (_, i) => start + i))
  }
  return (
    <span className="marbles" role="img" aria-label={`${count} bille${count > 1 ? 's' : ''}`}>
      {rows.map((row, r) => (
        <span key={r} className="marbles-row">
          {row.map(i => {
            const color = MARBLE_COLORS[(colorByRow ? r : i) % MARBLE_COLORS.length]
            return (
              <span
                key={i}
                className="marble"
                style={{ width: size, height: size, background:
                  `radial-gradient(circle at 32% 30%, rgba(255,255,255,0.9), rgba(255,255,255,0) 45%), ${color}` }}
              />
            )
          })}
        </span>
      ))}
    </span>
  )
}
