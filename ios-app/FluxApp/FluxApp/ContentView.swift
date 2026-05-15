import SwiftUI

struct ContentView: View {
    @State private var terminalOutput: String = "Flux AI Terminal [Version 1.0.0]\n(c) 2026 Muhammad Lutfi Muzaki Dev\n\nflux@ios:~$ "
    @State private var command: String = ""
    @ObservedObject var bridge = FluxBridge.shared

    var body: some View {
        ZStack {
            Color(red: 0.05, green: 0.05, blue: 0.08).edgesIgnoringSafeArea(.all)
            
            VStack(alignment: .leading, spacing: 0) {
                // Header
                HStack {
                    Image(systemName: "terminal.fill")
                        .foregroundColor(.green)
                    Text("Flux AI Terminal")
                        .font(.headline)
                        .foregroundColor(.white)
                    Spacer()
                    Circle()
                        .fill(Color.green)
                        .frame(width: 10, height: 10)
                    Text("Connected")
                        .font(.caption)
                        .foregroundColor(.gray)
                }
                .padding()
                .background(Color.white.opacity(0.05))

                // Terminal Area
                ScrollView {
                    Text(terminalOutput)
                        .font(.system(.body, design: .monospaced))
                        .foregroundColor(.green)
                        .padding()
                        .frame(maxWidth: .infinity, alignment: .leading)
                }
                
                // Input Area
                HStack {
                    Text("flux@ios:~$")
                        .font(.system(.body, design: .monospaced))
                        .foregroundColor(.blue)
                    TextField("", text: $command, onCommit: executeCommand)
                        .font(.system(.body, design: .monospaced))
                        .foregroundColor(.white)
                        .autocapitalization(.none)
                        .disableAutocorrection(true)
                }
                .padding()
                .background(Color.white.opacity(0.05))
            }
        }
    }

    func executeCommand() {
        let response = bridge.sendMessage(command)
        terminalOutput += "\(command)\n\(response)\nflux@ios:~$ "
        command = ""
    }
}
