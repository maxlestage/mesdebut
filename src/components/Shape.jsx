// figures géométriques dessinées en SVG (viewBox 100×100)
const SHAPE_SVGS = {
  cercle: <circle cx="50" cy="50" r="42" />,
  'carré': <rect x="14" y="14" width="72" height="72" rx="4" />,
  triangle: <polygon points="50,10 90,86 10,86" />,
  rectangle: <rect x="6" y="26" width="88" height="48" rx="4" />,
  losange: <polygon points="50,6 90,50 50,94 10,50" />,
  ovale: <ellipse cx="50" cy="50" rx="44" ry="28" />,
  'étoile': <polygon points="50,6 61,36 93,36 67,56 77,88 50,69 23,88 33,56 7,36 39,36" />,
  'cœur': <path d="M50 84 C50 84 12 58 12 32 C12 18 24 12 33 12 C42 12 48 18 50 24 C52 18 58 12 67 12 C76 12 88 18 88 32 C88 58 50 84 50 84 Z" />,
  hexagone: <polygon points="50,6 88,28 88,72 50,94 12,72 12,28" />,
  octogone: <polygon points="31,8 69,8 92,31 92,69 69,92 31,92 8,69 8,31" />,
}

export default function Shape({ name, size = 72, color = '#667eea' }) {
  return (
    <svg width={size} height={size} viewBox="0 0 100 100" fill={color} aria-label={name} role="img">
      {SHAPE_SVGS[name]}
    </svg>
  )
}
