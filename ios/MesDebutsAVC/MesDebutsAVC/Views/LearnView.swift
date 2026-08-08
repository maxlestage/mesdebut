import SwiftUI

struct LearnRow: View {
    let item: LearnItem
    var index: Int = 0

    var body: some View {
        HStack(spacing: 12) {
            leading
            VStack(alignment: .leading, spacing: 3) {
                Text(item.label)
                    .font(.body.weight(.semibold))
                    .foregroundColor(Color(hex: "#4a3f8f"))
                if let sub = item.sub {
                    Text(sub).font(.caption).foregroundColor(Color(hex: "#8a7fc9"))
                }
                if let m = item.marbles {
                    MarblesView(count: m, size: item.perRow == 10 ? 14 : 16,
                                perRow: item.perRow, colorByRow: item.colorByRow)
                        .padding(.top, 2)
                }
            }
            Spacer(minLength: 0)
        }
        .padding(.horizontal, 14)
        .padding(.vertical, 10)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color(hex: "#f4f2ff"))
        .clipShape(RoundedRectangle(cornerRadius: 10))
    }

    @ViewBuilder private var leading: some View {
        if let hex = item.colorHex {
            Circle()
                .fill(Color(hex: hex))
                .frame(width: 30, height: 30)
                .overlay(Circle().stroke(Color(hex: "#cccccc"),
                                         lineWidth: hex.lowercased() == "#ffffff" ? 2 : 0))
        } else if let shape = item.shapeName {
            GeometricShapeView(name: shape, size: 30)
        } else {
            Text("\(item.num ?? index + 1)")
                .font(.caption.bold())
                .foregroundColor(.white)
                .frame(minWidth: 30, minHeight: 30)
                .padding(.horizontal, 6)
                .background(Color(hex: "#667eea"))
                .clipShape(Capsule())
        }
    }
}

struct LearnView: View {
    let categoryKey: String
    var onQuiz: () -> Void
    var onBack: () -> Void

    @State private var expanded: Set<UUID> = []

    private let letterColumns = Array(repeating: GridItem(.flexible(), spacing: 8), count: 5)

    var body: some View {
        Group {
            switch QuestionEngine.learnContent(categoryKey) {
            case let .list(title, intro, items):
                ScreenTitle(text: title)
                if let intro { ScreenSubtitle(text: intro) }
                VStack(spacing: 8) {
                    ForEach(Array(items.enumerated()), id: \.element.id) { i, item in
                        LearnRow(item: item, index: i)
                    }
                }

            case let .grid(title, letters):
                ScreenTitle(text: title)
                LazyVGrid(columns: letterColumns, spacing: 8) {
                    ForEach(letters, id: \.self) { l in
                        Text(l)
                            .font(.title3.bold())
                            .foregroundColor(Color(hex: "#4a3f8f"))
                            .frame(maxWidth: .infinity, minHeight: 44)
                            .background(Color(hex: "#f4f2ff"))
                            .clipShape(RoundedRectangle(cornerRadius: 10))
                    }
                }

            case let .accordion(title, intro, groups):
                ScreenTitle(text: title)
                if let intro { ScreenSubtitle(text: intro) }
                VStack(spacing: 8) {
                    ForEach(Array(groups.enumerated()), id: \.element.id) { gi, group in
                        AccordionGroupView(group: group, startOpen: gi == 0)
                    }
                }
            }

            BigButton(title: "🎯 Je suis prêt·e, quiz !", action: onQuiz)
            BackButton(title: "← Retour au menu", action: onBack)
        }
    }
}

struct AccordionGroupView: View {
    let group: LearnGroup
    let startOpen: Bool
    @State private var isOpen: Bool = false

    var body: some View {
        VStack(spacing: 8) {
            Button {
                withAnimation(.easeInOut(duration: 0.15)) { isOpen.toggle() }
            } label: {
                HStack {
                    Text(group.label).font(.headline).foregroundColor(.white)
                    Spacer()
                    Image(systemName: isOpen ? "chevron.down" : "chevron.right")
                        .foregroundColor(.white.opacity(0.85))
                        .font(.subheadline)
                }
                .padding(.horizontal, 16)
                .frame(maxWidth: .infinity, minHeight: 52)
                .background(brandGradient)
                .clipShape(RoundedRectangle(cornerRadius: 12))
            }
            .buttonStyle(.plain)

            if isOpen {
                VStack(spacing: 8) {
                    ForEach(group.items) { item in LearnRow(item: item) }
                }
            }
        }
        .onAppear { isOpen = startOpen }
    }
}
