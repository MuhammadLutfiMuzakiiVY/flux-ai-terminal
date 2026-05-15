import SwiftUI

struct ContentView: View {
    var body: some View {
        TabView {
            TerminalView()
                .tabItem {
                    Label("Terminal", systemImage: "terminal")
                }
            
            AiAssistantView()
                .tabItem {
                    Label("AI Assistant", systemImage: "sparkles")
                }
                
            Text("File Explorer")
                .tabItem {
                    Label("Files", systemImage: "folder")
                }
        }
        .preferredColorScheme(.dark)
    }
}

#Preview {
    ContentView()
}
