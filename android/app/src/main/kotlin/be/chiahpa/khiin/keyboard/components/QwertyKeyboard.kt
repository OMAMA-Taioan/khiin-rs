package be.chiahpa.khiin.keyboard.components

import androidx.compose.runtime.Composable
import androidx.compose.ui.unit.Dp
import be.chiahpa.khiin.keyboard.KeyboardLayout
import be.chiahpa.khiin.keyboard.KeyboardViewModel

@Composable
fun QwertyKeyboard(viewModel: KeyboardViewModel, rowHeight: Dp) {
    KeyboardLayout(
        viewModel = viewModel
    ) {
        this.rowHeight = rowHeight

        row {
            alpha("q")
            alpha("w")
            alpha("e")
            alpha("r")
            alpha("t")
            alpha("y")
            alpha("u")
            alpha("i")
            alpha("o")
            alpha("p")
        }

        row {
            alpha("a", 1.5f, KeyPosition.ALIGN_RIGHT)
            alpha("s")
            alpha("d")
            alpha("f")
            alpha("g")
            alpha("h")
            alpha("j")
            alpha("k")
            alpha("l", 1.5f, KeyPosition.ALIGN_LEFT)
        }

        row {
            shift(1.5f)
            alpha("z")
            alpha("x")
            alpha("c")
            alpha("v")
            alpha("b")
            alpha("n")
            alpha("m")
            backspace(1.5f)
        }

        row {
            symbols(1.5f)
            alpha(",")
            spacebar(5f)
            alpha(".")
            enter(1.5f)
        }
    }
}