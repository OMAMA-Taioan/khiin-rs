import Foundation
import SwiftyBeaver

let log = SwiftyBeaver.self
let kLogFileName = "khiin_im.log"

class Logger {
    private static var isSetup: Bool = false

    static func setup() {
        guard !isSetup else {
            // Already done
            return
        }

        let logFileUrl = Logger.logFileUrl(filename: kLogFileName)
        let logFile = FileDestination(logFileURL: logFileUrl)
        log.addDestination(logFile)
        self.isSetup = true
    }

    static func logFileUrl(filename: String) -> URL? {
        let fileManager = FileManager.default

        // platform-dependent logfile directory default
        var baseURL: URL?
        if let url = fileManager.urls(
            for: .cachesDirectory, in: .userDomainMask
        ).first {
            baseURL = url
            // try to use ~/Library/Caches/APP NAME instead of ~/Library/Caches
            if let appName = Bundle.main.object(
                forInfoDictionaryKey: "CFBundleExecutable") as? String
            {
                do {
                    if let appURL = baseURL?.appendingPathComponent(
                        appName, isDirectory: true)
                    {
                        try fileManager.createDirectory(
                            at: appURL,
                            withIntermediateDirectories: true, attributes: nil)
                        baseURL = appURL
                    }
                } catch {
                    print(
                        "Warning! Could not create folder /Library/Caches/\(appName)"
                    )
                }
            }
        }

        if let baseURL = baseURL {
            return baseURL.appendingPathComponent(filename, isDirectory: false)
        }

        return nil
    }
}
