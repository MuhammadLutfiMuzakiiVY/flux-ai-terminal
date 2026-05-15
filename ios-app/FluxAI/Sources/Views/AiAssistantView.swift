import SwiftUI

struct Message: Identifiable {
    let id = UUID()
    let isUser: Bool
    let text: String
}

struct AiAssistantView: View {
    @State private var messages: [Message] = [
        Message(isUser: false, text: "Hello! I am Flux AI. How can I help you with your code or terminal tasks today?")
    ]
    @State private var inputText: String = ""
    @State private var isLoading = false
    
    var body: some View {
        VStack {
            ScrollView {
                LazyVStack(spacing: 12) {
                    ForEach(messages) { message in
                        MessageBubble(message: message)
                    }
                }
                .padding()
            }
            
            if isLoading {
                ProgressView()
                    .padding()
            }
            
            HStack {
                TextField("Ask Flux AI...", text: $inputText)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .onSubmit { sendMessage() }
                
                Button(action: sendMessage) {
                    Image(systemName: "paperplane.fill")
                        .foregroundColor(.blue)
                }
            }
            .padding()
            .background(Color(UIColor.systemBackground))
        }
        .navigationTitle("Flux AI")
    }
    
    private func sendMessage() {
        guard !inputText.isEmpty else { return }
        
        let userText = inputText
        inputText = ""
        messages.append(Message(isUser: true, text: userText))
        isLoading = true
        
        // Mock async request to Rust core
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
            let jsonCmd = """
            {"AiChat": {"message": "\(userText)"}}
            """
            let response = send_message(jsonCmd)
            self.isLoading = false
            self.messages.append(Message(isUser: false, text: "I'm a placeholder response. Rust core AI is not fully connected in this mock."))
        }
    }
}

struct MessageBubble: View {
    let message: Message
    
    var body: some View {
        HStack {
            if message.isUser { Spacer() }
            
            Text(message.text)
                .padding(12)
                .background(message.isUser ? Color.blue : Color(UIColor.secondarySystemBackground))
                .foregroundColor(message.isUser ? .white : .primary)
                .cornerRadius(16)
            
            if !message.isUser { Spacer() }
        }
    }
}
