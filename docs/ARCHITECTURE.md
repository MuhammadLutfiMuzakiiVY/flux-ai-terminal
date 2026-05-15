# Architecture of Flux AI Terminal

## Overview

Flux AI is designed with a strict separation of concerns, utilizing a high-performance Rust backend for all core logic, communicating via Foreign Function Interfaces (FFI) to lightweight, native UI layers.

## Core Rust Engine (`flux-core`)

The Rust engine is the heart of Flux AI. It runs the entire Linux abstraction and AI integration.

### Modules
*   **Shell (`shell/`)**: Parses commands, handles pipes (`|`), redirects (`>`, `>>`), aliases, environment variables, and builtins (`cd`, `ls`, `pwd`, etc.).
*   **Terminal (`terminal/`)**: Manages the PTY buffer, multi-tab state, ANSI color parsing, cursor positioning, and window splitting.
*   **AI Engine (`ai/`)**: Handles communication with LLM providers (OpenAI, Anthropic, Gemini, Ollama). Maintains context awareness (current directory, file contents, recent errors).
*   **Filesystem (`filesystem/`)**: A virtual Linux filesystem (`/bin`, `/etc`, `/home`, etc.) mapped to the app's sandboxed data directory.
*   **Package Manager (`package/`)**: Implements `apt` and `dpkg` command parsing, resolving dependencies, and managing installed binaries.
*   **Process Manager (`process/`)**: Virtual PID tracker that manages background jobs and provides output for commands like `ps` and `kill`.
*   **Security (`security/`)**: Encrypted storage for API keys, biometric auth hooks, and a command safety system (blocking `rm -rf /`).
*   **Sync (`sync/`)**: Manages cloud synchronization of the workspace and settings.
*   **Device Integration (`device/`)**: Manages access to native hardware features like Camera, Microphone, GPS Location, Clipboard, Notifications, Biometrics, File Picker, and Sensors.
*   **Developer Tools (`tools/`)**: Integrated SSH and SFTP manager, Code Editor buffers, Workspace directory registration, and Local Server port exposure.

## Shared Bindings (`flux-shared-bindings`)

This crate exposes the Rust core to external languages.
*   **JNI**: Exported C-compatible functions that the Android JVM can call into.
*   **UniFFI**: Used to generate Swift bindings for the iOS app.

### Communication Protocol
The UI and Core communicate primarily through asynchronous JSON message passing (`BridgeMessage`), allowing complex structured data (like chat history or directory trees) to be easily serialized and deserialized across the FFI boundary.

## Native Frontends

*   **Android (`android-app`)**: Built with Kotlin and Jetpack Compose. Utilizes a `ViewModel` architecture to manage state received from the Rust core.
*   **iOS (`ios-app`)**: Built with Swift and SwiftUI. Follows an MVVM pattern, interfacing with the Rust core via the generated Swift wrapper.

## Emulator Compatibility

The Rust core detects if it is running within known Android emulators (BlueStacks, LDPlayer, NoxPlayer) and automatically adjusts settings such as enabling desktop keyboard passthrough and mapping host clipboard events to the terminal buffer.
