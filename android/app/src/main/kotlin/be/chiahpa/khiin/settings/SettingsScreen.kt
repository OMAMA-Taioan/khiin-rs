package be.chiahpa.khiin.settings

import android.content.Context.INPUT_METHOD_SERVICE
import android.content.Intent
import android.view.inputmethod.InputMethodManager
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Slider
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import be.chiahpa.khiin.keyboard.KeyboardViewModel
import be.chiahpa.khiin.utils.loggerFor
import khiin.proto.Command
import kotlinx.coroutines.launch

private val log = loggerFor("SettingsScreen")

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen(
) {
    val context = LocalContext.current

    var input by remember {
        mutableStateOf("")
    }

    val sliderPositionState  = Settings.rowHeightFlow.collectAsStateWithLifecycle(
        initialValue = 60f
    )

    val sliderPosition = sliderPositionState.value
    val coroutinescope = rememberCoroutineScope()

    Surface(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        color = MaterialTheme.colorScheme.background
    ) {
        Column {
            Button(onClick = {
                context.startActivity(Intent(android.provider.Settings.ACTION_INPUT_METHOD_SETTINGS))
            }) {
                Text(text = "Open Settings")
            }
            Button(onClick = {
                val imm = context.getSystemService(INPUT_METHOD_SERVICE) as InputMethodManager
                imm.showInputMethodPicker()
            }) {
                Text(text = "Select Input Method")
            }

            Spacer(modifier = Modifier.height(24.dp))

            Text(text = "Type here to test the keyboard:")
            OutlinedTextField(
                modifier = Modifier.fillMaxWidth(),
                value = input,
                label = { Text("Test Input") },
                onValueChange = { input = it }
            )

            Spacer(modifier = Modifier.height(24.dp))

            Text(text = "Keyboard height")
            Slider(
                value = sliderPosition,
                onValueChange = {
                    coroutinescope.launch {
                        Settings.setRowHeight(it)
                    }
                },
                valueRange = 48f..72f,
            )
            TextButton(onClick = {
                coroutinescope.launch {
                    Settings.setRowHeight(60f)
                }
            }) {
                Text("Reset")
            }
        }
    }
}
