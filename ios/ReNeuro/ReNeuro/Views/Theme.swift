import SwiftUI

// Le fond des écrans est `Color(.systemBackground)` : blanc en thème clair,
// noir en thème sombre. Un texte de couleur figée devient donc illisible dans
// l'un des deux — l'énoncé du quiz tombait à 1,66:1 de contraste sur fond noir.
// Chaque couleur posée sur ce fond a donc ses deux versions, toutes deux
// au-dessus du seuil WCAG AA de 4,5:1. Les couleurs posées sur un fond fixe
// (les boutons-pilules) restent figées : elles sont lisibles dans les deux cas.
extension Color {
    init(clair: String, sombre: String) {
        self = Color(UIColor { traits in
            UIColor(Color(hex: traits.userInterfaceStyle == .dark ? sombre : clair))
        })
    }

    static let encre = Color(clair: "#333333", sombre: "#f2f2f7")
    static let titre = Color(clair: "#4a3f8f", sombre: "#c9c0ff")
    static let accentDoux = Color(clair: "#5d51ad", sombre: "#bcb2f2")
    static let accentPale = Color(clair: "#6f63b0", sombre: "#a99fdd")
    static let alerte = Color(clair: "#c23434", sombre: "#ff8f8f")
    static let attention = Color(clair: "#9c6000", sombre: "#ffb340")
}

let brandGradient = LinearGradient(
    colors: [Color(hex: "#667eea"), Color(hex: "#764ba2")],
    startPoint: .topLeading, endPoint: .bottomTrailing)

struct ScreenTitle: View {
    let text: String
    var body: some View {
        Text(text)
            .font(.title2.bold())
            .foregroundColor(.titre)
            .multilineTextAlignment(.center)
            .frame(maxWidth: .infinity)
    }
}

struct ScreenSubtitle: View {
    let text: String
    var body: some View {
        Text(text)
            .foregroundColor(.secondary)
            .multilineTextAlignment(.center)
            .frame(maxWidth: .infinity)
    }
}

struct BigButton: View {
    let title: String
    var secondary: Bool = false
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.headline)
                .frame(maxWidth: .infinity, minHeight: 56)
                .foregroundColor(secondary ? Color(hex: "#555555") : .white)
                .background {
                    if secondary { Color(hex: "#eeeeee") } else { brandGradient }
                }
                .clipShape(RoundedRectangle(cornerRadius: 14))
        }
        .buttonStyle(.plain)
    }
}

struct BackButton: View {
    let title: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.callout.weight(.bold))
                .padding(.horizontal, 24)
                .frame(minHeight: 48)
                .foregroundColor(Color(hex: "#6a5fc9"))
                .background(Color(hex: "#f4f2ff"))
                .clipShape(Capsule())
                .overlay(Capsule().stroke(Color(hex: "#e2ddff"), lineWidth: 2))
        }
        .buttonStyle(.plain)
        .padding(.top, 6)
    }
}

// Carte blanche arrondie sur fond dégradé, avec défilement interne.
struct ScreenCard<Content: View>: View {
    @ViewBuilder var content: Content
    var body: some View {
        ScrollView {
            VStack(spacing: 16) { content }
                .padding(24)
                .frame(maxWidth: .infinity)
        }
        .background(Color(.systemBackground))
        .clipShape(RoundedRectangle(cornerRadius: 28))
        .padding(.horizontal, 12)
        .padding(.vertical, 14)
        .frame(maxWidth: 600)
    }
}
