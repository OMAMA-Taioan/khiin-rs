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
    func applicationDidFinishLaunching(_ notification: Notification) {
        Logger.setup()

        let name =
            Bundle.main.infoDictionary?["InputMethodConnectionName"] as? String
        let identifier = Bundle.main.bundleIdentifier
        let _ = IMKServer(name: name, bundleIdentifier: identifier)

        log.debug("IMKServer initialized")
    }

    func applicationWillTerminate(_ notification: Notification) {
        // empty
    }
}
