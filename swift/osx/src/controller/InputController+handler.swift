import InputMethodKit

extension KhiinInputController {
    override func recognizedEvents(_ sender: Any!) -> Int {
        let masks: NSEvent.EventTypeMask = [.keyDown]
        return Int(masks.rawValue)
    }

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        let modifiers = event.modifierFlags
        let changeInputMode = modifiers.contains(.option) && event.keyCode.representative == .punctuation("`")
        let shouldIgnoreCurrentEvent: Bool =
            !changeInputMode && (modifiers.contains(.command) || modifiers.contains(.option))
        
        guard !shouldIgnoreCurrentEvent else { return false }
        guard let client: IMKTextInput = sender as? IMKTextInput else {
            return false
        }
        currentOrigin = client.position

        log.debug("Current origin: \(String(describing: currentOrigin))")

        let currentClientID = currentClient?.uniqueClientIdentifierString()
        let clientID = client.uniqueClientIdentifierString()
        if clientID != currentClientID {
            currentClient = client
        }

        if (changeInputMode) {
            self.candidateViewModel.changeInputMode()
            self.reset()
            client.clearMarkedText()
            return true
        }

        switch event.keyCode.representative {
            case .alphabet(var char):
                if (char == "n" && modifiers.contains(.shift) && self.isManualMode()) {
                    char = "N"
                }
                self.candidateViewModel.handleChar(char)
                if (self.isCommited()) {
                    client.insert(self.currentDisplayText())
                    self.reset()
                } else {
                    self.resetWindow()
                    client.mark(self.currentDisplayText())
                }
                return true
            case .number(let num):
                self.candidateViewModel.handleChar(String(num))
                self.resetWindow()
                return true
            default:
                log.debug("key is special key")
        }

        if (!self.isEdited()) {
            return false
        }

        switch event.keyCode.representative {
            case .enter:
                let committed = self.commitCurrent()
                self.candidateViewModel.reset()
                return committed
            case .backspace:
                self.candidateViewModel.handleBackspace()
            case .escape:
                self.reset()
                client.clearMarkedText()
                return true
            case .space:
                self.candidateViewModel.handleSpace()
            case .arrow(Direction.up):
                self.candidateViewModel.handleArrowUp()
            case .arrow(Direction.down):
                self.candidateViewModel.handleArrowDown()
            default:
                log.debug("Event not handled")
                self.resetWindow()
                return false
        }

        if (self.isEdited()) {
            self.resetWindow()
            client.mark(self.currentDisplayText())
        } else {
            self.reset()
            client.clearMarkedText()
        }
        return true 
    }
}
