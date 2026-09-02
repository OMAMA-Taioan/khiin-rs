import SwiftUI
import KhiinSwift

private let focusIndicatorColor = Color(
    red: 0xDA / 255, green: 0x3C / 255, blue: 0x86 / 255)

struct CandidateView: View {
    @EnvironmentObject private var viewModel: CandidateViewModel

    var body: some View {
        let candList = self.viewModel.currentCommand.response.candidateList
        let candidates = candList.candidates
        let focused = Int(candList.focused)
        let page = Int(candList.page)

        let start = page * 9
        let end = start + 9 > candidates.count ? candidates.count : start + 9
        let focus = focused == -1 ? -1 : focused % 9

        ZStack {
            VStack(alignment: .leading, spacing: 0) {
                ForEach(Array(zip(0...8, candidates[start..<end])), id: \.0) {
                    index, candidate in

                    CandidateItem(
                        index: Int(index), focus: focus, candidate: candidate)
                }
            }
            .background(
                Color("BackgroundColor"),
                in: RoundedRectangle(cornerRadius: 8)
            )
            .roundedHUDVisualEffect()
        }
        .padding(EngineController.instance.candidateMetrics.windowPadding)
    }
}

struct CandidateItem: View {
    var index: Int
    var focus: Int
    var candidate: Khiin_Proto_Candidate

    var body: some View {
        let metrics = EngineController.instance.candidateMetrics

        VStack(alignment: .leading, spacing: 0) {
            HStack(spacing: 0) {
                RoundedRectangle(
                    cornerRadius: 8, style: .continuous
                )
                .fill(
                    index == focus ? focusIndicatorColor : .clear
                )
                .frame(width: metrics.markerWidth, height: metrics.markerHeight)

                if index > 0 && focus != -1 {
                    Text("\(index).")
                        .font(.system(size: metrics.indexFontSize))
                        .frame(minWidth: metrics.indexWidth)
                }
                Text(candidate.value).font(.system(size: metrics.valueFontSize))
                formatAnnotation(
                    from: candidate.annotation,
                    fontSize: metrics.annotationFontSize)
            }
            .frame(height: metrics.rowHeight)
            .padding(.horizontal, 8)
            .padding(.vertical, 0)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(
            index == focus
                ? Color("CandidateHighlight")
                : .clear,
            in: RoundedRectangle(cornerRadius: 8)
        )
    }

    func formatAnnotation(from annotation: String, fontSize: CGFloat) -> Text {
        if !annotation.contains(Character("+")) {
            return Text(annotation).font(.system(size: fontSize))
        }
        var result = Text("")
        var preString: String = ""
        for character in annotation {
            if character == "+" {
                result = result + Text(preString).underline().font(.system(size: fontSize))
                preString = ""
            } else {
                result = result + Text(preString).font(.system(size: fontSize))
                preString = String(character);
            }
        }
        result = result + Text(preString).font(.system(size: fontSize))
        return result
    }
}
