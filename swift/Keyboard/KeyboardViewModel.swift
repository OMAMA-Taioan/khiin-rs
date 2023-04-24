import SwiftUI

class KeyboardViewModel: ObservableObject {
    let controller: KeyboardViewController
    
    init(controller: KeyboardViewController) {
        self.controller = controller
    }
    
    func handleKey(_ key: Key) {
        print("Key pressed: \(key.label)")
    }
}
