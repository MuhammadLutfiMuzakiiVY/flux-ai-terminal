package dev.fluxai.app.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import dev.fluxai.app.ui.theme.*
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen() {
    val drawerState = rememberDrawerState(initialValue = DrawerValue.Closed)
    val scope = rememberCoroutineScope()
    var currentRoute by remember { mutableStateOf("Terminal") }
    var isLocked by remember { mutableStateOf(true) }

    if (isLocked) {
        LockScreen(onUnlock = { token ->
            // Try to unlock the Rust core hardware layer
            try {
                val response = dev.fluxai.app.bridge.FluxBridge.sendMessage(
                    org.json.JSONObject().apply {
                        put("type", "Unlock")
                        put("biometric_token", token)
                    }.toString()
                )
                if (response.contains("unlocked")) {
                    isLocked = false
                }
            } catch (e: Exception) {
                // Keep locked on error
            }
        })
        return
    }

    ModalNavigationDrawer(
        drawerState = drawerState,
        drawerContent = {
            ModalDrawerSheet(
                drawerContainerColor = SidebarBackground,
                drawerContentColor = TextPrimary
            ) {
                Spacer(Modifier.height(24.dp))
                // App Logo & Branding
                Column(modifier = Modifier.padding(horizontal = 24.dp, vertical = 16.dp)) {
                    Text(
                        text = "FLUX",
                        color = NeonGreen,
                        fontSize = 28.sp,
                        fontWeight = FontWeight.Bold,
                        letterSpacing = 2.sp
                    )
                    Text(
                        text = "DEVELOPER WORKSTATION",
                        color = CyanBlue,
                        fontSize = 10.sp,
                        letterSpacing = 1.sp
                    )
                }
                
                Divider(color = SelectionBg, modifier = Modifier.padding(horizontal = 16.dp))
                Spacer(Modifier.height(16.dp))
                
                // Navigation Items
                val navItems = listOf(
                    "Terminal" to Icons.Default.Terminal,
                    "File Explorer" to Icons.Default.Folder,
                    "AI Assistant" to Icons.Default.AutoAwesome,
                    "SSH Manager" to Icons.Default.Computer,
                    "Packages" to Icons.Default.Inventory,
                    "Settings" to Icons.Default.Settings
                )
                
                navItems.forEach { (title, icon) ->
                    NavigationDrawerItem(
                        icon = { Icon(icon, contentDescription = null, tint = if(currentRoute == title) NeonGreen else TextSecondary) },
                        label = { Text(title, color = if(currentRoute == title) TextPrimary else TextSecondary) },
                        selected = currentRoute == title,
                        onClick = {
                            currentRoute = title
                            scope.launch { drawerState.close() }
                        },
                        colors = NavigationDrawerItemDefaults.colors(
                            selectedContainerColor = SelectionBg,
                            unselectedContainerColor = Color.Transparent
                        ),
                        modifier = Modifier.padding(horizontal = 12.dp, vertical = 4.dp)
                    )
                }
            }
        }
    ) {
        Scaffold(
            topBar = {
                TopAppBar(
                    title = { Text(currentRoute, fontSize = 16.sp, color = TextPrimary) },
                    navigationIcon = {
                        IconButton(onClick = { scope.launch { drawerState.open() } }) {
                            Icon(Icons.Default.Menu, contentDescription = "Menu", tint = TextPrimary)
                        }
                    },
                    actions = {
                        if (currentRoute == "Terminal") {
                            IconButton(onClick = { /* Handle Add Tab */ }) {
                                Icon(Icons.Default.Add, contentDescription = "New Tab", tint = NeonGreen)
                            }
                        }
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = Background
                    )
                )
            },
            containerColor = Background
        ) { padding ->
            Box(modifier = Modifier.padding(padding).fillMaxSize()) {
                when (currentRoute) {
                    "Terminal" -> TerminalScreen()
                    "File Explorer" -> FileExplorerScreen()
                    "AI Assistant" -> AiAssistantScreen()
                    else -> PlaceholderScreen(currentRoute)
                }
            }
        }
    }
}

@Composable
fun LockScreen(onUnlock: (String) -> Unit) {
    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Background),
        contentAlignment = Alignment.Center
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Icon(
                Icons.Default.Fingerprint,
                contentDescription = "Lock",
                modifier = Modifier
                    .size(80.dp)
                    .clickable { onUnlock("biometric_hardware_token_v2_secure") },
                tint = NeonGreen
            )
            Spacer(Modifier.height(16.dp))
            Text(
                "FLUX SECURE VAULT",
                color = TextPrimary,
                fontWeight = FontWeight.Bold,
                letterSpacing = 2.sp
            )
            Text(
                "Touch fingerprint sensor to unlock system",
                color = TextSecondary,
                fontSize = 12.sp
            )
        }
    }
}

@Composable
fun PlaceholderScreen(title: String) {
    Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
        Text("$title is under construction. Powered by Rust.", color = TextSecondary)
    }
}
