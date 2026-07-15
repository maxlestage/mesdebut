// billes à compter, groupées par 5 pour aider à compter d'un coup d'œil
const MARBLE_COLORS = ['#e53935', '#1e88e5', '#fdd835', '#43a047', '#fb8c00',
  '#8e24aa', '#f06292', '#26c6da', '#795548', '#5c6bc0']

export default function Marbles({ count, size = 28 }) {
  if (count === 0) {
    return <span className="marbles-empty">(aucune bille)</span>
  }
  const rows = []
  for (let start = 0; start < count; start += 5) {
    rows.push(Array.from({ length: Math.min(5, count - start) }, (_, i) => start + i))
  }
  return (
    <span className="marbles" role="img" aria-label={`${count} bille${count > 1 ? 's' : ''}`}>
      {rows.map((row, r) => (
        <span key={r} className="marbles-row">
          {row.map(i => (
            <span
              key={i}
              className="marble"
              style={{ width: size, height: size, background:
                `radial-gradient(circle at 32% 30%, rgba(255,255,255,0.9), rgba(255,255,255,0) 45%), ${MARBLE_COLORS[i % MARBLE_COLORS.length]}` }}
            />
          ))}
        </span>
      ))}
    </span>
  )
}
