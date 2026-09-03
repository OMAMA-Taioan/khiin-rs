import AppKit
import InputMethodKit
import SwiftyBeaver

final class KhiinIMApplication: NSApplication {
    private let appDelegate = AppDelegate()

    override init() {
        super.init()
        self.delegate = appDelegate
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        // No need for implementation
        fatalError("init(coder:) has not been implemented")
    }
}

@main
final class AppDelegate: NSObject, NSApplicationDelegate {
    // One monitor for the whole process. It used to be installed by every
    // input controller and never removed, so a single click reached every
    // controller that had ever been created, deallocated ones included.
    private var mouseMonitor: Any?

    func applicationDidFinishLaunching(_ notification: Notification) {
        Logger.setup()

        let name =
            Bundle.main.infoDictionary?["InputMethodConnectionName"] as? String
        let identifier = Bundle.main.bundleIdentifier
        let _ = IMKServer(name: name, bundleIdentifier: identifier)

        setupMouseEventMonitor()

        log.debug("IMKServer initialized")
    }

    // A click outside the input method moves the caret, so the composition in
    // progress has to be released. Only the controller holding the input
    // session has one to release.
    private func setupMouseEventMonitor() {
        self.mouseMonitor = NSEvent.addGlobalMonitorForEvents(
            matching: .leftMouseDown
        ) { _ in
            KhiinInputController.activeController?.resetController()
        }
    }

    func applicationWillTerminate(_ notification: Notification) {
        if let monitor = self.mouseMonitor {
            NSEvent.removeMonitor(monitor)
            self.mouseMonitor = nil
        }
    }
}
