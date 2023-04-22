package be.chiahpa.khiin.keyboard

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Rect
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import be.chiahpa.khiin.EngineManager
import be.chiahpa.khiin.utils.loggerFor
import khiin.proto.CandidateList
import khiin.proto.CommandType
import khiin.proto.Request
import khiin.proto.keyEvent
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

private val log = loggerFor("KeyboardViewModel")

sealed class CandidateState {
    object Empty : CandidateState()
    class Loaded(val candidates: CandidateList) : CandidateState()
}

sealed class KeyHintState {
    object None : KeyHintState()
    class Showing(val key: KeyData, val bounds: Rect) : KeyHintState()
}

data class KeyBounds(
    val touchTarget: Rect = Rect.Zero,
    val key: Rect = Rect.Zero
)

typealias KeyCoordinateMap = Map<KeyData, KeyBounds>

internal fun KeyCoordinateMap.keyAt(offset: Offset): KeyData? {
    this.forEach { (key, bounds) ->
        if (bounds.touchTarget.contains(offset)) {
            return key
        }
    }

    return null
}

class KeyboardViewModel(dbPath: String) : ViewModel() {
    init {
        EngineManager.startup(dbPath)
    }

    override fun onCleared() {
        super.onCleared()
        EngineManager.shutdown()
    }

    private val _candidateState =
        MutableStateFlow<CandidateState>(CandidateState.Empty)
    val candidateState = _candidateState.asStateFlow()

    private val _keyBounds = MutableStateFlow<KeyCoordinateMap>(mapOf())
    val keyBounds = _keyBounds.asStateFlow()

    private val _keyHintState =
        MutableStateFlow<KeyHintState>(KeyHintState.None)
    val keyHintState = _keyHintState.asStateFlow()

    fun sendKey(key: KeyData) {
        val req = Request.newBuilder()

        if (key.type == KeyType.LETTER && !key.label.isNullOrEmpty()) {
            req.apply {
                type = CommandType.CMD_SEND_KEY
                keyEvent = keyEvent {
                    keyCode = key.label[0].code
                }
            }
        }

        if (req.type != CommandType.CMD_UNSPECIFIED) {
            viewModelScope.launch {
                val res = EngineManager.sendCommand(req.build())
                if (res.response.candidateList.candidatesList.isNotEmpty()) {
                    _candidateState.value =
                        CandidateState.Loaded(res.response.candidateList)
                }
            }
        }
    }

    fun setKeyBounds(
        keyData: KeyData,
        touchTarget: Rect? = null,
        key: Rect? = null
    ) {
        val next = keyBounds.value.toMutableMap()

        if (touchTarget != null) {
            next[keyData] =
                next[keyData]?.copy(touchTarget = touchTarget)
                    ?: KeyBounds(touchTarget = touchTarget)
            _keyBounds.value = next
        }

        if (key != null) {
            next[keyData] =
                next[keyData]?.copy(key = key)
                    ?: KeyBounds(key = key)
        }

        _keyBounds.value = next
    }

    fun showKeyHint(key: KeyData, bounds: Rect) {
        _keyHintState.value = KeyHintState.Showing(key, bounds)
    }

    fun hideKeyHint() {
        _keyHintState.value = KeyHintState.None
    }
}
