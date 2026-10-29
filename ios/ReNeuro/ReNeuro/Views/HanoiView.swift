import SwiftUI

// « 🗼 Tours de Hanoï » : le mini-jeu de réflexion. Les règles viennent du noyau
// (HanoiEngine) ; ces vues dessinent les tiges et transmettent les coups. On
// touche une tige pour prendre son disque du haut, puis une autre pour l'y poser.

struct HanoiLevelsView: View {
    var onSelect: (Int) -> Void
    var onBack: () -> Void

    var body: some View {
        Group {
            ScreenTitle(text: "🗼 Tours de Hanoï")
            ScreenSubtitle(text: "Déplace toute la pile sur la dernière tige, un disque à la fois, sans jamais poser un grand disque sur un plus petit.")
            ForEach(HanoiEngine.levels) { level in
                BigButton(title: "\(level.emoji) \(level.libelle)") { onSelect(level.id) }
            }
            BackButton(title: "← Retour au menu", action: onBack)
        }
    }
}

struct HanoiView: View {
    let level: HanoiLevel
    var onLevels: () -> Void
    var onNext: (Int) -> Void

    @State private var game: HanoiGame?
    /// La tige dont on a pris le disque du haut.
    @State private var held: Int?
    @State private var refused = false

    /// Du plus petit disque au plus grand ; le numéro blanc reste lisible sur chacun.
    private static let colors = ["#c2185b", "#6a1b9a", "#1565c0", "#00796b", "#e65100"]

    var body: some View {
        Group {
            Text("🗼 Tours de Hanoï · \(level.libelle)")
                .font(.subheadline.bold())
                .foregroundColor(.accentPale)
            if let game {
                HStack {
                    Text("Coups : \(game.coups)")
                    Spacer()
                    Text("Minimum : \(game.minimum)")
                }
                .font(.footnote)
                .foregroundColor(.secondary)

                HStack(spacing: 8) {
                    ForEach(0..<3, id: \.self) { i in peg(i, in: game) }
                }

                if game.gagne {
                    finish(game)
                } else {
                    Text(refused ? "Un grand disque ne va jamais sur un plus petit !"
                                 : "Touche une tige pour prendre son disque du haut, puis une autre pour l'y poser.")
                        .font(.callout.weight(.semibold))
                        .foregroundColor(refused ? .alerte : .secondary)
                        .multilineTextAlignment(.center)
                    BigButton(title: "🔁 Recommencer", secondary: true, action: restart)
                }
            }
            BackButton(title: "← Changer de niveau", action: onLevels)
        }
        .onAppear { if game == nil { restart() } }
    }

    private func peg(_ i: Int, in game: HanoiGame) -> some View {
        let tower = game.tours[i]
        let isHeld = held == i
        return Button { tap(i) } label: {
            GeometryReader { geo in
                ZStack(alignment: .bottom) {
                    Capsule()
                        .fill(Color.accentPale.opacity(0.45))
                        .frame(width: 6)
                        .padding(.top, 18)
                        .padding(.bottom, 10)
                    Capsule()
                        .fill(Color.accentPale.opacity(0.45))
                        .frame(width: geo.size.width * 0.8, height: 5)
                        .padding(.bottom, 8)
                    VStack(spacing: 4) {
                        // du haut vers le bas : le dernier disque de la tige en premier
                        ForEach(Array(tower.reversed()), id: \.self) { disk in
                            diskView(disk, of: game.disques, width: geo.size.width,
                                     lifted: isHeld && disk == tower.last)
                        }
                    }
                    .padding(.bottom, 14)
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
            }
            .frame(height: 190)
            .background(isHeld ? Color.accentPale.opacity(0.15) : Color(.secondarySystemBackground))
            .clipShape(RoundedRectangle(cornerRadius: 14))
            .overlay(
                RoundedRectangle(cornerRadius: 14)
                    .stroke(isHeld ? Color.accentDoux : Color(.separator),
                            style: StrokeStyle(lineWidth: 3, dash: i == 2 && !isHeld ? [6, 4] : []))
            )
            .animation(.easeOut(duration: 0.15), value: isHeld)
        }
        .buttonStyle(.plain)
        .accessibilityLabel(label(i, tower: tower, isHeld: isHeld))
    }

    private func diskView(_ disk: Int, of count: Int, width: CGFloat, lifted: Bool) -> some View {
        let ratio = 0.3 + 0.7 * CGFloat(disk) / CGFloat(max(count, 1))
        return Text("\(disk)")
            .font(.caption.bold())
            .foregroundColor(.white)
            .frame(width: width * ratio, height: 24)
            .background(Color(hex: Self.colors[(disk - 1) % Self.colors.count]))
            .clipShape(Capsule())
            .shadow(color: .black.opacity(0.2), radius: 2, y: 2)
            .offset(y: lifted ? -14 : 0)
    }

    private func label(_ i: Int, tower: [Int], isHeld: Bool) -> String {
        guard let top = tower.last else { return "Tige \(i + 1) : vide" }
        if isHeld { return "Tige \(i + 1) : disque \(top) pris" }
        return "Tige \(i + 1) : \(tower.count) disque\(tower.count > 1 ? "s" : ""), en haut le \(top)"
    }

    @ViewBuilder private func finish(_ game: HanoiGame) -> some View {
        ScreenTitle(text: "Bravo, la pile est arrivée !")
        Text(game.etoiles ?? "").font(.system(size: 44))
        Text(game.message ?? "")
            .font(.title3.bold())
            .foregroundColor(.titre)
            .multilineTextAlignment(.center)
        BigButton(title: "🔁 Rejouer", action: restart)
        if let next = HanoiEngine.levels.first(where: { $0.id == level.id + 1 }) {
            BigButton(title: "\(next.emoji) Essayer avec \(next.libelle)") { onNext(next.id) }
        }
    }

    private func restart() {
        game = HanoiEngine.start(disks: level.disques)
        held = nil
        refused = false
    }

    private func tap(_ i: Int) {
        guard let current = game, !current.gagne else { return }
        guard let from = held else {
            if !current.tours[i].isEmpty {
                held = i
                refused = false
            }
            return
        }
        held = nil
        guard from != i, let next = HanoiEngine.move(current, from: from, to: i) else { return }
        if next.accepte {
            game = next
            refused = false
            if next.gagne { Haptics.success() }
        } else {
            refused = true
            Haptics.error()
        }
    }
}
