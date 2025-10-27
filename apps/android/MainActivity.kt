package com.transformer.demo

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class MainActivity : ComponentActivity() {
    
    private var transformer: TransformerBridge? = null
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        setContent {
            MaterialTheme {
                TransformerScreen()
            }
        }
    }
    
    override fun onDestroy() {
        super.onDestroy()
        transformer?.free()
    }
    
    @Composable
    fun TransformerScreen() {
        var status by remember { mutableStateOf("Not initialized") }
        var backend by remember { mutableStateOf("") }
        var inputText by remember { mutableStateOf("1, 2, 3") }
        var results by remember { mutableStateOf<List<Pair<Int, Float>>>(emptyList()) }
        var isProcessing by remember { mutableStateOf(false) }
        val scope = rememberCoroutineScope()
        
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background
        ) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(16.dp)
                    .verticalScroll(rememberScrollState()),
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                // Title
                Text(
                    text = "🚀 Tiny Transformer",
                    style = MaterialTheme.typography.headlineMedium,
                    fontWeight = FontWeight.Bold
                )
                
                // Status card
                Card(
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        Text(
                            text = "Status",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.Bold
                        )
                        Text(
                            text = status,
                            style = MaterialTheme.typography.bodyMedium
                        )
                        if (backend.isNotEmpty()) {
                            Text(
                                text = "Backend: $backend",
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.primary
                            )
                        }
                    }
                }
                
                // Input field
                OutlinedTextField(
                    value = inputText,
                    onValueChange = { inputText = it },
                    label = { Text("Input Token IDs") },
                    placeholder = { Text("e.g., 1, 2, 3") },
                    modifier = Modifier.fillMaxWidth()
                )
                
                // Buttons
                Button(
                    onClick = {
                        scope.launch {
                            initializeEngine { s, b ->
                                status = s
                                backend = b
                            }
                        }
                    },
                    modifier = Modifier.fillMaxWidth(),
                    enabled = transformer == null
                ) {
                    Text("Initialize Engine")
                }
                
                Button(
                    onClick = {
                        scope.launch {
                            createModel { s -> status = s }
                        }
                    },
                    modifier = Modifier.fillMaxWidth(),
                    enabled = transformer != null
                ) {
                    Text("Create Model")
                }
                
                Button(
                    onClick = {
                        scope.launch {
                            runInference(inputText) { s, r ->
                                status = s
                                results = r
                            }
                        }
                    },
                    modifier = Modifier.fillMaxWidth(),
                    enabled = transformer != null && !isProcessing
                ) {
                    if (isProcessing) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(24.dp),
                            color = MaterialTheme.colorScheme.onPrimary
                        )
                    } else {
                        Text("Run Inference")
                    }
                }
                
                // Results
                if (results.isNotEmpty()) {
                    Card(
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Column(
                            modifier = Modifier.padding(16.dp),
                            verticalArrangement = Arrangement.spacedBy(8.dp)
                        ) {
                            Text(
                                text = "Top 5 Predictions",
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.Bold
                            )
                            results.forEachIndexed { index, (token, prob) ->
                                Row(
                                    modifier = Modifier.fillMaxWidth(),
                                    horizontalArrangement = Arrangement.SpaceBetween
                                ) {
                                    Text(
                                        text = "${index + 1}. Token $token",
                                        style = MaterialTheme.typography.bodyMedium
                                    )
                                    Text(
                                        text = String.format("%.2f%%", prob * 100),
                                        style = MaterialTheme.typography.bodyMedium,
                                        color = MaterialTheme.colorScheme.primary
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    private suspend fun initializeEngine(callback: (String, String) -> Unit) {
        withContext(Dispatchers.IO) {
            val bridge = TransformerBridge()
            if (bridge.init()) {
                val backendName = bridge.getBackend()
                transformer = bridge
                withContext(Dispatchers.Main) {
                    callback("Engine initialized", backendName)
                }
            } else {
                withContext(Dispatchers.Main) {
                    callback("Failed to initialize", "")
                }
            }
        }
    }
    
    private suspend fun createModel(callback: (String) -> Unit) {
        withContext(Dispatchers.IO) {
            val success = transformer?.createDefaultModel() ?: false
            val vocabSize = transformer?.getVocabSize() ?: 0
            withContext(Dispatchers.Main) {
                if (success) {
                    callback("Model created (vocab: $vocabSize)")
                } else {
                    callback("Failed to create model")
                }
            }
        }
    }
    
    private suspend fun runInference(
        input: String,
        callback: (String, List<Pair<Int, Float>>) -> Unit
    ) {
        withContext(Dispatchers.IO) {
            val tokens = input.split(",")
                .mapNotNull { it.trim().toFloatOrNull() }
                .toFloatArray()
            
            if (tokens.isEmpty()) {
                withContext(Dispatchers.Main) {
                    callback("Invalid input", emptyList())
                }
                return@withContext
            }
            
            val output = transformer?.forward(tokens)
            
            withContext(Dispatchers.Main) {
                if (output != null) {
                    val indexed = output.mapIndexed { index, value -> Pair(index, value) }
                    val sorted = indexed.sortedByDescending { it.second }
                    val top5 = sorted.take(5)
                    callback("Inference complete", top5)
                } else {
                    callback("Inference failed", emptyList())
                }
            }
        }
    }
}
