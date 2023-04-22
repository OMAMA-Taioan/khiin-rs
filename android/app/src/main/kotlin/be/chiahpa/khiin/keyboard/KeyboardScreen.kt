package be.chiahpa.khiin.keyboard

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.detectDragGesturesAfterLongPress
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntRect
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.LayoutDirection
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.round
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.Popup
import androidx.compose.ui.window.PopupPositionProvider
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import be.chiahpa.khiin.keyboard.components.QwertyKeyboard
import be.chiahpa.khiin.settings.Settings
import be.chiahpa.khiin.utils.loggerFor
import kotlin.math.roundToInt

private val log = loggerFor("KeyboardScreen")

@Composable
fun KeyboardScreen(
    viewModel: KeyboardViewModel
) {
    val rowHeightDp by Settings.rowHeightFlow.collectAsStateWithLifecycle(
        initialValue = 60f
    )

    val candidateBarHeight by Settings.candidateBarHeight.collectAsStateWithLifecycle(
        initialValue = 60f
    )

    val keyHintState by viewModel.keyHintState.collectAsStateWithLifecycle()

    Box {
        Surface(
            Modifier
                .fillMaxWidth()
        ) {
            Column(Modifier.fillMaxWidth()) {
                CandidatesBar(
                    viewModel = viewModel,
                    height = candidateBarHeight.dp
                )
                QwertyKeyboard(
                    viewModel = viewModel,
                    rowHeight = rowHeightDp.dp
                )
            }
        }

        when (val state = keyHintState) {
            is KeyHintState.Showing -> {
                KeyHintPopup(state)
            }

            else -> {}
        }
    }
}

@Composable
fun KeyHintPopup(state: KeyHintState.Showing) {
    val width = with(LocalDensity.current) { state.bounds.width.toDp() }
    val height = with(LocalDensity.current) { state.bounds.height.toDp() }

    Popup(popupPositionProvider = object : PopupPositionProvider {
        override fun calculatePosition(
            anchorBounds: IntRect,
            windowSize: IntSize,
            layoutDirection: LayoutDirection,
            popupContentSize: IntSize
        ): IntOffset {
            return (state.bounds.topLeft + Offset(
                0f,
                -state.bounds.height
            )).round()
        }
    }) {
        Surface(
            modifier = Modifier
                .height(height * 2)
                .width(width)
                .clip(RoundedCornerShape(16.dp)),
            shadowElevation = 16.dp,
        ) {
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .background(Color.Transparent)
                    .padding(0.dp, 8.dp),
                contentAlignment = Alignment.TopCenter
            ) {
                Text(
                    text = state.key.label ?: "",
                    fontSize = 28.sp
                )
            }
        }
    }
}
