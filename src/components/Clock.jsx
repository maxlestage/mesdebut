// Horloge analogique à aiguilles, dessinée en SVG (repère 100×100).
// La petite aiguille (foncée, courte) indique les heures,
// la grande (violette, longue) indique les minutes.
export default function Clock({ hours, minutes, size = 150 }) {
  const toRad = (deg) => (deg * Math.PI) / 180
  // l'aiguille des heures avance aussi avec les minutes (3h30 → entre 3 et 4)
  const hourAngle = toRad((hours % 12) * 30 + minutes * 0.5)
  const minuteAngle = toRad(minutes * 6)
  const tip = (angle, len) => [50 + len * Math.sin(angle), 50 - len * Math.cos(angle)]

  const [hx, hy] = tip(hourAngle, 25)
  const [mx, my] = tip(minuteAngle, 37)

  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      role="img"
      aria-label={`Horloge indiquant ${hours} heures ${minutes} minutes`}
    >
      <circle cx="50" cy="50" r="47" fill="#fff" stroke="#4a3f8f" strokeWidth="3" />

      {/* graduations : trait long toutes les 5 minutes */}
      {Array.from({ length: 60 }, (_, i) => {
        const a = toRad(i * 6)
        const inner = i % 5 === 0 ? 41 : 44
        const [x1, y1] = tip(a, inner)
        const [x2, y2] = tip(a, 46)
        return (
          <line key={i} x1={x1} y1={y1} x2={x2} y2={y2}
                stroke={i % 5 === 0 ? '#8a7fc9' : '#d5cff0'}
                strokeWidth={i % 5 === 0 ? 1.6 : 0.7} />
        )
      })}

      {/* chiffres des heures */}
      {Array.from({ length: 12 }, (_, i) => {
        const a = toRad(i * 30)
        const [x, y] = tip(a, 33)
        return (
          <text key={i} x={x} y={y} textAnchor="middle" dominantBaseline="central"
                fontSize="11" fontWeight="700" fill="#4a3f8f">
            {i === 0 ? 12 : i}
          </text>
        )
      })}

      {/* petite aiguille = heures, grande aiguille = minutes */}
      <line x1="50" y1="50" x2={hx} y2={hy} stroke="#4a3f8f" strokeWidth="5" strokeLinecap="round" />
      <line x1="50" y1="50" x2={mx} y2={my} stroke="#667eea" strokeWidth="3" strokeLinecap="round" />
      <circle cx="50" cy="50" r="3.2" fill="#4a3f8f" />
    </svg>
  )
}
