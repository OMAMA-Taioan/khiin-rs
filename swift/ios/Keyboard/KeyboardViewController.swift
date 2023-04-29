import SwiftUI
import SwiftyBeaver
import UIKit
import KhiinSwift

let log = SwiftyBeaver.self

class KeyboardViewController: UIInputViewController {
    let engine = EngineController.instance
    
    override func viewDidLoad() {
        super.viewDidLoad()
        self.setupInitialWidth()
        log.addDestination(ConsoleDestination())
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
        log.debug("Handling key: \(key.label)")

        switch key.action {
        case .char(let c):
            let _ = self.engine.handleChar(c)
        default:
            log.debug("Not a char")
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
