import SwiftUI

struct HUDVisualEffect: NSViewRepresentable {
    func makeNSView(context: Self.Context) -> NSView {
        let view = NSVisualEffectView()
        view.material = .hudWindow
        view.blendingMode = .behindWindow
        view.state = .active
        return view
    }
    func updateNSView(_ nsView: NSView, context: Context) {}
}

struct CandidateView: View {
    @EnvironmentObject private var viewModel: CandidateViewModel

    var body: some View {
        let candList = self.viewModel.currentCommand.response.candidateList

        VStack(alignment: .leading, spacing: 0) {
            ForEach(0..<candList.candidates.count, id: \.self) { index in
                Text(candList.candidates[index].value)
            }
        }
        .padding(16)
        .background(
            HUDVisualEffect()
                .cornerRadius(8)
                .shadow(radius: 4)
        )
    }
}
