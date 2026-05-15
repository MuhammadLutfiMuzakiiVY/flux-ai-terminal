//! Pseudo-Terminal (PTY) Implementation for Android & iOS
//! 
//! This module binds directly to the low-level UNIX PTY master/slave interface
//! via the `nix` crate to provide true native terminal emulation (just like Linux).

use crate::{FluxResult, FluxError};
use nix::pty::{posix_openpt, grantpt, unlockpt, ptsname, PtyMaster};
use nix::fcntl::{OFlag, open};
use nix::sys::stat::Mode;
use nix::unistd::{setsid, dup2, execvp, close, read, write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ffi::CString;

pub struct NativePty {
    pub master_fd: RawFd,
    pub slave_name: String,
    pub child_pid: Option<i32>,
}

impl NativePty {
    pub fn new() -> FluxResult<Self> {
        // Open PTY master
        let master = posix_openpt(OFlag::O_RDWR | OFlag::O_NOCTTY)
            .map_err(|e| FluxError::Terminal(format!("Failed to open PTY master: {}", e)))?;
        
        grantpt(&master).map_err(|e| FluxError::Terminal(e.to_string()))?;
        unlockpt(&master).map_err(|e| FluxError::Terminal(e.to_string()))?;
        
        let slave_name = unsafe { ptsname(&master) }
            .map_err(|e| FluxError::Terminal(e.to_string()))?;
            
        Ok(Self {
            master_fd: master.as_raw_fd(),
            slave_name,
            child_pid: None,
        })
    }

    /// Fork the current process and attach the slave PTY to the child's std streams
    pub fn spawn_shell(&mut self, shell_path: &str) -> FluxResult<()> {
        match unsafe { nix::unistd::fork() } {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                self.child_pid = Some(child.into());
                Ok(())
            }
            Ok(nix::unistd::ForkResult::Child) => {
                // We are in the child process
                let _ = setsid(); // Create a new session
                
                // Open slave PTY
                let slave_fd = open(self.slave_name.as_str(), OFlag::O_RDWR, Mode::empty()).unwrap();
                
                // Redirect standard streams to the slave PTY
                let _ = dup2(slave_fd, 0); // stdin
                let _ = dup2(slave_fd, 1); // stdout
                let _ = dup2(slave_fd, 2); // stderr
                
                let _ = close(slave_fd);
                
                // Execute the shell
                let c_shell = CString::new(shell_path).unwrap();
                let _ = execvp(&c_shell, &[&c_shell]);
                
                std::process::exit(1);
            }
            Err(e) => Err(FluxError::Terminal(format!("Fork failed: {}", e))),
        }
    }

    pub fn write_to_master(&self, data: &[u8]) -> FluxResult<usize> {
        write(self.master_fd, data).map_err(|e| FluxError::Terminal(e.to_string()))
    }

    pub fn read_from_master(&self, buffer: &mut [u8]) -> FluxResult<usize> {
        read(self.master_fd, buffer).map_err(|e| FluxError::Terminal(e.to_string()))
    }
}
