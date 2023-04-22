package be.chiahpa.khiin

import android.app.Application
import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.preferencesDataStore

class Khiin: Application() {
    companion object {
        lateinit var context: Khiin
            private set
    }

    override fun onCreate() {
        super.onCreate()
        context = this
    }
}
