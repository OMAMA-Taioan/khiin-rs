import InputMethodKit

extension KhiinInputController {
    override func recognizedEvents(_ sender: Any!) -> Int {
        let masks: NSEvent.EventTypeMask = [.keyDown]
        return Int(masks.rawValue)
    }

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        let modifiers = event.modifierFlags
        let shouldIgnoreCurrentEvent: Bool =
            modifiers.contains(.command) || modifiers.contains(.option)
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

        //        let shiftPressed = modifiers == .shift
        var isHandled = false
        switch event.keyCode.representative {
        case .alphabet(let char):
            self.candidateViewModel.handleChar(char)
            self.resetWindow()
            client.mark(self.currentDisplayText())
            return true
        case .number(let num):
            self.candidateViewModel.handleChar(String(num))
            self.resetWindow()
            return true
        case .enter:
            let committed = self.commitCurrent()
            self.candidateViewModel.reset()
            return committed
        case .backspace:
            self.candidateViewModel.handleBackspace()
            isHandled = true
        case .escape:
            self.reset()
            client.clearMarkedText()
            return true
        case .space:
            self.candidateViewModel.handleSpace()
            isHandled = true
        case .arrow(Direction.up):
            self.candidateViewModel.handleArrowUp()
            isHandled = true
        case .arrow(Direction.down):
            self.candidateViewModel.handleArrowDown()
            isHandled = true
        default:
            log.debug("Event not handled")
        }

        if (isHandled) {
            if (self.isEdited()) {
                self.resetWindow()
                client.mark(self.currentDisplayText())
                return true
            } else {
                self.reset()
                client.clearMarkedText()
                return false
            }
        }

        self.resetWindow()

        return false
    }
}
