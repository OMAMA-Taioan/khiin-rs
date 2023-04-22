package be.chiahpa.khiin.keyboard

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.compose.collectAsStateWithLifecycle

@Composable
fun CandidatesBar(viewModel: KeyboardViewModel, height: Dp) {
    val candidateState by viewModel.candidateState.collectAsStateWithLifecycle()

    Row(
        horizontalArrangement = Arrangement.SpaceAround,
        modifier = Modifier
            .fillMaxWidth()
            .height(height)
    ) {
        when (val state = candidateState) {
            is CandidateState.Empty -> {
                Text("Show candidates here")
            }
            is CandidateState.Loaded -> {
                state.candidates.candidatesList.forEach {
                    Text(it.value, fontSize = 28.sp)
                }
            }
        }
    }
}
