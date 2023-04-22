package be.chiahpa.khiin.keyboard.components

import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.boundsInParent
import androidx.compose.ui.layout.boundsInRoot
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import be.chiahpa.khiin.utils.loggerFor

private val log = loggerFor("BaseKey")

enum class KeyPosition {
    FULL_WEIGHT,
    ALIGN_RIGHT,
    ALIGN_LEFT
}

@Composable
fun RowScope.BaseKey(
    weight: Float = 1f,
    keyColor: Color = Color.Transparent,
    keyPosition: KeyPosition = KeyPosition.FULL_WEIGHT,
    cornerSize: Dp = 12.dp,
    onTouchTargetPositioned: (Rect) -> Unit = {},
    onKeyPositioned: (Rect) -> Unit = {},
    content: @Composable () -> Unit
) {
    Box(
        contentAlignment = Alignment.Center,
        modifier = Modifier
            .weight(weight)
            .fillMaxHeight()
            .onGloballyPositioned {
                onTouchTargetPositioned(it.boundsInRoot())
            }

    ) {
        Row(
            modifier = Modifier.fillMaxSize()
        ) {
            val keyWeight = 1f / weight
            val spacerWeight = 1f - keyWeight

            if (weight != 1f && keyPosition == KeyPosition.ALIGN_RIGHT) {
                Spacer(modifier = Modifier.weight(spacerWeight))
            }

            Box(
                contentAlignment = Alignment.Center,
                modifier = Modifier
                    .fillMaxSize()
                    .weight(keyWeight)
                    .padding(4.dp, 8.dp)
                    .clip(RoundedCornerShape(cornerSize))
                    .background(keyColor)
                    .onGloballyPositioned {
                        onKeyPositioned(it.boundsInRoot())
                    }
            ) {
                content()
            }

            if (weight != 1f && keyPosition == KeyPosition.ALIGN_LEFT) {
                Spacer(modifier = Modifier.weight(spacerWeight))
            }
        }
    }
}
