package com.rmp.bar

import Counter
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Enable Compose testTag values to be accessible for Appium testing
        System.setProperty("compose.test.resource.id.mapping", "true")
        
        setContent { App() }
    }
}

@Composable
fun App() {
    val context = LocalContext.current
    val viewModel = ViewModel(context)
    Counter(viewModel)
}
