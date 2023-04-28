import KhiinBridge
import SwiftUI

struct Constants {
    static let dbFile: String = "khiin"
    static let dbFileExt: String = "db"
}

class EngineController {
    static let instance = EngineController()

    static func getDbPath() -> String? {
        Bundle.main.path(
            forResource: Constants.dbFile, ofType: Constants.dbFileExt)
    }

    private let engine: EngineBridge?

    init() {
        guard let dbpath = EngineController.getDbPath() else {
            self.engine = nil
            return
        }

        guard let engine = EngineBridge.new(dbpath) else {
            self.engine = nil
            return
        }

        self.engine = engine
    }

    func reset() {
        guard let engine = self.engine else {
            return
        }

        var req = Khiin_Proto_Request()
        req.type = .cmdReset
        let _ = sendCommand(req)
    }

    func handleChar(_ char: Character) -> Khiin_Proto_Command? {
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

    func sendCommand(_ request: Khiin_Proto_Request) -> Khiin_Proto_Command? {
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
