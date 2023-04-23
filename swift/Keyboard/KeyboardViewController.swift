import SwiftUI
import UIKit

class KeyboardViewController: UIInputViewController {
    override func viewDidLoad() {
        super.viewDidLoad()
        print("Loaded")
        guard let inputView = self.inputView else {
            return
        }

        let viewModel = KeyboardViewModel(document: self.textDocumentProxy)
        let vc = UIHostingController(
            rootView: KeyboardWrapperView(viewModel)
        )
        vc.view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        vc.view.translatesAutoresizingMaskIntoConstraints = false

        self.addChild(vc)
        self.view.addSubview(vc.view)
        vc.didMove(toParent: self)

        NSLayoutConstraint.activate([
            vc.view.topAnchor.constraint(equalTo: self.view.topAnchor),
            vc.view.rightAnchor.constraint(equalTo: self.view.rightAnchor),
            vc.view.bottomAnchor.constraint(equalTo: self.view.bottomAnchor),
            vc.view.leftAnchor.constraint(equalTo: self.view.leftAnchor),
        ])
                
        inputView.backgroundColor = .clear
    }
}
