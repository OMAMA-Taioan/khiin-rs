import SwiftUI

class KeyboardViewModel: ObservableObject {
    let controller: KeyboardViewController
    
    init(controller: KeyboardViewController) {
        self.controller = controller
    }
    
    func handleKey(_ key: Key) {
        log.debug("Key pressed: \(key.label)")
    }
}
