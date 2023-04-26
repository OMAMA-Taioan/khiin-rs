import SwiftUI
import UIKit

class KeyboardViewController: UIInputViewController {
    let engine = EngineController()

    override func viewDidLoad() {
        super.viewDidLoad()
        self.setupInitialWidth()
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        self.setup { controller in
            KeyboardWrapperView(
                controller: controller,
                width: self.view.frame.width
            )
        }
    }

    func setupInitialWidth() {
        self.view.frame.size.width = UIScreen.main.bounds.width
    }

    func setup<Content: View>(
        @ViewBuilder with rootView: @escaping (
            _ controller: KeyboardViewController
        ) -> Content
    ) {
        let view = KeyboardRootView { [unowned self] in rootView(self) }
        self.children.forEach { $0.removeFromParent() }
        self.view.subviews.forEach { $0.removeFromSuperview() }
        let host = KeyboardHostingController(rootView: view)
        host.add(to: self)
    }

    func handleKey(key: Key) {
        print("Handling key: \(key.label)")

        switch key.action {
        case .char(let c):
            self.engine.handleChar(c)
        default:
            print("Not a char")
        }

        self.textDocumentProxy.insertText(key.label)
    }
}

struct KeyboardRootView<ViewType: View>: View {
    init(@ViewBuilder _ view: @escaping () -> ViewType) {
        self.view = view
    }

    var view: () -> ViewType

    var body: some View {
        self.view()
    }
}
