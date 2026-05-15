//! Virtual process manager with PID tracking
use crate::FluxResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub command: String,
    pub status: ProcessStatus,
    pub cpu_percent: f32,
    pub memory_kb: u64,
    pub started_at: DateTime<Utc>,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessStatus { Running, Sleeping, Stopped, Zombie }

impl std::fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessStatus::Running => write!(f, "R"),
            ProcessStatus::Sleeping => write!(f, "S"),
            ProcessStatus::Stopped => write!(f, "T"),
            ProcessStatus::Zombie => write!(f, "Z"),
        }
    }
}

pub struct ProcessManager {
    processes: HashMap<u32, ProcessInfo>,
    next_pid: u32,
}

impl ProcessManager {
    pub fn new() -> Self {
        let mut pm = Self { processes: HashMap::new(), next_pid: 100 };
        // Init system processes
        let init_procs = vec![
            (1, 0, "init", "/sbin/init"),
            (2, 1, "kthreadd", "[kthreadd]"),
            (10, 1, "flux-core", "/usr/bin/flux-core"),
            (11, 10, "flux-shell", "/bin/bash"),
            (12, 10, "flux-ai", "/usr/bin/flux-ai"),
        ];
        for (pid, ppid, name, cmd) in init_procs {
            pm.processes.insert(pid, ProcessInfo {
                pid, ppid, name: name.into(), command: cmd.into(),
                status: ProcessStatus::Running, cpu_percent: 0.0, memory_kb: 1024,
                started_at: Utc::now(), user: "root".into(),
            });
        }
        pm
    }

    pub fn spawn(&mut self, name: &str, command: &str, user: &str) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;
        self.processes.insert(pid, ProcessInfo {
            pid, ppid: 11, name: name.into(), command: command.into(),
            status: ProcessStatus::Running, cpu_percent: 0.0, memory_kb: 512,
            started_at: Utc::now(), user: user.into(),
        });
        pid
    }

    pub fn kill(&mut self, pid: u32, _signal: i32) -> FluxResult<()> {
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.status = ProcessStatus::Zombie;
            Ok(())
        } else {
            Err(crate::FluxError::NotFound(format!("No such process: {}", pid)))
        }
    }

    pub fn list(&self) -> Vec<&ProcessInfo> {
        let mut procs: Vec<_> = self.processes.values().collect();
        procs.sort_by_key(|p| p.pid);
        procs
    }

    pub fn ps_output(&self) -> String {
        let mut out = format!("{:<6} {:<6} {:<4} {:<6} {:<8} {}\n", "PID", "PPID", "STAT", "%CPU", "MEM(KB)", "COMMAND");
        for p in self.list() {
            out.push_str(&format!("{:<6} {:<6} {:<4} {:<6.1} {:<8} {}\n",
                p.pid, p.ppid, p.status, p.cpu_percent, p.memory_kb, p.command));
        }
        out
    }

    pub fn get(&self, pid: u32) -> Option<&ProcessInfo> { self.processes.get(&pid) }
    pub fn cleanup_zombies(&mut self) { self.processes.retain(|_, p| p.status != ProcessStatus::Zombie); }
}
