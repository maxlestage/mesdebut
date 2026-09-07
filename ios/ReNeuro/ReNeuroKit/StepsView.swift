import SwiftUI

/// Les étapes montrées sous l'énoncé d'une question de planification.
/// Pour « quelle étape manque ? », le trou est matérialisé à sa place —
/// sans quoi la question serait insoluble.
struct StepsView: View {
    let steps: [String]
    var hole: Int? = nil
    var compact: Bool = false

    /// Les étapes, avec `nil` à l'emplacement du trou éventuel.
    private var lignes: [String?] {
        guard let hole, hole <= steps.count else { return steps.map { $0 } }
        var out: [String?] = steps.map { $0 }
        out.insert(nil, at: hole)
        return out
    }

    var body: some View {
        VStack(alignment: .leading, spacing: compact ? 4 : 6) {
            ForEach(Array(lignes.enumerated()), id: \.offset) { _, ligne in
                Text(ligne ?? "❓ à trouver")
                    .font(compact ? .caption : .subheadline)
                    .fontWeight(ligne == nil ? .bold : .medium)
                    .foregroundColor(ligne == nil ? Color(hex: "#a97a10") : Color(hex: "#444444"))
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.vertical, compact ? 4 : 7)
                    .padding(.horizontal, compact ? 7 : 10)
                    .background(ligne == nil ? Color(hex: "#fff7e0") : Color(hex: "#faf9ff"))
                    .clipShape(RoundedRectangle(cornerRadius: 10))
                    .overlay(
                        RoundedRectangle(cornerRadius: 10)
                            .stroke(ligne == nil ? Color(hex: "#ffd98a") : Color(hex: "#e2ddff"),
                                    lineWidth: 1.5)
                    )
            }
        }
        .frame(maxWidth: compact ? .infinity : 320)
    }
}
