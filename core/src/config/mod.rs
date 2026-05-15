//! Application configuration and settings
use crate::{FluxResult, FluxError, ai::AiProviderConfig, shell::ShellType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxConfig {
    pub general: GeneralConfig,
    pub terminal: TerminalConfig,
    pub ai: AiProviderConfig,
    pub security: SecurityConfig,
    pub sync: SyncConfig,
    pub emulator: EmulatorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub app_name: String,
    pub author: String,
    pub version: String,
    pub data_dir: String,
    pub first_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub default_shell: ShellType,
    pub font_size: u32,
    pub font_family: String,
    pub theme: String,
    pub cursor_style: String,
    pub cursor_blink: bool,
    pub scrollback_lines: u32,
    pub prompt: String,
    pub bell_enabled: bool,
    pub copy_on_select: bool,
    pub paste_on_middle_click: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub biometric_enabled: bool,
    pub encrypted_storage: bool,
    pub command_safety: bool,
    pub sandbox_plugins: bool,
    pub auto_lock_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub enabled: bool,
    pub provider: String,
    pub auto_sync: bool,
    pub sync_interval_seconds: u32,
    pub git_enabled: bool,
    pub github_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorConfig {
    pub detect_emulator: bool,
    pub keyboard_passthrough: bool,
    pub mouse_support: bool,
    pub clipboard_sync: bool,
    pub resizable_window: bool,
}

impl Default for FluxConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                app_name: "Flux AI Terminal".into(),
                author: "Muhammad Lutfi Muzaki Dev".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                data_dir: String::new(),
                first_run: true,
            },
            terminal: TerminalConfig {
                default_shell: ShellType::Bash,
                font_size: 14,
                font_family: "JetBrains Mono".into(),
                theme: "Flux Dark".into(),
                cursor_style: "block".into(),
                cursor_blink: true,
                scrollback_lines: 10000,
                prompt: "\\u@\\h:\\w\\$ ".into(),
                bell_enabled: false,
                copy_on_select: true,
                paste_on_middle_click: true,
            },
            ai: AiProviderConfig::default(),
            security: SecurityConfig {
                biometric_enabled: true,
                encrypted_storage: true,
                command_safety: true,
                sandbox_plugins: true,
                auto_lock_seconds: 300,
            },
            sync: SyncConfig {
                enabled: false,
                provider: "none".into(),
                auto_sync: false,
                sync_interval_seconds: 300,
                git_enabled: true,
                github_token: String::new(),
            },
            emulator: EmulatorConfig {
                detect_emulator: true,
                keyboard_passthrough: true,
                mouse_support: true,
                clipboard_sync: true,
                resizable_window: true,
            },
        }
    }
}

impl FluxConfig {
    pub fn load_or_default(data_dir: &str) -> FluxResult<Self> {
        let config_path = format!("{}/config.toml", data_dir);
        match std::fs::read_to_string(&config_path) {
            Ok(content) => {
                let mut config: FluxConfig = toml::from_str(&content)
                    .map_err(|e| FluxError::Config(e.to_string()))?;
                config.general.data_dir = data_dir.into();
                Ok(config)
            }
            Err(_) => {
                let mut config = FluxConfig::default();
                config.general.data_dir = data_dir.into();
                let _ = std::fs::create_dir_all(data_dir);
                let toml_str = toml::to_string_pretty(&config)
                    .map_err(|e| FluxError::Config(e.to_string()))?;
                let _ = std::fs::write(&config_path, toml_str);
                Ok(config)
            }
        }
    }

    pub fn save(&self) -> FluxResult<()> {
        let config_path = format!("{}/config.toml", self.general.data_dir);
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| FluxError::Config(e.to_string()))?;
        std::fs::write(&config_path, toml_str)?;
        Ok(())
    }
}
