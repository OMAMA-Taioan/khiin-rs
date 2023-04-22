package be.chiahpa.khiin.keyboard

import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.detectDragGesturesAfterLongPress
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.boundsInRoot
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.unit.Dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import be.chiahpa.khiin.utils.loggerFor

private val log = loggerFor("KeyboardTouchDelegate")

@Composable
fun KeyboardTouchDelegate(viewModel: KeyboardViewModel, totalHeight: Dp) {
    val keyBounds by viewModel.keyBounds.collectAsStateWithLifecycle()
    var currentKey by remember { mutableStateOf(KeyData()) }
    var currentOffset by remember { mutableStateOf(Offset.Zero) }
    var heightOffset by remember { mutableStateOf(0f) }
    val correct: (Offset) -> Offset = { it.copy(y = it.y + heightOffset) }

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .height(totalHeight)
            .onGloballyPositioned {
                heightOffset = it.boundsInRoot().top
            }
            .pointerInput(Unit) {
                detectTapGestures(
                    onPress = {
                        currentOffset = correct(it)
                        keyBounds
                            .keyAt(currentOffset)
                            ?.let { key ->
                                currentKey = key
                                viewModel.showKeyHint(key, keyBounds[key]!!.key)
                            }
                    },
                    onTap = {
                        viewModel.sendKey(currentKey)
                        viewModel.hideKeyHint()
                    }
                )
            }
            .pointerInput(Unit) {
                detectDragGestures(
                    onDrag = { _, dragAmount ->
                        currentOffset += dragAmount
                        keyBounds
                            .keyAt(currentOffset)
                            ?.let { key ->
                                if (key != currentKey) {
                                    currentKey = key
                                    viewModel.showKeyHint(
                                        key,
                                        keyBounds[key]!!.key
                                    )
                                }
                            }
                    },
                    onDragEnd = {
                        viewModel.sendKey(currentKey)
                        viewModel.hideKeyHint()
                    }
                )
            }
            .pointerInput(Unit) {
                detectDragGesturesAfterLongPress(
                    onDragStart = {
                        currentOffset = correct(it)
                        viewModel.hideKeyHint()
                        keyBounds
                            .keyAt(currentOffset)
                            ?.let { key ->
                                currentKey = key
                                log("Long pressed key: ${key.label}")
                            }
                    },
                    onDrag = { _, dragAmount ->
                        currentOffset += dragAmount
                        keyBounds
                            .keyAt(currentOffset)
                            ?.let { key ->
                                if (key != currentKey) {
                                    currentKey = key
                                    log("Dragged to key: ${key.label}")
                                }
                            }
                    }
                )
            }
    )
}
