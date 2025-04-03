import androidx.compose.foundation.layout.*
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.rmp.bar.ViewModel
import uniffi.bar.Action

@Composable
fun Counter(viewModel: ViewModel) {
        val context = LocalContext.current
        val counter = viewModel.count.collectAsState().value

        Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.Center
                ) {
                        Button(
                                onClick = { viewModel.action(Action.DECREMENT) },
                                colors = ButtonDefaults.buttonColors(containerColor = Color.Red),
                                modifier =
                                        Modifier.size(64.dp).testTag("decrementButton").semantics {
                                                contentDescription = "decrementButton"
                                        }
                        ) { Text("-", color = Color.White, fontSize = 40.sp) }

                        Text(
                                text = "${counter}",
                                fontSize = 32.sp,
                                modifier =
                                        Modifier.padding(horizontal = 16.dp)
                                                .testTag("counterValue")
                                                .semantics { contentDescription = "counterValue" }
                        )

                        Button(
                                onClick = { viewModel.action(Action.INCREMENT) },
                                colors = ButtonDefaults.buttonColors(containerColor = Color.Green),
                                modifier =
                                        Modifier.size(64.dp).testTag("incrementButton").semantics {
                                                contentDescription = "incrementButton"
                                        }
                        ) { Text("+", color = Color.White, fontSize = 32.sp) }
                }
        }
}
