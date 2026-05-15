import SwiftUI

@main
struct FluxAIApp: App {
    init() {
        // Initialize Flux core
        let dataDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!.path
        _ = init_flux(dataDir)
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
