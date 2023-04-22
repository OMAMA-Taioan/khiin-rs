package be.chiahpa.khiin.utils

fun loggerFor(tag: String): (String) -> Unit {
    return { android.util.Log.d(tag, it) }
}
