import SwiftUI

// Horloge analogique à aiguilles, composée de vues SwiftUI simples.
// La petite aiguille (foncée, courte) indique les heures,
// la grande (violette, longue) indique les minutes.
struct ClockView: View {
    let hours: Int
    let minutes: Int
    var size: CGFloat = 150

    // l'aiguille des heures avance aussi avec les minutes (3 h 30 → entre 3 et 4)
    private var hourDegrees: Double { Double(hours % 12) * 30 + Double(minutes) * 0.5 }
    private var minuteDegrees: Double { Double(minutes) * 6 }

    private let dialColor = Color(hex: "#4a3f8f")
    private let handColor = Color(hex: "#667eea")

    var body: some View {
        ZStack {
            Circle().fill(Color.white)
            Circle().strokeBorder(dialColor, lineWidth: size * 0.03)

            // graduations : trait long toutes les 5 minutes
            ForEach(0 ..< 60, id: \.self) { i in
                let major = i % 5 == 0
                Capsule()
                    .fill(Color(hex: major ? "#8a7fc9" : "#d5cff0"))
                    .frame(width: major ? size * 0.016 : size * 0.007,
                           height: major ? size * 0.055 : size * 0.025)
                    .offset(y: -size * 0.435)
                    .rotationEffect(.degrees(Double(i) * 6))
            }

            // chiffres des heures, maintenus droits par une contre-rotation
            ForEach(1 ... 12, id: \.self) { i in
                Text("\(i)")
                    .font(.system(size: size * 0.105, weight: .bold))
                    .foregroundColor(dialColor)
                    .rotationEffect(.degrees(-Double(i) * 30))
                    .offset(y: -size * 0.33)
                    .rotationEffect(.degrees(Double(i) * 30))
            }

            // petite aiguille = heures
            Capsule()
                .fill(dialColor)
                .frame(width: size * 0.05, height: size * 0.25)
                .offset(y: -size * 0.125)
                .rotationEffect(.degrees(hourDegrees))

            // grande aiguille = minutes
            Capsule()
                .fill(handColor)
                .frame(width: size * 0.03, height: size * 0.37)
                .offset(y: -size * 0.185)
                .rotationEffect(.degrees(minuteDegrees))

            // axe central
            Circle()
                .fill(dialColor)
                .frame(width: size * 0.07, height: size * 0.07)
        }
        .frame(width: size, height: size)
        .accessibilityLabel("Horloge indiquant \(hours) heures \(minutes) minutes")
    }
}
