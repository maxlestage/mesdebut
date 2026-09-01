import SwiftUI

// Horloge analogique à aiguilles, dessinée dans un Canvas.
// La petite aiguille (foncée, courte) indique les heures,
// la grande (violette, longue) indique les minutes.
struct ClockView: View {
    let hours: Int
    let minutes: Int
    var size: CGFloat = 150

    var body: some View {
        Canvas { ctx, canvasSize in
            let c = CGPoint(x: canvasSize.width / 2, y: canvasSize.height / 2)
            let r = min(canvasSize.width, canvasSize.height) / 2

            // point situé à `frac` × rayon, dans la direction `deg` (0° = midi, sens horaire)
            func pt(_ deg: Double, _ frac: Double) -> CGPoint {
                let a = deg * .pi / 180
                return CGPoint(x: c.x + r * frac * sin(a), y: c.y - r * frac * cos(a))
            }

            // cadran
            let dial = Path(ellipseIn: CGRect(x: c.x - r * 0.97, y: c.y - r * 0.97,
                                              width: r * 1.94, height: r * 1.94))
            ctx.fill(dial, with: .color(.white))
            ctx.stroke(dial, with: .color(Color(hex: "#4a3f8f")), lineWidth: r * 0.06)

            // graduations : trait long toutes les 5 minutes
            for i in 0 ..< 60 {
                var p = Path()
                p.move(to: pt(Double(i) * 6, i % 5 == 0 ? 0.82 : 0.88))
                p.addLine(to: pt(Double(i) * 6, 0.93))
                ctx.stroke(p,
                           with: .color(Color(hex: i % 5 == 0 ? "#8a7fc9" : "#d5cff0")),
                           lineWidth: i % 5 == 0 ? r * 0.032 : r * 0.014)
            }

            // chiffres des heures
            for i in 1 ... 12 {
                let label = Text("\(i)")
                    .font(.system(size: r * 0.21, weight: .bold))
                    .foregroundColor(Color(hex: "#4a3f8f"))
                ctx.draw(label, at: pt(Double(i) * 30, 0.66))
            }

            // petite aiguille = heures (elle avance aussi avec les minutes)
            var hourHand = Path()
            hourHand.move(to: c)
            hourHand.addLine(to: pt(Double(hours % 12) * 30 + Double(minutes) * 0.5, 0.50))
            ctx.stroke(hourHand, with: .color(Color(hex: "#4a3f8f")),
                       style: StrokeStyle(lineWidth: r * 0.10, lineCap: .round))

            // grande aiguille = minutes
            var minuteHand = Path()
            minuteHand.move(to: c)
            minuteHand.addLine(to: pt(Double(minutes) * 6, 0.74))
            ctx.stroke(minuteHand, with: .color(Color(hex: "#667eea")),
                       style: StrokeStyle(lineWidth: r * 0.06, lineCap: .round))

            // axe central
            let dot = Path(ellipseIn: CGRect(x: c.x - r * 0.065, y: c.y - r * 0.065,
                                             width: r * 0.13, height: r * 0.13))
            ctx.fill(dot, with: .color(Color(hex: "#4a3f8f")))
        }
        .frame(width: size, height: size)
        .accessibilityLabel("Horloge indiquant \(hours) heures \(minutes) minutes")
    }
}
