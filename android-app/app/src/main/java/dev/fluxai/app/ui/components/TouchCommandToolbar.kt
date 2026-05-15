package dev.fluxai.app.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import dev.fluxai.app.ui.theme.SelectionBg
import dev.fluxai.app.ui.theme.TextPrimary

@Composable
fun TouchCommandToolbar(
    onCommandClick: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    val commands = listOf("ESC", "CTRL", "ALT", "TAB", "↑", "↓", "←", "→", "-", "/", "|", "grep")
    
    LazyRow(
        modifier = modifier
            .fillMaxWidth()
            .background(Color(0xFF121A25)) // slightly lighter than background
            .padding(vertical = 4.dp),
        horizontalArrangement = Arrangement.spacedBy(4.dp),
        contentPadding = PaddingValues(horizontal = 8.dp)
    ) {
        items(commands) { cmd ->
            Box(
                modifier = Modifier
                    .background(SelectionBg, shape = androidx.compose.foundation.shape.RoundedCornerShape(4.dp))
                    .clickable { onCommandClick(cmd) }
                    .padding(horizontal = 12.dp, vertical = 8.dp),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = cmd,
                    color = TextPrimary,
                    fontSize = 12.sp,
                    fontFamily = FontFamily.Monospace
                )
            }
        }
    }
}
