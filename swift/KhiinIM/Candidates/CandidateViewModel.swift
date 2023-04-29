import SwiftUI

class CandidateViewModel: ObservableObject {
    @Published private(set) var currentCommand = Khiin_Proto_Command()

    func reset() {
        self.currentCommand = Khiin_Proto_Command()
    }

    func handleChar(_ char: String) {
        let engine = EngineController.instance

        guard let res = engine.handleChar(char.first!) else {
            return
        }

        self.currentCommand = res
    }
}
