package be.chiahpa.khiin.keyboard

import androidx.compose.material3.ColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.ui.graphics.Color

data class KeyboardColorScheme(
    val background: Color,
    val key: Color,
    val label: Color,
    val accentKey: Color,
    val accentLabel: Color,
    val actionKey: Color,
    val actionLabel: Color
)

fun KeyboardColorScheme.toLightColorScheme(): ColorScheme {
    return lightColorScheme(
        background = background,
    )
}
