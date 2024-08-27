import SwiftUI
import KhiinSwift

struct CandidateView: View {
    @EnvironmentObject private var viewModel: CandidateViewModel

    var body: some View {
        let candList = self.viewModel.currentCommand.response.candidateList
        let candidates = candList.candidates
        let focused = Int(candList.focused)

        let start = focused / 9 * 9
        let end = start + 9 > candidates.count ? candidates.count : start + 9
        let display = self.viewModel.toShow ? candidates[start..<end] : []
        let focus = focused % 9 + 1

        ZStack {
            VStack(alignment: .leading, spacing: 0) {
                ForEach(Array(zip(1...9, display)), id: \.0) {
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
        .padding(8)
    }
}

struct CandidateItem: View {
    var index: Int
    var focus: Int
    var candidate: Khiin_Proto_Candidate

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack(spacing: 0) {
                RoundedRectangle(
                    cornerRadius: 8, style: .continuous
                )
                .fill(
                    index == focus ? .blue : .clear
                )
                .frame(width: 4, height: 16)

                Text("\(index).")
                    .frame(minWidth: 16)
                Text(candidate.value)
            }
            .frame(height: 24)
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
}
