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

        switch event.keyCode.representative {
        case .alphabet(let char):
            self.candidateViewModel.handleChar(char)
            self.resetWindow()
            return true
        case .number(let num):
            self.candidateViewModel.handleChar(String(num))
            self.resetWindow()
            return true
        case .enter:
            let committed = self.commitCurrent()
            self.candidateViewModel.reset()
            return committed
        default:
            log.debug("Event not handled")
        }

        self.resetWindow()

        return false
    }
}
