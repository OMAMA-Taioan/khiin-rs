import SwiftUI

class KeyboardViewModel: ObservableObject {
    let document: UITextDocumentProxy
    
    init(document: UITextDocumentProxy) {
        self.document = document
    }
    
    func handleKey(_ key: Key) {
        print("Key pressed: \(key.label)")
    }
}
