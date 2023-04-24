import SwiftUI
import UIKit

import KhiinBridge

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
        if let engine = EngineController.new(dbFilePath) {
            self.engine = engine
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
        
        guard let engine = self.engine else {
            return
        }
        
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
        
        let bytes: Data? = {
            do {
                var cmd = Khiin_Proto_Command()
                cmd.request = req
                let data = try cmd.serializedData()
                return data
            } catch {
                return nil
            }
        }()
        
        guard let bytes = bytes else {
            return
        }
        
        var lenOutput: UInt = 0
        var cmdOutput = UnsafeMutablePointer<UInt8>.allocate(capacity: 0)
        
        let result = bytes.withUnsafeBytes {
            (cmdInput: UnsafeRawBufferPointer) -> Int32 in
                return engine.sendCommand(
                    (cmdInput.baseAddress?.assumingMemoryBound(to: UInt8.self))!,
                    UInt(bytes.count),
                    &cmdOutput,
                    &lenOutput
                )
        }
        
        if result != 0 {
            print("Could not read result from engine")
            return
        }
        
        let outputData = Data(
            bytesNoCopy: cmdOutput,
            count: Int(lenOutput),
            deallocator: .custom({ (pointer, size) in
                pointer.deallocate()
            })
        )
        
        let cmdResponse: Khiin_Proto_Command? = {
            do {
                let res = try Khiin_Proto_Command.init(serializedData: outputData)
                return res
            } catch {
                print("Unable to decode bytes from engine")
                return nil
            }
        }()
        
        guard let cmd = cmdResponse else {
            return
        }
        
        print("Obtained response with \(cmd.response.candidateList.candidates.count) candidates")
        
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
