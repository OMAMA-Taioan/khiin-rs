package be.chiahpa.khiin.keyboard

import android.view.KeyEvent
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import be.chiahpa.khiin.keyboard.components.KeyPosition
import be.chiahpa.khiin.utils.ImmutableList

interface KeyboardLayoutScope {
    fun row(content: KeyboardRowScope.() -> Unit)
    var rowHeight: Dp
}

class KeyboardLayoutScopeImpl : KeyboardLayoutScope {
    val rows: MutableList<KeyboardRowData> = mutableListOf()
    override var rowHeight = 0.dp

    override fun row(content: KeyboardRowScope.() -> Unit) {
        val scopeContent = KeyboardRowScopeImpl().apply(content)
        rows.add(KeyboardRowData(scopeContent.keys))
    }

    fun toImmutable(): ImmutableList<ImmutableList<KeyData>> =
        ImmutableList(rows.map { row ->
            ImmutableList(row.keys.toList())
        }.toList())
}

interface KeyboardRowScope {
    fun alpha(
        label: String,
        weight: Float = 1f,
        position: KeyPosition = KeyPosition.FULL_WEIGHT
    )

    fun shift(weight: Float = 1f)

    fun backspace(weight: Float = 1f)

    fun symbols(weight: Float = 1f)

    fun spacebar(weight: Float = 1f)

    fun enter(weight: Float = 1f)
}

class KeyboardRowScopeImpl : KeyboardRowScope {
    val keys: MutableList<KeyData> = mutableListOf()

    override fun alpha(label: String, weight: Float, position: KeyPosition) {
        keys.add(
            KeyData(
                weight = weight,
                label = label,
                type = KeyType.LETTER,
                position = position
            )
        )
    }

    override fun shift(weight: Float) {
        keys.add(KeyData(weight = weight, type = KeyType.SHIFT))
    }

    override fun backspace(weight: Float) {
        keys.add(KeyData(weight = weight, type = KeyType.BACKSPACE))
    }

    override fun symbols(weight: Float) {
        keys.add(KeyData(weight = weight, type = KeyType.SYMBOLS))
    }

    override fun spacebar(weight: Float) {
        keys.add(KeyData(weight = weight, type = KeyType.SPACEBAR))
    }

    override fun enter(weight: Float) {
        keys.add(KeyData(weight = weight, type = KeyType.ENTER))
    }
}