# Flux AI Terminal - Roadmap & Vision 🗺️

**Positioning**: Not just a terminal clone, but a "Mobile Developer Workstation".
**Formula**: Terminal + File Explorer + Code Editor + Git + SSH + Workspace + Sync + AI.

---

## 🚀 Core v1 (Foundation & Workstation Essentials)
*Focus: Build a robust, standalone mobile developer environment.*

- [x] **Terminal Emulator & Shell**: Multiple sessions, split panes, ANSI themes, and a Touch Command Toolbar (Ctrl, Esc, Tab, Arrows).
- [x] **Debian-like Environment**: Authentic `/bin`, `/etc`, `/usr` filesystem abstraction.
- [x] **Package Manager (CLI & GUI)**: `apt` support without Termux-style `pkg`. UI wrapper for searching, installing, and updating packages.
- [x] **File Explorer**: Built-in graphical sidebar for storage browsing, quick previews, and renaming without relying purely on CLI.
- [x] **Native Code Editor**: Lightweight editor with syntax highlighting, tabs, and line numbers to complement vim/nano.
- [x] **SSH Manager**: GUI for saving server lists, key imports, and SFTP browsing.
- [x] **Git Integration**: Clone, commit history, branch switching, and pull/push buttons directly in the UI.
- [x] **Session Restore**: Restore last session, save profiles, and named tabs.
- [x] **Android Support**: Native Kotlin/Jetpack Compose app.

## 🌟 v2 (Ecosystem & Extension)
*Focus: Expanding platforms, sync, and customization.*

- [x] **iOS Support**: Same Rust engine, package system, and shell running on Swift/SwiftUI. (Engine ready, UI pending).
- [x] **Cross-device Sync**: Sync config, projects, shell history, and installed packages via GitHub Gist, Google Drive, or iCloud.
- [x] **Plugin System**: API for Docker remote, database clients, and markdown viewers.
- [x] **Workspace Manager**: Recent projects, pin projects, per-project environments, and templates.
- [ ] **Package Profiles**: Install developer presets (Python dev, Rust dev, Node.js profile).

## 🔮 v3 (Cloud & Advanced Features)
*Focus: Collaboration, security, and advanced extensions.*

- [ ] **Security Enhancements**: App lock, biometric, encrypted SSH keys, and command confirmation mode.
- [ ] **Built-in Server Tools**: Local HTTP/FTP/SFTP servers, port forwarding, network monitor.
- [ ] **Resource Monitor**: GUI for CPU usage, memory, running processes, and disk usage.
- [ ] **Package Snapshot System**: Export and restore entire development environments.
- [ ] **Collaboration & Marketplace**: Share environments and download custom community plugins.
