//! # Flux AI Core
//!
//! The core engine powering Flux AI Terminal - a portable Linux AI terminal
//! for modern developers.
//!
//! ## Architecture
//!
//! Flux Core is organized into the following modules:
//!
//! - **shell**: Shell interpreter (bash/zsh/sh), command parsing, aliases, history
//! - **terminal**: Terminal emulation, PTY management, multi-tab, split views
//! - **ai**: AI assistant engine with multi-provider support
//! - **package**: Debian/Ubuntu-style apt package manager
//! - **filesystem**: Virtual Linux filesystem abstraction
//! - **process**: Process manager with virtual PID tracking
//! - **sync**: Cloud sync, Git integration, device sync
//! - **plugins**: Plugin SDK and marketplace integration
//! - **emulator**: Android emulator compatibility (BlueStacks, LDPlayer, NoxPlayer)
//! - **security**: Encryption, biometric auth, sandboxing
//! - **bridge**: JNI/FFI bridge for Android/iOS native UIs
//! - **config**: Application configuration and settings
//!
//! ## Created by
//!
//! Muhammad Lutfi Muzaki Dev
//!
//! ## License
//!
//! MIT

pub mod ai;
pub mod bridge;
pub mod config;
pub mod device;
pub mod emulator;
pub mod filesystem;
pub mod package;
pub mod plugins;
pub mod process;
pub mod security;
pub mod shell;
pub mod sync;
pub mod terminal;
pub mod tools;
pub mod gui;

use thiserror::Error;

/// Application-wide version constant
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = "Flux AI Terminal";

/// Application author
pub const AUTHOR: &str = "Muhammad Lutfi Muzaki Dev";

/// Application tagline
pub const TAGLINE: &str = "Portable Linux AI terminal for modern developers";

/// Core error type for Flux
#[derive(Error, Debug)]
pub enum FluxError {
    #[error("Shell error: {0}")]
    Shell(String),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("AI error: {0}")]
    Ai(String),

    #[error("Package error: {0}")]
    Package(String),

    #[error("Filesystem error: {0}")]
    Filesystem(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("Sync error: {0}")]
    Sync(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Theme error: {0}")]
    Theme(String),

    #[error("Tools error: {0}")]
    Tools(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Bridge error: {0}")]
    Bridge(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Core result type
pub type FluxResult<T> = Result<T, FluxError>;

/// Initialize the Flux core engine
///
/// Sets up logging, filesystem, configuration, and all subsystems.
pub async fn initialize(data_dir: &str) -> FluxResult<FluxEngine> {
    tracing::info!("Initializing {} v{}", APP_NAME, VERSION);
    tracing::info!("Created by {}", AUTHOR);

    // Initialize configuration
    let config = config::FluxConfig::load_or_default(data_dir)?;

    // Initialize virtual filesystem
    let fs = filesystem::VirtualFilesystem::new(data_dir)?;

    // Initialize package manager
    let package_manager = package::AptPackageManager::new(&fs)?;

    // Initialize security manager
    let security = security::SecurityManager::new(data_dir)?;

    // Initialize AI engine
    let ai_engine = ai::AiEngine::new(&config)?;

    // Initialize process manager
    let process_manager = process::ProcessManager::new();

    // Initialize plugin manager
    let plugin_manager = plugins::PluginManager::new(data_dir)?;

    // Initialize sync manager
    let sync_manager = sync::SyncManager::new(&config)?;

    // Initialize device manager
    let device_manager = device::DeviceManager::new();

    // Initialize tools manager (SSH, Workspace, Editor)
    let tools_manager = tools::ToolsManager::new();

    // Initialize GUI Display Server
    let gui_config = gui::VirtualDisplayConfig {
        width: 1920,
        height: 1080,
        dpi: 320,
        color_depth: 32,
        hardware_acceleration: true,
    };
    let gui_server = gui::DisplayServer::new(gui_config)?;

    tracing::info!("{} initialized successfully", APP_NAME);

    Ok(FluxEngine {
        config,
        filesystem: fs,
        package_manager,
        security,
        ai_engine,
        process_manager,
        plugin_manager,
        sync_manager,
        device_manager,
        tools_manager,
        gui_server,
    })
}

/// The main Flux engine that coordinates all subsystems
pub struct FluxEngine {
    pub config: config::FluxConfig,
    pub filesystem: filesystem::VirtualFilesystem,
    pub package_manager: package::AptPackageManager,
    pub security: security::SecurityManager,
    pub ai_engine: ai::AiEngine,
    pub process_manager: process::ProcessManager,
    pub plugin_manager: plugins::PluginManager,
    pub sync_manager: sync::SyncManager,
    pub device_manager: device::DeviceManager,
    pub tools_manager: tools::ToolsManager,
    pub gui_server: gui::DisplayServer,
}

impl FluxEngine {
    /// Execute a command string with firewall protection
    pub async fn execute_command(&mut self, session_id: &str, command: &str) -> FluxResult<shell::CommandOutput> {
        tracing::debug!("Executing command in session {}: {}", session_id, command);

        // Layer 1: Firewall Check
        let safety = self.security.check_command_safety(command);
        if safety == security::SafetyAction::Block {
            return Err(crate::FluxError::Security(format!("Command blocked by Flux Firewall: Prohibited operation detected.")));
        }
        
        if safety == security::SafetyAction::Warn {
            tracing::warn!("Dangerous command detected: {}", command);
        }

        let output = shell::execute_command(
            command,
            &mut self.filesystem,
            &mut self.package_manager,
            &self.config,
        ).await?;
        Ok(output)
    }

    pub fn unlock_security(&mut self, biometric_token: &str) -> FluxResult<()> {
        self.security.unlock_hardware_layer(biometric_token)
    }

    /// Send a message to the AI assistant
    pub async fn ai_chat(&mut self, message: &str, context: Option<ai::AiContext>) -> FluxResult<String> {
        self.ai_engine.chat(message, context).await
    }

    /// Get system information
    pub fn system_info(&self) -> SystemInfo {
        SystemInfo {
            app_name: APP_NAME.to_string(),
            version: VERSION.to_string(),
            author: AUTHOR.to_string(),
            tagline: TAGLINE.to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        }
    }
}

/// System information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemInfo {
    pub app_name: String,
    pub version: String,
    pub author: String,
    pub tagline: String,
    pub os: String,
    pub arch: String,
}
