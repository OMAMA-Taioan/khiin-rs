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

    // True while the shift key is held down on its own. Any other key or
    // modifier arriving before it is released clears it, so that shift only
    // switches the input mode when it is tapped alone (see the flagsChanged
    // handling in InputController+handler).
    var shiftHeldAlone: Bool = false

    let candidateViewModel = CandidateViewModel()

    override func activateServer(_ sender: Any!) {
        Logger.setup()
        EngineController.instance.reset()
        self.shiftHeldAlone = false
        self.currentClient = sender as? IMKTextInput
        self.currentOrigin = self.currentClient?.position
    }

    override func deactivateServer(_ sender: Any!) {
        log.debug("deactivateServer ");
        self.shiftHeldAlone = false
        _ = commitAll()
        candidateViewModel.reset()
        self.currentClient?.clearMarkedText()
        self.window?.setFrame(.zero, display: true)
        self.resetWindow()
    }

    override init!(server: IMKServer!, delegate: Any!, client inputClient: Any!) {
        super.init(server: server, delegate: delegate, client: inputClient)
        setupMouseEventMonitor()
    }

    func setupMouseEventMonitor() {
        NSEvent.addGlobalMonitorForEvents(matching: .leftMouseDown) { [weak self] event in
            log.debug("mouse click event")
            self!.resetController()
        }
    }

    func resetController() {
        // A shift-click (extending a selection, say) must not be read as a
        // bare shift tap when the key comes back up.
        self.shiftHeldAlone = false
        _ = commitAll()
        candidateViewModel.reset()
        self.currentClient?.clearMarkedText()
        self.window?.setFrame(.zero, display: true)
        self.resetWindow()
    }

    override func menu() -> NSMenu! {
        // 创建自定义菜单项
        let settingMenuItem = NSMenuItem(
            title: "設定・Siat-tēng",
            action: #selector(self.openSettingApp),
            keyEquivalent: ""
        )
        settingMenuItem.target = self
        
        let khiinMenu = NSMenu();
        khiinMenu.addItem(settingMenuItem)

        return khiinMenu;
    }

    func isEdited() -> Bool {
        return self.candidateViewModel.currentCommand.response.editState != .esEmpty
    }

    func isCommited() -> Bool {
        return self.candidateViewModel.currentCommand.response.committed;
    }

    func isIllegal() -> Bool {
        return self.candidateViewModel.currentCommand.response.editState == .esIllegal
    }

    func isCandidateListOpen() -> Bool {
        return !self.candidateViewModel
            .currentCommand
            .response
            .candidateList
            .candidates
            .isEmpty
    }

    func isManualMode() -> Bool {
        return EngineController.instance.isManualMode();
    }

    func isClassicMode() -> Bool {
        return EngineController.instance.isClassicMode();
    }

    func isHanjiFirst() -> Bool {
        return EngineController.instance.isHanjiFirst();
    }

    func isInputModeShortcutShift() -> Bool {
        return EngineController.instance.isInputModeShortcutShift();
    }

    // Case-insensitive: the engine lower-cases the key before matching it
    // against the configured hyphen/khin keys, so a shifted "D"/"V" is still
    // the hyphen/khin key here.
    func isHyphenOrKhinKey(_ char: String) -> Bool {
        let key = char.lowercased()
        return isEdited()
            && (key == EngineController.instance.hyphenKey().lowercased()
                || key == EngineController.instance.khinKey().lowercased())
    }

    func getCommitedText() -> String {
        return self.candidateViewModel.currentCommand.response.committedText
    }

    func handleResponse() -> Bool {
        guard let client = self.currentClient else {
            return false
        }
        if (self.isCommited()) {
            let commitText = self.candidateViewModel
                .currentCommand
                .response
                .committedText
            client.insert(commitText)
            self.reset()
        } else {
            self.resetWindow()
            client.mark(self.currentDisplayText())
        }
        return true
    }

    func handlePunctuation(_ char: String) -> Bool {
        _ = self.commitAll();
        self.candidateViewModel.handleChar(char)
        return self.handleResponse();
    }

    func commitAll() -> Bool {
        var commitText = ""
        if (isManualMode()) {
            commitText = currentDisplayText();
        } else if (isClassicMode()) {
            self.candidateViewModel.handleCommit();
            commitText = self.candidateViewModel
                .currentCommand
                .response
                .committedText
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
        if (isClassicMode()) {
            self.resetWindow()
            client.mark(self.currentDisplayText())
        } else {
            self.candidateViewModel.reset()
            EngineController.instance.reset()
            self.window?.setFrame(.zero, display: true)
        }
        return true
    }

    // Ends the composition the same way the Windows IME does when the left
    // arrow key cancels it: whatever is shown in the pre-edit is released to
    // the client as-is, without sending a commit to the engine, then the
    // candidate window and the buffer state are reset.
    func releaseComposition() -> Bool {
        guard let client = self.currentClient else {
            return false
        }

        let text = self.currentDisplayText()
        if (text.isEmpty) {
            client.clearMarkedText()
        } else {
            client.insert(text)
        }
        self.reset()
        return true
    }

    func commitCurrent() -> Bool {
        var commitText = ""
        if (isManualMode()) {
            commitText = currentDisplayText();
        } else if (isClassicMode()) {
            self.candidateViewModel.handleEnter();
            commitText = self.candidateViewModel
                .currentCommand
                .response
                .committedText
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
        if (isClassicMode()) {
            self.resetWindow()
            client.mark(self.currentDisplayText())
        } else {
            self.candidateViewModel.reset()
            EngineController.instance.reset()
            self.window?.setFrame(.zero, display: true)
        }
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

    @objc func openSettingApp() {
        let mainBundle = Bundle.main
        let appPath = mainBundle.bundleURL.appendingPathComponent("Contents/Applications/khiin_helper.app").path

        guard let bundle = Bundle(path: appPath),
            let executablePath = bundle.executableURL?.path else {
            log.debug("Can't find helper app.")
            return
        }
        
        let process = Process()
        process.executableURL = URL(fileURLWithPath: executablePath)
        
        process.terminationHandler = { proc in
            DispatchQueue.main.async {
                EngineController.instance.reloadSettings()
                log.debug("Run helper exit code: \(proc.terminationStatus)")
            }
        }

        do {
            try process.run()
            // process.waitUntilExit()
            // EngineController.instance.reloadSettings()
            // log.debug("Run helper exit code:\(process.terminationStatus)")
        } catch {
            log.debug("Run helper error:\(error)")
        }
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
