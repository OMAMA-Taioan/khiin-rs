import SwiftUI
import UIKit

class KeyboardViewController: UIInputViewController {
    var engine: EngineController?
    
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
        guard let dbFilePath = Bundle.main.path(forResource: "khiin", ofType: "db") else {
            return;
        }
        print("Found database: \(String(describing: dbFilePath))")
        self.engine = EngineController(dbFilePath)
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
        var req = Khiin_Proto_Request()
        var keyEvent = Khiin_Proto_KeyEvent()
        
        switch key.action{
        case .char(let c):
            req.type = .cmdSendKey
            keyEvent.keyCode = c
        default:
            req.type = .cmdUnspecified
        }
        
        req.keyEvent = keyEvent
        
        if let cmd = self.engine?.sendCommand(req) {
            print("Obtained response with \(cmd.response.candidateList.candidates.count) candidates")
        }
        
        print("Handling key: \(key.label)")
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
