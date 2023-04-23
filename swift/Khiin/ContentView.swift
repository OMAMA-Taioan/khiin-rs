import SwiftUI

struct ContentView: View {
    @State private var text = ""
    
    var body: some View {
        VStack {
            Text(bytesToString())
            
            TextField("Enter text", text: self.$text)
                .textFieldStyle(RoundedBorderTextFieldStyle())
            
            Text("You entered: \(self.text)")
        }
    }
}

func bytesToString() -> String {
    let byteArr: [UInt8] = [72, 101, 108, 108, 111]
    return byteArr.withUnsafeBufferPointer { bufferPointer -> String in
        if let baseAddress = bufferPointer.baseAddress,
           let cString = rust_bytes_to_string(baseAddress, bufferPointer.count) {
            let string = String(cString: cString)
            rust_string_free(UnsafeMutablePointer(mutating: cString))
            return string
        } else {
            return "Error"
        }
    }
}
