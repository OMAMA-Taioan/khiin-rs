package be.chiahpa.khiin

import be.chiahpa.khiin.utils.loggerFor
import khiin.proto.Command
import khiin.proto.CommandType
import khiin.proto.Request
import khiin.proto.Response
import khiin.proto.command
import khiin.proto.request

const val KHIIN_ANDROID_NATIVE_LIBRARY = "khiin_droid"

val log = loggerFor("EngineManager")

object EngineManager {
    init {
        System.loadLibrary(KHIIN_ANDROID_NATIVE_LIBRARY)
    }

    fun startup(dbPath: String) {
        if (enginePtr != 0L) {
            reset()
        } else {
            dbFileName = dbPath
            enginePtr = load(dbFileName)
        }
    }

    fun sendCommand(req: Request): Command {
        val res = sendCommand(enginePtr, req.toByteArray())
        return Command.parseFrom(res)
    }

    fun reset() {
        val req = request { type = CommandType.CMD_RESET }
        sendCommand(req)
    }

    fun shutdown() {
        log("Khiin engine shutting down...")
        shutdown(enginePtr)
    }

    private external fun load(dbFileName: String): Long

    private external fun sendCommand(enginePtr: Long, cmdBytes: ByteArray): ByteArray

    private external fun shutdown(enginePtr: Long)

    private var dbFileName: String = ""
    private var enginePtr: Long = 0
}
