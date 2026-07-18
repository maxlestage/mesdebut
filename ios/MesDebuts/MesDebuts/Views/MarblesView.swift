import SwiftUI

// Billes à compter, groupées par rangées (5 par défaut, ou 10 pour les dizaines).
struct MarblesView: View {
    let count: Int
    var size: CGFloat = 28
    var perRow: Int = 5
    var colorByRow: Bool = false

    static let colors: [Color] = [
        Color(hex: "#e53935"), Color(hex: "#1e88e5"), Color(hex: "#fdd835"), Color(hex: "#43a047"),
        Color(hex: "#fb8c00"), Color(hex: "#8e24aa"), Color(hex: "#f06292"), Color(hex: "#26c6da"),
        Color(hex: "#795548"), Color(hex: "#5c6bc0"),
    ]

    private var rows: [[Int]] {
        stride(from: 0, to: count, by: perRow).map { start in
            Array(start ..< min(start + perRow, count))
        }
    }

    var body: some View {
        if count == 0 {
            Text("(aucune bille)")
                .italic()
                .foregroundColor(.secondary)
        } else {
            VStack(spacing: 6) {
                ForEach(Array(rows.enumerated()), id: \.offset) { r, row in
                    HStack(spacing: 6) {
                        ForEach(row, id: \.self) { i in
                            marble(base: Self.colors[(colorByRow ? r : i) % Self.colors.count])
                        }
                    }
                }
            }
        }
    }

    private func marble(base: Color) -> some View {
        ZStack {
            Circle().fill(base)
            Circle().fill(
                RadialGradient(gradient: Gradient(colors: [Color.white.opacity(0.9), Color.white.opacity(0)]),
                               center: UnitPoint(x: 0.32, y: 0.30), startRadius: 0, endRadius: size * 0.5)
            )
        }
        .frame(width: size, height: size)
        .shadow(color: Color.black.opacity(0.2), radius: 1.5, x: 0, y: 2)
    }
}
