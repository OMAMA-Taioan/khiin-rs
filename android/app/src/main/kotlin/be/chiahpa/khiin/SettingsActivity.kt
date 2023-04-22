package be.chiahpa.khiin

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import be.chiahpa.khiin.settings.SettingsScreen
import be.chiahpa.khiin.theme.KhiinTheme

class SettingsActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            KhiinTheme {
                SettingsScreen()
            }
        }
    }
}
