import Foundation

class EngineController {
    let dbfile: String
    let engine_ptr: UnsafeMutableRawPointer
    
    init(_ dbfile: String) {
        self.dbfile = dbfile
        let dbfileCString = self.dbfile.cString(using: .utf8)
        self.engine_ptr = Rust_khiin_engine_load(dbfileCString)
        print("Engine loaded with pointer")
    }
    
    func sendCommand(_ request: Khiin_Proto_Request) -> Khiin_Proto_Command? {
        var cmd = Khiin_Proto_Command()
        cmd.request = request

        let bytes: Data? = {
            do {
                let data = try cmd.serializedData()
                return data
            } catch {
                print("Unable to serialize data")
                return nil
            }
        }()
        
        guard let bytes = bytes else {
            return nil
        }
        
        var lenOutput: UInt = 0
        var cmdOutput: UnsafeMutablePointer<UInt8>?
        
        let result = bytes.withUnsafeBytes {
            (cmdInput: UnsafeRawBufferPointer) -> Int32 in
            return Rust_khiin_engine_send_command(
                self.engine_ptr,
                cmdInput.baseAddress?.assumingMemoryBound(to: UInt8.self),
                bytes.count,
                &cmdOutput,
                &lenOutput
            )
        }
        
        if result != 0 {
            print("Engine call failed")
            return nil
        }
        
        guard let cmdOutput = cmdOutput else {
            print("No output from engine")
            return nil
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
        
        guard let cmdResponse = cmdResponse else {
            print("Unable to decode bytes from engine")
            return nil
        }
        
        return cmdResponse
    }
}
