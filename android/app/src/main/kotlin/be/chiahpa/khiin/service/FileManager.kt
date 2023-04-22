package be.chiahpa.khiin.service

import android.content.Context
import android.util.Log
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.io.InputStream
import java.io.OutputStream

@Throws(IOException::class)
private fun copyFile(inputStream: InputStream, outputStream: OutputStream) {
    val buffer = ByteArray(8192)
    var read: Int
    while (inputStream.read(buffer).also { read = it } != -1) {
        outputStream.write(buffer, 0, read)
    }
}

fun copyAssetToFiles(
    context: Context,
    filename: String,
    overwrite: Boolean = false
) {
    if (context.assets.list("")?.contains(filename) != true) {
        return
    }

    val outFile = File(context.filesDir, filename)

    if (outFile.exists() && !overwrite) {
        return
    }

    val instream = context.assets.open(filename)
    val outstream = FileOutputStream(outFile)

    try {
        copyFile(instream, outstream)
    } catch (e: Exception) {
        Log.e("copyfile", "Could not copy $filename", e)
    }

    instream.close()
    outstream.flush()
    outstream.close()
}
