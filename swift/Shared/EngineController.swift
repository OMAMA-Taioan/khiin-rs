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

    func handleChar(_ charCode: Int32) -> Khiin_Proto_Command? {
        guard let engine = self.engine else {
            return nil
        }

        var req = Khiin_Proto_Request()
        var keyEvent = Khiin_Proto_KeyEvent()

        req.type = .cmdSendKey
        keyEvent.keyCode = charCode
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
            return nil
        }

        let result: RustVec<UInt8>? = bytes.withUnsafeBytes {
            (ptr: UnsafeRawBufferPointer) -> RustVec<UInt8>? in
            let bufferPointer = ptr.bindMemory(to: UInt8.self)
            return engine.sendCommand(bufferPointer)
        }

        guard let result = result else {
            print("No result from engine")
            return nil
        }

        let resultData = Data(
            bytes: result.as_ptr(),
            count: result.len()
        )

        let cmdResponse: Khiin_Proto_Command? = {
            do {
                let res = try Khiin_Proto_Command.init(
                    serializedData: resultData)
                return res
            } catch {
                print("Unable to decode bytes from engine")
                return nil
            }
        }()

        guard let cmd = cmdResponse else {
            return nil
        }

        print(
            "Obtained response with \(cmd.response.candidateList.candidates.count) candidates"
        )
        return cmd
    }
}
