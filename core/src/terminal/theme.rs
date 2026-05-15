//! Advanced Theme Engine for Flux AI Terminal
use crate::{FluxResult, FluxError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main application theme configuration encompassing both Terminal and UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTheme {
    pub id: String,
    pub name: String,
    pub author: String,
    pub is_dark: bool,
    
    // Terminal specific settings
    pub terminal: TerminalThemeConfig,
    
    // Font settings
    pub typography: TypographyConfig,
    
    // Application UI colors
    pub ui: UiThemeConfig,
    
    // Window styling
    pub window: WindowStyleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalThemeConfig {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
    pub prompt: String,
    pub command: String,
    pub error: String,
    pub success: String,
    pub warning: String,
    pub hyperlink: String,
    pub ansi_colors: [String; 16],
    pub opacity: f32,
    pub background_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SupportedFont {
    // Standard Monospace
    JetBrainsMono,
    FiraCode,
    CascadiaCode,
    SourceCodePro,
    UbuntuMono,
    Hack,
    Inconsolata,
    IbmPlexMono,
    AnonymousPro,
    RobotoMono,
    
    // Nerd Fonts (Icon support)
    JetBrainsMonoNerd,
    FiraCodeNerd,
    HackNerd,
    CaskaydiaCoveNerd,
    MesloLgsNerd,
    
    // Custom fallback
    Custom(String),
}

impl Default for SupportedFont {
    fn default() -> Self {
        Self::JetBrainsMono
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SupportedUiFont {
    Inter,
    SfPro,
    Roboto,
    NotoSans,
    Poppins,
    Custom(String),
}

impl Default for SupportedUiFont {
    fn default() -> Self {
        Self::Inter
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SupportedDisplayFont {
    Orbitron,
    Exo2,
    Rajdhani,
    Audiowide,
    SpaceGrotesk,
    Custom(String),
}

impl Default for SupportedDisplayFont {
    fn default() -> Self {
        Self::Orbitron
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    pub font_family: SupportedFont,        // Terminal font
    pub ui_font_family: SupportedUiFont,   // Sidebar, settings, menus
    pub display_font_family: SupportedDisplayFont, // Banners, logos, splash
    pub font_size: f32,
    pub font_weight: u16,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub enable_ligatures: bool,
    pub bold_is_bright: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiThemeConfig {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub surface: String,
    pub sidebar_bg: String,
    pub tab_bg: String,
    pub tab_active: String,
    pub toolbar_bg: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub divider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowStyleConfig {
    pub rounded_corners: bool,
    pub glass_effect: bool,
    pub blur_radius: f32,
    pub fullscreen: bool,
}

/// Pre-defined prompt styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptStyle {
    Ubuntu,     // user@host:~/dir$
    Debian,     // user@host:~/dir$
    Minimal,    // >
    Powerline,  //  user  ~/dir 
    Zsh,        // ➜  dir
    Custom(String),
}

pub struct ThemeManager {
    pub active_theme: AppTheme,
    pub installed_themes: HashMap<String, AppTheme>,
    pub prompt_style: PromptStyle,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            active_theme: Self::flux_dark(),
            installed_themes: HashMap::new(),
            prompt_style: PromptStyle::Ubuntu,
        };
        manager.register_builtin_themes();
        manager
    }

    fn register_builtin_themes(&mut self) {
        let themes = vec![
            Self::flux_dark(),
            Self::flux_light(),
            Self::hacker_green(),
            Self::midnight_blue(),
            Self::ubuntu_style(),
            Self::debian_style(),
            Self::dracula(),
            Self::nord(),
            Self::monokai(),
            Self::gruvbox(),
            Self::matrix(),
            Self::cyberpunk(),
            Self::minimal_black(),
        ];
        for t in themes {
            self.installed_themes.insert(t.id.clone(), t);
        }
    }

    pub fn set_theme(&mut self, id: &str) -> FluxResult<()> {
        if let Some(theme) = self.installed_themes.get(id) {
            self.active_theme = theme.clone();
            Ok(())
        } else {
            Err(FluxError::Theme("Theme not found".into()))
        }
    }

    pub fn import_theme_json(&mut self, json_data: &str) -> FluxResult<()> {
        let theme: AppTheme = serde_json::from_str(json_data)?;
        self.installed_themes.insert(theme.id.clone(), theme);
        Ok(())
    }

    pub fn export_theme_json(&self, id: &str) -> FluxResult<String> {
        let theme = self.installed_themes.get(id)
            .ok_or_else(|| FluxError::Theme("Theme not found".into()))?;
        let json = serde_json::to_string_pretty(theme)?;
        Ok(json)
    }

    pub fn set_prompt_style(&mut self, style: PromptStyle) {
        self.prompt_style = style;
    }

    pub fn apply_emulator_optimizations(&mut self) {
        // Larger scaling and layout for desktop-like emulators
        self.active_theme.typography.font_size *= 1.2;
        self.active_theme.typography.line_height = 1.4;
        self.active_theme.window.fullscreen = true;
    }

    // --- Built-in Themes Definitions ---

    pub fn flux_dark() -> AppTheme {
        AppTheme {
            id: "flux_dark".into(),
            name: "Flux Dark".into(),
            author: "Flux AI".into(),
            is_dark: true,
            terminal: TerminalThemeConfig {
                background: "#01060E".into(),
                foreground: "#C7C7C7".into(),
                cursor: "#EA6C73".into(),
                selection: "#253340".into(),
                prompt: "#91B362".into(),
                command: "#53BDFA".into(),
                error: "#EA6C73".into(),
                success: "#91B362".into(),
                warning: "#F9AF4F".into(),
                hyperlink: "#53BDFA".into(),
                opacity: 0.95,
                background_image: None,
                ansi_colors: [
                    "#01060E".into(), "#EA6C73".into(), "#91B362".into(), "#F9AF4F".into(),
                    "#53BDFA".into(), "#FAE994".into(), "#90E1C6".into(), "#C7C7C7".into(),
                    "#686868".into(), "#F07178".into(), "#C2D94C".into(), "#FFB454".into(),
                    "#59C2FF".into(), "#FFEE99".into(), "#95E6CB".into(), "#FFFFFF".into(),
                ],
            },
            typography: TypographyConfig {
                font_family: SupportedFont::JetBrainsMono, 
                ui_font_family: SupportedUiFont::Inter,
                display_font_family: SupportedDisplayFont::Orbitron,
                font_size: 14.0, font_weight: 400,
                line_height: 1.2, letter_spacing: 0.0, enable_ligatures: true, bold_is_bright: true,
            },
            ui: UiThemeConfig {
                primary: "#91B362".into(), secondary: "#53BDFA".into(), background: "#0A0E14".into(),
                surface: "#01060E".into(), sidebar_bg: "#0A0E14".into(), tab_bg: "#121A25".into(),
                tab_active: "#01060E".into(), toolbar_bg: "#0A0E14".into(), text_primary: "#E0E0E0".into(),
                text_secondary: "#888888".into(), divider: "#253340".into(),
            },
            window: WindowStyleConfig {
                rounded_corners: true, glass_effect: true, blur_radius: 10.0, fullscreen: false,
            },
        }
    }

    pub fn flux_light() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "flux_light".into(); t.name = "Flux Light".into(); t.is_dark = false;
        t.terminal.background = "#FAFAFA".into(); t.terminal.foreground = "#333333".into();
        t.ui.background = "#EEEEEE".into(); t.ui.surface = "#FFFFFF".into();
        t.ui.text_primary = "#111111".into(); t.ui.tab_bg = "#E0E0E0".into();
        t
    }

    pub fn hacker_green() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "hacker_green".into(); t.name = "Hacker Green".into();
        t.terminal.foreground = "#00FF00".into(); t.terminal.cursor = "#00FF00".into();
        t.typography.font_family = SupportedFont::SourceCodePro;
        t
    }

    pub fn midnight_blue() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "midnight_blue".into(); t.name = "Midnight Blue".into();
        t.terminal.background = "#0A192F".into(); t.ui.background = "#020C1B".into();
        t.ui.surface = "#112240".into();
        t
    }

    pub fn ubuntu_style() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "ubuntu".into(); t.name = "Ubuntu Style".into();
        t.terminal.background = "#300A24".into(); t.terminal.foreground = "#FFFFFF".into();
        t.typography.font_family = SupportedFont::UbuntuMono;
        t.ui.primary = "#E95420".into();
        t
    }

    pub fn debian_style() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "debian".into(); t.name = "Debian Style".into();
        t.terminal.background = "#1D2B36".into(); t.ui.primary = "#D70A53".into();
        t
    }

    pub fn dracula() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "dracula".into(); t.name = "Dracula".into();
        t.terminal.background = "#282A36".into(); t.terminal.foreground = "#F8F8F2".into();
        t.ui.primary = "#FF79C6".into(); t.ui.secondary = "#BD93F9".into();
        t
    }

    pub fn nord() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "nord".into(); t.name = "Nord".into();
        t.terminal.background = "#2E3440".into(); t.terminal.foreground = "#D8DEE9".into();
        t.ui.primary = "#88C0D0".into();
        t
    }

    pub fn monokai() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "monokai".into(); t.name = "Monokai".into();
        t.terminal.background = "#272822".into(); t.terminal.foreground = "#F8F8F2".into();
        t.ui.primary = "#A6E22E".into(); t.ui.secondary = "#F92672".into();
        t
    }

    pub fn gruvbox() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "gruvbox".into(); t.name = "Gruvbox".into();
        t.terminal.background = "#282828".into(); t.terminal.foreground = "#EBDBB2".into();
        t.ui.primary = "#B8BB26".into(); t.ui.secondary = "#FABD2F".into();
        t
    }

    pub fn matrix() -> AppTheme {
        let mut t = Self::hacker_green();
        t.id = "matrix".into(); t.name = "Matrix".into();
        t.terminal.background = "#000000".into(); t.terminal.opacity = 1.0;
        t
    }

    pub fn cyberpunk() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "cyberpunk".into(); t.name = "Cyberpunk".into();
        t.terminal.background = "#000B18".into(); t.ui.primary = "#00FF9F".into();
        t.ui.secondary = "#00B8FF".into(); t.terminal.cursor = "#FF003C".into();
        t
    }

    pub fn minimal_black() -> AppTheme {
        let mut t = Self::flux_dark();
        t.id = "minimal_black".into(); t.name = "Minimal Black".into();
        t.terminal.background = "#000000".into(); t.ui.background = "#000000".into();
        t.ui.surface = "#000000".into(); t.ui.divider = "#333333".into();
        t.window.glass_effect = false;
        t
    }
}
