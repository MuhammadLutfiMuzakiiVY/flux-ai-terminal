//! Terminal emulation engine - PTY, multi-tab, split views, color themes

use crate::{FluxError, FluxResult, filesystem::VirtualFilesystem, shell::ShellType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

pub mod theme;

/// Terminal tab
pub struct TerminalTab {
    pub id: String,
    pub title: String,
    pub shell_type: ShellType,
    pub buffer: TerminalBuffer,
    pub cursor: CursorPos,
    pub scroll_offset: usize,
    pub is_active: bool,
    pub split: Option<SplitLayout>,
}

#[derive(Debug, Clone)]
pub struct CursorPos {
    pub row: usize,
    pub col: usize,
    pub visible: bool,
}

// Removed TerminalTheme as it's handled by theme::ThemeManager now.

#[derive(Debug, Clone)]
pub enum SplitLayout {
    Horizontal { ratio: f32, top: String, bottom: String },
    Vertical { ratio: f32, left: String, right: String },
}

/// Terminal character buffer
pub struct TerminalBuffer {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<Cell>>,
    pub scrollback: Vec<Vec<Cell>>,
    pub max_scrollback: usize,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub ch: char,
    pub fg: u8,
    pub bg: u8,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub inverse: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', fg: 7, bg: 0, bold: false, italic: false, underline: false, blink: false, inverse: false }
    }
}

impl TerminalBuffer {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![Cell::default(); cols]; rows];
        Self { rows, cols, cells, scrollback: Vec::new(), max_scrollback: 10000 }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
        self.cells.resize(rows, vec![Cell::default(); cols]);
        for row in &mut self.cells {
            row.resize(cols, Cell::default());
        }
    }

    pub fn write_char(&mut self, row: usize, col: usize, ch: char, fg: u8, bg: u8) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = Cell { ch, fg, bg, ..Cell::default() };
        }
    }

    pub fn scroll_up(&mut self) {
        if !self.cells.is_empty() {
            let top_row = self.cells.remove(0);
            if self.scrollback.len() >= self.max_scrollback {
                self.scrollback.remove(0);
            }
            self.scrollback.push(top_row);
            self.cells.push(vec![Cell::default(); self.cols]);
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![vec![Cell::default(); self.cols]; self.rows];
    }
}

impl TerminalTab {
    pub fn new(shell_type: ShellType, _fs: &VirtualFilesystem) -> FluxResult<Self> {
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            title: format!("{}", shell_type),
            shell_type,
            buffer: TerminalBuffer::new(24, 80),
            cursor: CursorPos { row: 0, col: 0, visible: true },
            scroll_offset: 0,
            is_active: true,
            split: None,
        })
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.buffer.resize(rows, cols);
    }

    pub fn write_output(&mut self, text: &str) {
        for ch in text.chars() {
            match ch {
                '\n' => {
                    self.cursor.row += 1;
                    self.cursor.col = 0;
                    if self.cursor.row >= self.buffer.rows {
                        self.buffer.scroll_up();
                        self.cursor.row = self.buffer.rows - 1;
                    }
                }
                '\r' => { self.cursor.col = 0; }
                '\x08' => { if self.cursor.col > 0 { self.cursor.col -= 1; } }
                _ => {
                    self.buffer.write_char(self.cursor.row, self.cursor.col, ch, 7, 0);
                    self.cursor.col += 1;
                    if self.cursor.col >= self.buffer.cols {
                        self.cursor.col = 0;
                        self.cursor.row += 1;
                        if self.cursor.row >= self.buffer.rows {
                            self.buffer.scroll_up();
                            self.cursor.row = self.buffer.rows - 1;
                        }
                    }
                }
            }
        }
    }

    pub fn split_horizontal(&mut self, ratio: f32) -> FluxResult<String> {
        let new_id = Uuid::new_v4().to_string();
        self.split = Some(SplitLayout::Horizontal {
            ratio,
            top: self.id.clone(),
            bottom: new_id.clone(),
        });
        Ok(new_id)
    }

    pub fn split_vertical(&mut self, ratio: f32) -> FluxResult<String> {
        let new_id = Uuid::new_v4().to_string();
        self.split = Some(SplitLayout::Vertical {
            ratio,
            left: self.id.clone(),
            right: new_id.clone(),
        });
        Ok(new_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardShortcut {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub action: ShortcutAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShortcutAction {
    NewTab,
    CloseTab,
    NextTab,
    PrevTab,
    SplitHorizontal,
    SplitVertical,
    FocusNextPane,
    FocusPrevPane,
    ToggleFullscreen,
    CopySelection,
    PasteClipboard,
    ClearBuffer,
    ScrollUp,
    ScrollDown,
}

pub struct TerminalSession {
    pub id: String,
    pub name: String,
    pub tabs: HashMap<String, TerminalTab>,
    pub active_tab: Option<String>,
}

impl TerminalSession {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            tabs: HashMap::new(),
            active_tab: None,
        }
    }
}

/// Terminal manager handles multiple sessions, tabs, and shortcuts
pub struct TerminalManager {
    pub sessions: HashMap<String, TerminalSession>,
    pub active_session: Option<String>,
    pub theme_manager: theme::ThemeManager,
    pub shortcuts: Vec<KeyboardShortcut>,
}

impl TerminalManager {
    pub fn new() -> Self {
        let mut tm = Self {
            sessions: HashMap::new(),
            active_session: None,
            theme_manager: theme::ThemeManager::new(),
            shortcuts: vec![
                KeyboardShortcut { key: "t".into(), ctrl: true, alt: false, shift: true, action: ShortcutAction::NewTab },
                KeyboardShortcut { key: "w".into(), ctrl: true, alt: false, shift: true, action: ShortcutAction::CloseTab },
                KeyboardShortcut { key: "\"".into(), ctrl: true, alt: false, shift: true, action: ShortcutAction::SplitHorizontal },
                KeyboardShortcut { key: "%".into(), ctrl: true, alt: false, shift: true, action: ShortcutAction::SplitVertical },
            ],
        };
        // Create default session
        let default_session = TerminalSession::new("default");
        tm.active_session = Some(default_session.id.clone());
        tm.sessions.insert(default_session.id.clone(), default_session);
        tm
    }

    pub fn add_tab(&mut self, session_id: &str, tab: TerminalTab) -> FluxResult<()> {
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| FluxError::Terminal("Session not found".into()))?;
        let id = tab.id.clone();
        session.tabs.insert(id.clone(), tab);
        if session.active_tab.is_none() {
            session.active_tab = Some(id);
        }
        Ok(())
    }

    pub fn close_tab(&mut self, session_id: &str, tab_id: &str) {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.tabs.remove(tab_id);
            if session.active_tab.as_deref() == Some(tab_id) {
                session.active_tab = session.tabs.keys().next().cloned();
            }
        }
    }

    pub fn active_tab(&self) -> Option<&TerminalTab> {
        let sid = self.active_session.as_ref()?;
        let session = self.sessions.get(sid)?;
        let tid = session.active_tab.as_ref()?;
        session.tabs.get(tid)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut TerminalTab> {
        let sid = self.active_session.clone()?;
        let session = self.sessions.get_mut(&sid)?;
        let tid = session.active_tab.clone()?;
        session.tabs.get_mut(&tid)
    }
}
