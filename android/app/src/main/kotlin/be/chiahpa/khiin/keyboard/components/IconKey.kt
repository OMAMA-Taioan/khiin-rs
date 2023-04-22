package be.chiahpa.khiin.keyboard.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.Image
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.size
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

@Composable
fun RowScope.IconKey(
    icon: Int,
    weight: Float = 1f,
    keyColor: Color = Color.Transparent,
    tint: Color = MaterialTheme.colorScheme.onSurface,
    cornerSize: Dp = 12.dp,
    onClick: () -> Unit = {}
) {
    BaseKey(
        weight = weight,
        keyColor = keyColor,
        cornerSize = cornerSize,
    ) {
        Image(
            painterResource(id = icon),
            null,
            modifier = Modifier.size(32.dp),
            colorFilter = ColorFilter.tint(tint)
        )
    }
}
