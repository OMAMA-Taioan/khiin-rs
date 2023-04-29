import SwiftUI

struct ContentView: View {
    @State private var text = ""
    
    var body: some View {
        VStack {
            Text("Hello World")
            
            TextField("Enter text", text: self.$text)
                .textFieldStyle(RoundedBorderTextFieldStyle())
            
            Text("You entered: \(self.text)")
        }
    }
}
