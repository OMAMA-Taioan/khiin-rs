package be.chiahpa.khiin.keyboard.components

import androidx.compose.foundation.layout.RowScope
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.sp
import be.chiahpa.khiin.utils.loggerFor

private val log = loggerFor("LetterKey")

@Composable
fun RowScope.LetterKey(
    label: String,
    weight: Float = 1f,
    fontSize: TextUnit = 28.sp,
    textColor: Color = MaterialTheme.colorScheme.onSurface,
    keyColor: Color = Color.Transparent,
    keyPosition: KeyPosition = KeyPosition.FULL_WEIGHT,
    onTouchTargetPositioned: (Rect) -> Unit = {},
    onKeyPositioned: (Rect) -> Unit = {},
) {
    BaseKey(
        weight = weight,
        keyColor = keyColor,
        keyPosition = keyPosition,
        onTouchTargetPositioned = onTouchTargetPositioned,
        onKeyPositioned = onKeyPositioned
    ) {
        Text(
            text = label,
            fontSize = fontSize,
            color = textColor
        )
    }
}
