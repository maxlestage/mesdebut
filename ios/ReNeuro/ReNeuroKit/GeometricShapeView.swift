import SwiftUI

// Formes géométriques dessinées en vectoriel (repère 100×100, comme les SVG du web).
struct GeometricShape: Shape {
    let name: String

    func path(in rect: CGRect) -> Path {
        let s = min(rect.width, rect.height) / 100.0
        func p(_ x: CGFloat, _ y: CGFloat) -> CGPoint { CGPoint(x: x * s, y: y * s) }
        var path = Path()

        func polygon(_ pts: [(CGFloat, CGFloat)]) {
            guard let first = pts.first else { return }
            path.move(to: p(first.0, first.1))
            for pt in pts.dropFirst() { path.addLine(to: p(pt.0, pt.1)) }
            path.closeSubpath()
        }

        switch name {
        case "cercle":
            path.addEllipse(in: CGRect(x: 8 * s, y: 8 * s, width: 84 * s, height: 84 * s))
        case "carré":
            path.addRoundedRect(in: CGRect(x: 14 * s, y: 14 * s, width: 72 * s, height: 72 * s),
                                cornerSize: CGSize(width: 4 * s, height: 4 * s))
        case "triangle":
            polygon([(50, 10), (90, 86), (10, 86)])
        case "rectangle":
            path.addRoundedRect(in: CGRect(x: 6 * s, y: 26 * s, width: 88 * s, height: 48 * s),
                                cornerSize: CGSize(width: 4 * s, height: 4 * s))
        case "losange":
            polygon([(50, 6), (90, 50), (50, 94), (10, 50)])
        case "ovale":
            path.addEllipse(in: CGRect(x: 6 * s, y: 22 * s, width: 88 * s, height: 56 * s))
        case "étoile":
            polygon([(50, 6), (61, 36), (93, 36), (67, 56), (77, 88),
                     (50, 69), (23, 88), (33, 56), (7, 36), (39, 36)])
        case "cœur":
            path.move(to: p(50, 84))
            path.addCurve(to: p(12, 32), control1: p(50, 84), control2: p(12, 58))
            path.addCurve(to: p(33, 12), control1: p(12, 18), control2: p(24, 12))
            path.addCurve(to: p(50, 24), control1: p(42, 12), control2: p(48, 18))
            path.addCurve(to: p(67, 12), control1: p(52, 18), control2: p(58, 12))
            path.addCurve(to: p(88, 32), control1: p(76, 12), control2: p(88, 18))
            path.addCurve(to: p(50, 84), control1: p(88, 58), control2: p(50, 84))
            path.closeSubpath()
        case "hexagone":
            polygon([(50, 6), (88, 28), (88, 72), (50, 94), (12, 72), (12, 28)])
        case "octogone":
            polygon([(31, 8), (69, 8), (92, 31), (92, 69), (69, 92), (31, 92), (8, 69), (8, 31)])
        default:
            break
        }
        return path
    }
}

struct GeometricShapeView: View {
    let name: String
    var size: CGFloat = 72
    var color: Color = Color(hex: "#667eea")

    var body: some View {
        GeometricShape(name: name)
            .fill(color)
            .frame(width: size, height: size)
    }
}
