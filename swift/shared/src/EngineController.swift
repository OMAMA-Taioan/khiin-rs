import KhiinBridge
import SwiftUI
import SwiftyBeaver

let log = SwiftyBeaver.self

struct Constants {
    static let dbFile: String = "khiin"
    static let dbFileExt: String = "db"
}

public class EngineController {
    public static let instance = EngineController()

    private let engine: EngineBridge?
    private var config: Khiin_Proto_AppConfig?

    init() {

        guard let dbpath = getDatabaseFilePath() else {
            self.config = nil
            self.engine = nil
            return
        }

        guard let settingsPath = getSettingFilePath() else {
            self.config = nil
            self.engine = nil
            return
        }

        guard let engine = EngineBridge.new(dbpath) else {
            log.debug("No engine")
            self.config = nil
            self.engine = nil
            return
        }

        guard let settings = engine.loadSettings(settingsPath) else {
            log.debug("No setting data loaded from engine")
            self.config = nil
            self.engine = nil
            return
        }

        let settingData = Data(
            bytes: settings.as_ptr(),
            count: settings.len()
        )

        guard
            let config = try? Khiin_Proto_AppConfig.init(
                serializedData: settingData
            )
        else {
            log.debug("Unable to decode config from engine")
            self.config = nil
            self.engine = nil
            return
        }

        self.config = config
        self.engine = engine
    }

    public func reset() {
        var req = Khiin_Proto_Request()
        req.type = .cmdReset
        let _ = sendCommand(req)
    }

    public func reloadSettings() {
        guard let settingsPath = getSettingFilePath() else {
            return
        }

        guard let settings = self.engine?.loadSettings(settingsPath) else {
            log.debug("No setting data loaded from engine")
            return
        }

        let settingData = Data(
            bytes: settings.as_ptr(),
            count: settings.len()
        )

        guard
            let config = try? Khiin_Proto_AppConfig.init(
                serializedData: settingData
            )
        else {
            log.debug("Unable to decode config from engine")
            return
        }

        self.config = config
        log.debug("reload settings, now input mode : \(String(describing:self.config?.inputMode))")
    }

    public func hyphenKey() -> String {
        if (self.config == nil) {
            return ""
        }
        return self.config?.keyConfig.altHyphen ?? ""
    }

    public func isManualMode() -> Bool {
        if (self.config == nil) {
            return false
        }
        return self.config?.inputMode == .manual
    }

    public func changeInputMode() -> Khiin_Proto_Command? {
        if (self.config == nil) {
            log.debug("Config not instantiated")
            return nil
        }
        
        if (self.config?.inputMode == .continuous) {
            self.config?.inputMode = .manual;
        } else {
            self.config?.inputMode = .continuous
        }

        var req = Khiin_Proto_Request()
        req.type = .cmdSwitchInputMode
        req.config = self.config!
        
        return sendCommand(req)
    }

    public func handleChar(_ char: Character) -> Khiin_Proto_Command? {
        guard let charCode = char.asciiValue else {
            return nil
        }

        var req = Khiin_Proto_Request()
        var keyEvent = Khiin_Proto_KeyEvent()

        req.type = .cmdSendKey
        keyEvent.keyCode = Int32(charCode)
        req.keyEvent = keyEvent

        return sendCommand(req)
    }

    public func handleSpecialKey(_ key: Khiin_Proto_SpecialKey)
        -> Khiin_Proto_Command?
    {
        var req = Khiin_Proto_Request()
        var keyEvent = Khiin_Proto_KeyEvent()

        req.type = .cmdSendKey
        keyEvent.specialKey = key
        req.keyEvent = keyEvent

        return sendCommand(req)
    }

    public func sendCommand(_ request: Khiin_Proto_Request)
        -> Khiin_Proto_Command?
    {
        guard let engine = self.engine else {
            log.debug("Engine not instantiated")
            return nil
        }

        var cmd = Khiin_Proto_Command()
        cmd.request = request

        guard let bytes = try? cmd.serializedData() else {
            log.debug("Unable to serialize data")
            return nil
        }

        guard
            let result = bytes.withUnsafeBytes({
                (ptr: UnsafeRawBufferPointer) -> RustVec<UInt8>? in
                let bufferPointer = ptr.bindMemory(to: UInt8.self)
                return engine.sendCommand(bufferPointer)
            })
        else {
            log.debug("No result from engine")
            return nil
        }

        let resultData = Data(
            bytes: result.as_ptr(),
            count: result.len()
        )

        guard
            let cmdResponse = try? Khiin_Proto_Command.init(
                serializedData: resultData
            )
        else {
            log.debug("Unable to decode bytes from engine")
            return nil
        }

        let count = cmdResponse.response.candidateList.candidates.count
        log.debug("Obtained response with \(count) candidates")

        return cmdResponse
    }
}
