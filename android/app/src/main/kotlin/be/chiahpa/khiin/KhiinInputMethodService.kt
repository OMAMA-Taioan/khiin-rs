package be.chiahpa.khiin

import android.inputmethodservice.InputMethodService
import android.view.View
import android.view.inputmethod.EditorInfo
import be.chiahpa.khiin.keyboard.ComposeKeyboardView
import be.chiahpa.khiin.service.KhiinServiceLifecycleOwner
import be.chiahpa.khiin.service.copyAssetToFiles
import be.chiahpa.khiin.utils.loggerFor
import java.io.File

private val logd = loggerFor("KhiinInputMethodService")

class KhiinInputMethodService : InputMethodService() {
    private val lifecycleOwner = KhiinServiceLifecycleOwner()
    private lateinit var dbPath: String

    override fun onCreate() {
        super.onCreate()
        lifecycleOwner.onCreate()

        copyAssetToFiles(this, "khiin.db")
        dbPath = File(filesDir, "khiin.db").absolutePath
    }

    override fun onCreateInputView(): View {
        val decorView = window?.window?.decorView
        lifecycleOwner.attachToDecorView(decorView)
        return ComposeKeyboardView(this, dbPath)
    }

    override fun onStartInputView(info: EditorInfo?, restarting: Boolean) {
        lifecycleOwner.onResume()
        window
    }

    override fun onFinishInputView(finishingInput: Boolean) {
        lifecycleOwner.onPause()
    }

    override fun onDestroy() {
        super.onDestroy()
        lifecycleOwner.onDestroy()
    }
}
