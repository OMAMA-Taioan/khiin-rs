import InputMethodKit
import SwiftyBeaver
import KhiinSwift

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
        log.debug("deactivateServer ");
        if (isManualMode() && isEdited()) {
            _ = commitCurrent();
            candidateViewModel.reset();
        }
        self.window?.setFrame(.zero, display: true)
    }

    func isEdited() -> Bool {
        return self.candidateViewModel.currentCommand.response.editState != .esEmpty
    }

    func isCommited() -> Bool {
        return self.candidateViewModel.currentCommand.response.committed;
    }

    func isManualMode() -> Bool {
        return EngineController.instance.isManualMode();
    }

    func getHyphenKey() -> String {
        return isEdited() ? EngineController.instance.hyphenKey() : "";
    }

    func commitCurrent() -> Bool {
        var commitText = ""
        if (isManualMode()) {
            commitText = currentDisplayText();
        } else {
            let candList = self.candidateViewModel
                .currentCommand
                .response
                .candidateList

            let candidates = candList.candidates
            let focus = Int(candList.focused)
            
            guard candidates.count > 0 else {
                return false
            }

            commitText = candidates[focus < 0 ? 0 : focus].value
        }


        if (commitText.isEmpty) {
            return false
        }
        
        guard let client = self.currentClient else {
            return false
        }

        client.insert(commitText)
        EngineController.instance.reset()
        self.window?.setFrame(.zero, display: true)
        return true
    }

    func currentDisplayText() -> String {
    
        // Khiin_Proto_Preedit
        let preedit = self.candidateViewModel
            .currentCommand
            .response
            .preedit
        
        var disp_buffer = ""
        // var attr_buffer = ""

        // var char_count = 0
        // var caret = 0

        for segment in preedit.segments {
            log.debug("segment: \(segment)")
            var disp_seg = ""

            // if preedit.caret == char_count {
            //     caret = disp_buffer.count + disp_seg.count
            // }

            for ch in segment.value {
                disp_seg.append(ch)
                // char_count += 1
            }

            // let attr: Character
            // switch segment.status {
            // case .ssUnmarked:
            //     attr = " "
            // case .ssComposing:
            //     attr = "┄"
            // case .ssConverted:
            //     attr = "─"
            // case .ssFocused:
            //     attr = "━"
            // default:
            //     attr = " "
            // }

            // let seg_width = disp_seg.count
            // let seg_attr = String(repeating: String(attr), count: seg_width)
            disp_buffer.append(disp_seg)
            // attr_buffer.append(seg_attr)
            log.debug("disp_buffer: \(disp_buffer)")
        }

        // if preedit.caret == char_count {
        //     caret = disp_buffer.count
        // }

        return disp_buffer

    }

    func reset() {
        self.candidateViewModel.reset()
        self.window?.setFrame(.zero, display: true)
        self.resetWindow()
        EngineController.instance.reset()
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
