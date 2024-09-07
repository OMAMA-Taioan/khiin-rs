import SwiftUI
import KhiinSwift

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

    func handleBackspace() {
        let engine = EngineController.instance

        guard let res = engine.handleSpecialKey(.skBackspace, false) else {
            return
        }

        self.currentCommand = res
    }

    func handleSpace(_ hasShift: Bool) {
        let engine = EngineController.instance

        guard let res = engine.handleSpecialKey(.skSpace, hasShift) else {
            return
        }
        
        self.currentCommand = res
    }

    func handleTab(_ hasShift: Bool) {
        let engine = EngineController.instance

        guard let res = engine.handleSpecialKey(.skTab, hasShift) else {
            return
        }

        self.currentCommand = res
    }

    func handleArrowUp() {
        let engine = EngineController.instance

        guard let res = engine.handleSpecialKey(.skUp, false) else {
            return
        }

        self.currentCommand = res
    }

    func handleArrowDown() {
        let engine = EngineController.instance

        guard let res = engine.handleSpecialKey(.skDown, false) else {
            return
        }

        self.currentCommand = res
    }

    func handleCommit() {
        let engine = EngineController.instance

        guard let res = engine.sendCommitCommand() else {
            return
        }

        self.currentCommand = res
    }

    func changeInputMode() {
        let engine = EngineController.instance

        guard let res = engine.changeInputMode() else {
            return
        }

        self.currentCommand = res
        
    }
}
