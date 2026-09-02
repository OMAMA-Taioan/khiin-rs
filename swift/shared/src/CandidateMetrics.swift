import CoreGraphics
import Foundation

// Sizing for the candidate menu. The size comes from `font_size` in the
// `[candidates]` table of settings.toml, the same file the settings app
// writes and the Windows IME reads: anything 20 or larger is the "大字"
// option, which scales the menu up by 27%, matching the 0.27 scale used in
// windows/ime/src/ui/candidates/candidate_window.rs.
public struct CandidateMetrics {
    public static let smallFontSize = 16
    public static let largeFontSize = 20
    public static let largeScale: CGFloat = 1.27

    public let scale: CGFloat

    public init(fontSize: Int) {
        self.scale =
            fontSize >= CandidateMetrics.largeFontSize
            ? CandidateMetrics.largeScale : 1.0
    }

    public var valueFontSize: CGFloat { 14.0 * self.scale }
    public var annotationFontSize: CGFloat { 9.0 * self.scale }
    // 13.0 is the macOS default body size, what the index used before it
    // became scalable
    public var indexFontSize: CGFloat { 13.0 * self.scale }
    public var indexWidth: CGFloat { 16.0 * self.scale }
    public var markerWidth: CGFloat { 4.0 }
    public var markerHeight: CGFloat { 16.0 * self.scale }
    public var rowHeight: CGFloat { 24.0 * self.scale }
    public var windowWidth: CGFloat { 500.0 * self.scale }
    public var windowPadding: CGFloat { 8.0 }
}

// Reads the one setting the candidate window needs out of settings.toml:
// what the bridge hands back from the engine is a protobuf AppConfig, which
// carries no font size.
func loadCandidateFontSize() -> Int {
    guard let path = getSettingFilePath(),
        let contents = try? String(contentsOfFile: path, encoding: .utf8)
    else {
        return CandidateMetrics.smallFontSize
    }

    return parseCandidateFontSize(contents) ?? CandidateMetrics.smallFontSize
}

func parseCandidateFontSize(_ toml: String) -> Int? {
    var inCandidates = false

    for rawLine in toml.split(whereSeparator: \.isNewline) {
        let line = rawLine.trimmingCharacters(in: .whitespaces)

        if line.hasPrefix("[") {
            inCandidates = line == "[candidates]"
            continue
        }

        guard inCandidates, let eq = line.firstIndex(of: "="),
            line[..<eq].trimmingCharacters(in: .whitespaces) == "font_size"
        else {
            continue
        }

        let value = line[line.index(after: eq)...]
            .trimmingCharacters(in: .whitespaces)
        return Int(value)
    }

    return nil
}
