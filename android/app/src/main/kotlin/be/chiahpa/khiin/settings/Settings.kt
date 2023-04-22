package be.chiahpa.khiin.settings

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.floatPreferencesKey
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import be.chiahpa.khiin.Khiin
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

private const val PREFS_FILE = "settings"
private const val DEFAULT_COLOR_SCHEME = "0xFF007FB1"
private val KBD_ROW_HEIGHT = floatPreferencesKey("keyboard_row_height")
private val CANDIDATE_BAR_HEIGHT = floatPreferencesKey("candidate_bar_height")
private val COLOR_SCHEME = stringPreferencesKey("color_scheme")

object Settings {
    private val Context.settingsDataStore: DataStore<Preferences> by preferencesDataStore(
        PREFS_FILE
    )

    private val dataStore
        get() = Khiin.context.settingsDataStore

    suspend fun setRowHeight(heightDp: Float) {
        dataStore.edit {
            it[KBD_ROW_HEIGHT] = heightDp
        }
    }

    val rowHeightFlow: Flow<Float> = dataStore.data.map {
        it[KBD_ROW_HEIGHT] ?: 60f
    }

    val candidateBarHeight: Flow<Float> = dataStore.data.map {
        it[CANDIDATE_BAR_HEIGHT] ?: 60f
    }

    val colorScheme: Flow<String> = dataStore.data.map {
        it[COLOR_SCHEME] ?: DEFAULT_COLOR_SCHEME
    }

    val defaultColorScheme: String
        get() = DEFAULT_COLOR_SCHEME
}
