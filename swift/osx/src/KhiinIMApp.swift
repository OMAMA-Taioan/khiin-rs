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
    // The server has to outlive applicationDidFinishLaunching. It owns the
    // connection named in Info.plist, and once it is released macOS no longer
    // recognises that name: the input method stays selectable but never gets
    // an input session, so nothing can be typed with it.
    private var server: IMKServer?

    // One monitor for the whole process. It used to be installed by every
    // input controller and never removed, so a single click reached every
    // controller that had ever been created, deallocated ones included.
    private var mouseMonitor: Any?

    func applicationDidFinishLaunching(_ notification: Notification) {
        Logger.setup()

        let name =
            Bundle.main.infoDictionary?["InputMethodConnectionName"] as? String
        let identifier = Bundle.main.bundleIdentifier
        self.server = IMKServer(name: name, bundleIdentifier: identifier)

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
