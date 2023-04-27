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
    var server = IMKServer()
    var candidateWindow = IMKCandidates()
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        Logger.setup()
        
        self.server = IMKServer(
            name: Bundle.main
                .infoDictionary?["InputMethodConnectionName"] as? String,
            bundleIdentifier: Bundle.main.bundleIdentifier)
        
        self.candidateWindow = IMKCandidates(
            server: self.server,
            panelType: kIMKSingleRowSteppingCandidatePanel,
            styleType: kIMKMain
        )
        
        log.debug("Tried connection")
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        // empty
    }
}
