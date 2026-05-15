package dev.fluxai.app.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import dev.fluxai.app.bridge.FluxBridge
import dev.fluxai.app.ui.theme.CursorColor
import dev.fluxai.app.ui.theme.TerminalBackground
import dev.fluxai.app.ui.theme.TextPrimary
import org.json.JSONObject

@Composable
fun TerminalScreen() {
    var outputLines by remember { mutableStateOf(listOf("Flux AI Terminal v1.0.0", "Type 'help' for commands.")) }
    var inputCommand by remember { mutableStateOf("") }
    
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(TerminalBackground)
            .padding(8.dp)
    ) {
        LazyColumn(
            modifier = Modifier.weight(1f),
            verticalArrangement = Arrangement.Bottom
        ) {
            items(outputLines) { line ->
                Text(
                    text = line,
                    color = TextPrimary,
                    fontFamily = FontFamily.Monospace,
                    fontSize = 14.sp
                )
            }
        }
        
        Row(modifier = Modifier.fillMaxWidth().padding(top = 8.dp)) {
            Text(
                text = "flux@flux:~$ ",
                color = dev.fluxai.app.ui.theme.Green,
                fontFamily = FontFamily.Monospace,
                fontSize = 14.sp
            )
            BasicTextField(
                value = inputCommand,
                onValueChange = { inputCommand = it },
                textStyle = TextStyle(
                    color = TextPrimary,
                    fontFamily = FontFamily.Monospace,
                    fontSize = 14.sp
                ),
                cursorBrush = SolidColor(CursorColor),
                keyboardOptions = KeyboardOptions(imeAction = ImeAction.Send),
                keyboardActions = KeyboardActions(
                    onSend = {
                        val cmd = inputCommand
                        inputCommand = ""
                        // Professional Bridge Execution with Force-Close Protection
                        try {
                            val requestJson = JSONObject().apply {
                                put("type", "ExecuteCommand")
                                put("command", cmd)
                                put("sessionId", "main")
                            }.toString()
                            
                            val responseJson = FluxBridge.sendMessage(requestJson)
                            val responseObj = JSONObject(responseJson)
                            
                            if (responseObj.has("error")) {
                                outputLines = outputLines + "E: ${responseObj.getString("error")}"
                            } else {
                                val out = responseObj.optString("stdout", "")
                                val err = responseObj.optString("stderr", "")
                                if (out.isNotEmpty()) outputLines = outputLines + out
                                if (err.isNotEmpty()) outputLines = outputLines + "E: $err"
                            }
                        } catch (e: Exception) {
                            outputLines = outputLines + "Flux Error: ${e.message}"
                            android.util.Log.e("TerminalScreen", "Bridge call failed", e)
                        }
                    }
                ),
                modifier = Modifier.fillMaxWidth().weight(1f)
            )
        }
        
        Spacer(modifier = Modifier.height(4.dp))
        
        // The newly requested Touch Command Toolbar
        dev.fluxai.app.ui.components.TouchCommandToolbar(
            onCommandClick = { cmd -> 
                // Simple mock behavior: append to input
                inputCommand += cmd
            }
        )
    }
}
