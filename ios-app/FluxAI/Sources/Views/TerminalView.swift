import SwiftUI

struct TerminalView: View {
    @State private var outputLines: [String] = ["Flux AI Terminal v1.0.0", "Type 'help' for commands."]
    @State private var inputCommand: String = ""
    
    var body: some View {
        VStack(spacing: 0) {
            ScrollViewReader { proxy in
                ScrollView {
                    VStack(alignment: .leading, spacing: 4) {
                        ForEach(outputLines.indices, id: \.self) { index in
                            Text(outputLines[index])
                                .font(.system(.body, design: .monospaced))
                                .foregroundColor(Color(hex: "B3B1AD"))
                                .frame(maxWidth: .infinity, alignment: .leading)
                        }
                    }
                    .padding()
                }
                .background(Color(hex: "01060E"))
            }
            
            HStack {
                Text("flux@flux:~$")
                    .font(.system(.body, design: .monospaced))
                    .foregroundColor(Color(hex: "91B362"))
                
                TextField("", text: $inputCommand)
                    .font(.system(.body, design: .monospaced))
                    .foregroundColor(Color(hex: "B3B1AD"))
                    .disableAutocorrection(true)
                    .autocapitalization(.none)
                    .onSubmit {
                        executeCommand()
                    }
            }
            .padding()
            .background(Color(hex: "01060E"))
        }
    }
    
    private func executeCommand() {
        let cmd = inputCommand
        guard !cmd.isEmpty else { return }
        
        inputCommand = ""
        outputLines.append("flux@flux:~$ \(cmd)")
        
        let jsonCmd = """
        {"ExecuteCommand": {"session_id": "default", "command": "\(cmd)"}}
        """
        
        let response = send_message(jsonCmd)
        // In a real app we'd parse the JSON properly using Codable
        // Here we just append a mock response if parsing fails
        if response.contains("CommandOutput") {
            // Mock output display
            outputLines.append("flux: executed \(cmd) (Rust core mock)")
        } else {
            outputLines.append("flux: executed \(cmd)")
        }
    }
}

extension Color {
    init(hex: String) {
        let hex = hex.trimmingCharacters(in: CharacterSet.alphanumerics.inverted)
        var int: UInt64 = 0
        Scanner(string: hex).scanHexInt64(&int)
        let a, r, g, b: UInt64
        switch hex.count {
        case 3: // RGB (12-bit)
            (a, r, g, b) = (255, (int >> 8) * 17, (int >> 4 & 0xF) * 17, (int & 0xF) * 17)
        case 6: // RGB (24-bit)
            (a, r, g, b) = (255, int >> 16, int >> 8 & 0xFF, int & 0xFF)
        case 8: // ARGB (32-bit)
            (a, r, g, b) = (int >> 24, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
        default:
            (a, r, g, b) = (1, 1, 1, 0)
        }
        self.init(
            .sRGB,
            red: Double(r) / 255,
            green: Double(g) / 255,
            blue:  Double(b) / 255,
            opacity: Double(a) / 255
        )
    }
}
