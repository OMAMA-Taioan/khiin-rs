import SwiftUI
import UIKit

class KeyboardViewController: UIInputViewController {
    override func viewDidLoad() {
        super.viewDidLoad()
        let vc = UIHostingController(rootView: MySwiftUIView())
        vc.view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        self.view.addSubview(vc.view)
    }
}

struct MySwiftUIView: View {
    @State var counter = 0
    
    var body: some View {
        VStack {
            Text("Counter: \(counter)")
                .font(.headline)
            Button(action: {
                counter += 1
            }) {
                Text("Increment")
            }
        }
        .padding()
    }
}
