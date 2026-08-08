import SwiftUI

let brandGradient = LinearGradient(
    colors: [Color(hex: "#667eea"), Color(hex: "#764ba2")],
    startPoint: .topLeading, endPoint: .bottomTrailing)

struct ScreenTitle: View {
    let text: String
    var body: some View {
        Text(text)
            .font(.title2.bold())
            .foregroundColor(Color(hex: "#4a3f8f"))
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
