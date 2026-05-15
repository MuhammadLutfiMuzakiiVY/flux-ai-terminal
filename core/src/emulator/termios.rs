//! Terminal I/O Settings (termios)
//! 
//! Manages low-level terminal attributes like baud rate, character size,
//! local echo, and canonical mode for full native Linux experience.

use crate::{FluxResult, FluxError};
use nix::sys::termios::{tcgetattr, tcsetattr, Termios, LocalFlags, InputFlags, OutputFlags, SetArg};
use std::os::unix::io::RawFd;

pub struct TerminalConfig {
    original_termios: Termios,
    fd: RawFd,
}

impl TerminalConfig {
    pub fn new(fd: RawFd) -> FluxResult<Self> {
        let original_termios = tcgetattr(fd)
            .map_err(|e| FluxError::Terminal(format!("tcgetattr failed: {}", e)))?;
            
        Ok(Self { original_termios, fd })
    }

    /// Enable Raw Mode (used by VIM, Emacs, and complex CLI tools)
    pub fn enable_raw_mode(&self) -> FluxResult<()> {
        let mut raw = self.original_termios.clone();
        
        // Disable ECHO, Canonical mode, Extended Input Processing, and Signals
        raw.local_flags.remove(LocalFlags::ECHO | LocalFlags::ICANON | LocalFlags::IEXTEN | LocalFlags::ISIG);
        
        // Disable software flow control
        raw.input_flags.remove(InputFlags::IXON | InputFlags::ICRNL);
        
        // Disable output processing
        raw.output_flags.remove(OutputFlags::OPOST);
        
        tcsetattr(self.fd, SetArg::TCSADRAIN, &raw)
            .map_err(|e| FluxError::Terminal(format!("tcsetattr failed: {}", e)))
    }

    /// Restore terminal to its original cooked mode
    pub fn restore(&self) -> FluxResult<()> {
        tcsetattr(self.fd, SetArg::TCSADRAIN, &self.original_termios)
            .map_err(|e| FluxError::Terminal(format!("Restore tcsetattr failed: {}", e)))
    }
}
