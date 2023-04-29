import InputMethodKit
import SwiftUI
import SwiftyBeaver

// @objc(InputController)
class KhiinInputController: IMKInputController {
    lazy var window: NSWindow? = nil

    lazy var currentClient: IMKTextInput? = nil {
        didSet {
            if window != nil {
                self.resetWindow()
            }
        }
    }

    lazy var currentOrigin: CGPoint? = nil

    let candidateViewModel = CandidateViewModel()

    override func activateServer(_ sender: Any!) {
        Logger.setup()
        EngineController.instance.reset()
        self.currentClient = sender as? IMKTextInput
        self.currentOrigin = self.currentClient?.position
    }

    override func deactivateServer(_ sender: Any!) {
        self.window?.setFrame(.zero, display: true)
    }

    func commitCurrent() -> Bool {
        let candidates = self.candidateViewModel
            .currentCommand
            .response
            .candidateList
            .candidates
        
        guard candidates.count > 0 else {
            return false
        }
        
        guard let client = self.currentClient else {
            return false
        }
        
        client.insert(candidates[0].value)
        EngineController.instance.reset()
        self.window?.setFrame(.zero, display: true)
        return true
    }

    //    override func inputText(_ string: String!, client sender: Any!) -> Bool {
    //        log.debug("inputText: \(string ?? "n/a")")
    //
    //        guard let client = self.currentClient else {
    //            return false
    //        }
    //
    //        if let first = string.first, first.isASCII && first.isLetter {
    //            let engine = EngineController.instance
    //
    //            let cmd = engine.handleChar(Int32(first.asciiValue!))
    //            if let cand = cmd?.response.candidateList.candidates.first?.value {
    //                client.insertText(
    //                    cand,
    //                    replacementRange: NSRange(
    //                        location: NSNotFound,
    //                        length: NSNotFound
    //                    )
    //                )
    //
    //                return true
    //            }
    //        }
    //
    //        client.insertText(
    //            string + string + string,
    //            replacementRange: NSRange(
    //                location: NSNotFound,
    //                length: NSNotFound
    //            )
    //        )
    //
    //        return true
    //    }
}
