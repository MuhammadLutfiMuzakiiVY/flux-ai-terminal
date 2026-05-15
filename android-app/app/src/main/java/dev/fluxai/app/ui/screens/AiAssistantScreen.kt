package dev.fluxai.app.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import dev.fluxai.app.bridge.FluxBridge
import org.json.JSONObject

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AiAssistantScreen() {
    var messages by remember { mutableStateOf(listOf(
        "assistant" to "Hello! I am Flux AI. How can I help you with your code or terminal tasks today?"
    )) }
    var inputMessage by remember { mutableStateOf("") }
    var isLoading by remember { mutableStateOf(false) }

    Column(
        modifier = Modifier.fillMaxSize().padding(16.dp)
    ) {
        LazyColumn(
            modifier = Modifier.weight(1f),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            items(messages) { msg ->
                val isUser = msg.first == "user"
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = if (isUser) Arrangement.End else Arrangement.Start
                ) {
                    Surface(
                        color = if (isUser) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.surfaceVariant,
                        shape = RoundedCornerShape(12.dp),
                        modifier = Modifier.widthIn(max = 300.dp)
                    ) {
                        Text(
                            text = msg.second,
                            modifier = Modifier.padding(12.dp),
                            color = if (isUser) MaterialTheme.colorScheme.onPrimary else MaterialTheme.colorScheme.onSurface
                        )
                    }
                }
            }
        }
        
        Spacer(modifier = Modifier.height(8.dp))
        
        if (isLoading) {
            CircularProgressIndicator(modifier = Modifier.size(24.dp))
        }

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            OutlinedTextField(
                value = inputMessage,
                onValueChange = { inputMessage = it },
                modifier = Modifier.weight(1f),
                placeholder = { Text("Ask Flux AI...") },
                singleLine = true
            )
            Button(
                onClick = {
                    if (inputMessage.isNotBlank()) {
                        val msg = inputMessage
                        messages = messages + ("user" to msg)
                        inputMessage = ""
                        isLoading = true
                        
                        // In real app, call FluxBridge asynchronously. Here we simulate it.
                        try {
                            val jsonCmd = JSONObject().apply {
                                put("AiChat", JSONObject().apply {
                                    put("message", msg)
                                    put("context", JSONObject.NULL)
                                })
                            }.toString()
                            
                            val responseStr = FluxBridge.sendMessage(jsonCmd)
                            val resJson = JSONObject(responseStr)
                            
                            val reply = if (resJson.has("AiResponse")) {
                                resJson.getJSONObject("AiResponse").getString("content")
                            } else {
                                "I'm a placeholder response. Rust core AI is not fully connected in this mock."
                            }
                            
                            messages = messages + ("assistant" to reply)
                        } catch (e: Exception) {
                            messages = messages + ("assistant" to "Error connecting to AI engine.")
                        } finally {
                            isLoading = false
                        }
                    }
                }
            ) {
                Text("Send")
            }
        }
    }
}
