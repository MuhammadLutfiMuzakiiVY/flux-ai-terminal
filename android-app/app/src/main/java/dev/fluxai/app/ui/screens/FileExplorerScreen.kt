package dev.fluxai.app.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Code
import androidx.compose.material.icons.filled.Description
import androidx.compose.material.icons.filled.Folder
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import dev.fluxai.app.ui.theme.*

data class FileItem(val name: String, val isDirectory: Boolean, val size: String, val date: String)

@Composable
fun FileExplorerScreen() {
    // Mock Data based on our Rust virtual filesystem
    val currentPath by remember { mutableStateOf("/home/flux/projects") }
    val files = remember {
        listOf(
            FileItem("android-app", true, "--", "Oct 24"),
            FileItem("core", true, "--", "Oct 24"),
            FileItem("main.rs", false, "12 KB", "Today"),
            FileItem("config.json", false, "2 KB", "Yesterday"),
            FileItem("README.md", false, "5 KB", "Oct 20")
        )
    }

    Column(modifier = Modifier.fillMaxSize().background(Background)) {
        // Path breadcrumbs
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(SelectionBg)
                .padding(horizontal = 16.dp, vertical = 12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text("root", color = CyanBlue, fontSize = 14.sp)
            Text(" / ", color = TextSecondary)
            Text("home", color = CyanBlue, fontSize = 14.sp)
            Text(" / ", color = TextSecondary)
            Text("flux", color = CyanBlue, fontSize = 14.sp)
            Text(" / ", color = TextSecondary)
            Text("projects", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 14.sp)
        }
        
        // File List
        LazyColumn(modifier = Modifier.weight(1f)) {
            items(files) { file ->
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { /* Handle open */ }
                        .padding(horizontal = 16.dp, vertical = 12.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Icon(
                        imageVector = if (file.isDirectory) Icons.Default.Folder else if(file.name.endsWith(".rs")) Icons.Default.Code else Icons.Default.Description,
                        contentDescription = null,
                        tint = if (file.isDirectory) NeonBlue else TextSecondary,
                        modifier = Modifier.size(24.dp)
                    )
                    Spacer(modifier = Modifier.width(16.dp))
                    Column(modifier = Modifier.weight(1f)) {
                        Text(file.name, color = TextPrimary, fontSize = 16.sp)
                        Text("${file.size} • ${file.date}", color = TextSecondary, fontSize = 12.sp)
                    }
                }
                Divider(color = Color(0xFF151D26), thickness = 1.dp)
            }
        }
        
        // Action Bar
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(Surface)
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceEvenly
        ) {
            Button(
                onClick = { /* New File */ },
                colors = ButtonDefaults.buttonColors(containerColor = SelectionBg, contentColor = NeonGreen)
            ) {
                Text("New File")
            }
            Button(
                onClick = { /* Open Editor */ },
                colors = ButtonDefaults.buttonColors(containerColor = Green, contentColor = Color.Black)
            ) {
                Text("Open in Editor")
            }
        }
    }
}
