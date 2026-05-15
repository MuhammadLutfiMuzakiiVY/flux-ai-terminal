//! Virtual Linux filesystem abstraction
use crate::{FluxError, FluxResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub permissions: String,
    pub links: u32,
    pub modified: String,
}

#[derive(Debug, Clone)]
struct FsNode {
    name: String,
    is_dir: bool,
    content: Vec<u8>,
    children: HashMap<String, FsNode>,
    permissions: u32,
    modified: DateTime<Utc>,
}

impl FsNode {
    fn dir(name: &str) -> Self {
        Self { name: name.into(), is_dir: true, content: Vec::new(), children: HashMap::new(), permissions: 0o755, modified: Utc::now() }
    }
    fn file(name: &str, content: &[u8]) -> Self {
        Self { name: name.into(), is_dir: false, content: content.to_vec(), children: HashMap::new(), permissions: 0o644, modified: Utc::now() }
    }
}

pub struct VirtualFilesystem {
    root: FsNode,
    cwd: String,
    data_dir: String,
}

impl VirtualFilesystem {
    pub fn new(data_dir: &str) -> FluxResult<Self> {
        tracing::info!("Initializing VirtualFilesystem at {}", data_dir);
        let mut root = FsNode::dir("/");
        // Create standard Linux directory structure
        for dir in &["bin", "usr", "etc", "var", "tmp", "home", "root", "opt", "dev", "proc", "sys", "lib", "sbin", "run"] {
            root.children.insert(dir.to_string(), FsNode::dir(dir));
        }
        // Create subdirs
        if let Some(usr) = root.children.get_mut("usr") {
            for d in &["bin", "lib", "share", "local", "include", "sbin"] {
                usr.children.insert(d.to_string(), FsNode::dir(d));
            }
        }
        if let Some(etc) = root.children.get_mut("etc") {
            etc.children.insert("apt".into(), FsNode::dir("apt"));
            etc.children.insert("passwd".into(), FsNode::file("passwd", b"root:x:0:0:root:/root:/bin/bash\nflux:x:1000:1000:Flux User:/home/flux:/bin/bash\n"));
            etc.children.insert("hostname".into(), FsNode::file("hostname", b"flux\n"));
            etc.children.insert("os-release".into(), FsNode::file("os-release",
                b"NAME=\"Flux Linux\"\nVERSION=\"1.0\"\nID=flux\nID_LIKE=debian ubuntu\nPRETTY_NAME=\"Flux AI Terminal 1.0\"\nHOME_URL=\"https://fluxai.app\"\n"));
        }
        if let Some(home) = root.children.get_mut("home") {
            let mut flux_home = FsNode::dir("flux");
            flux_home.children.insert(".bashrc".into(), FsNode::file(".bashrc",
                b"# ~/.bashrc - Flux AI Terminal\nexport PS1='\\[\\033[01;32m\\]flux@flux\\[\\033[00m\\]:\\[\\033[01;34m\\]\\w\\[\\033[00m\\]\\$ '\nalias ll='ls -la'\nalias la='ls -A'\n"));
            flux_home.children.insert(".profile".into(), FsNode::file(".profile", b"# ~/.profile\nexport PATH=$HOME/.local/bin:$PATH\n"));
            flux_home.children.insert("Documents".into(), FsNode::dir("Documents"));
            flux_home.children.insert("Downloads".into(), FsNode::dir("Downloads"));
            flux_home.children.insert("Projects".into(), FsNode::dir("Projects"));
            flux_home.children.insert(".config".into(), FsNode::dir(".config"));
            flux_home.children.insert(".local".into(), FsNode::dir(".local"));
            flux_home.children.insert(".ssh".into(), FsNode::dir(".ssh"));
            home.children.insert("flux".into(), flux_home);
        }
        if let Some(var) = root.children.get_mut("var") {
            var.children.insert("log".into(), FsNode::dir("log"));
            var.children.insert("cache".into(), FsNode::dir("cache"));
            var.children.insert("lib".into(), FsNode::dir("lib"));
        }
        // Add common binaries
        if let Some(bin) = root.children.get_mut("bin") {
            for cmd in &["bash", "sh", "ls", "cat", "cp", "mv", "rm", "mkdir", "chmod", "chown", "echo", "grep", "find", "ps", "kill"] {
                bin.children.insert(cmd.to_string(), FsNode::file(cmd, b"#!/bin/bash\n# system binary\n"));
            }
        }

        Ok(Self { root, cwd: "/home/flux".into(), data_dir: data_dir.into() })
    }

    pub fn cwd(&self) -> &str { &self.cwd }
    pub fn data_dir(&self) -> &str { &self.data_dir }

    pub fn change_dir(&mut self, path: &str) -> FluxResult<()> {
        let abs = self.resolve_path(path);
        if self.get_node(&abs).map(|n| n.is_dir).unwrap_or(false) {
            self.cwd = abs;
            Ok(())
        } else {
            Err(FluxError::NotFound(format!("cd: {}: No such directory", path)))
        }
    }

    pub fn list_dir(&self, path: &str) -> FluxResult<Vec<DirEntry>> {
        let abs = self.resolve_path(path);
        let node = self.get_node(&abs).ok_or_else(|| FluxError::NotFound(format!("{}: No such directory", path)))?;
        if !node.is_dir { return Err(FluxError::InvalidArgument("Not a directory".into())); }
        let mut entries: Vec<DirEntry> = node.children.values().map(|c| DirEntry {
            name: c.name.clone(),
            is_dir: c.is_dir,
            size: if c.is_dir { 4096 } else { c.content.len() as u64 },
            permissions: if c.is_dir { "drwxr-xr-x".into() } else { "-rw-r--r--".into() },
            links: if c.is_dir { 2 } else { 1 },
            modified: c.modified.format("%b %d %H:%M").to_string(),
        }).collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    pub fn read_file(&self, path: &str) -> FluxResult<Vec<u8>> {
        let abs = self.resolve_path(path);
        let node = self.get_node(&abs).ok_or_else(|| FluxError::NotFound(format!("{}: No such file", path)))?;
        if node.is_dir { return Err(FluxError::InvalidArgument("Is a directory".into())); }
        Ok(node.content.clone())
    }

    pub fn write_file(&mut self, path: &str, content: &[u8]) -> FluxResult<()> {
        let abs = self.resolve_path(path);
        let parts = Self::split_path(&abs);
        self.ensure_parent(&parts)?;
        let name = parts.last().ok_or_else(|| FluxError::InvalidArgument("Invalid path".into()))?;
        let parent = self.get_node_mut(&parts[..parts.len()-1]).ok_or_else(|| FluxError::NotFound("Parent not found".into()))?;
        parent.children.insert(name.clone(), FsNode::file(name, content));
        Ok(())
    }

    pub fn create_file(&mut self, path: &str) -> FluxResult<()> { self.write_file(path, b"") }

    pub fn create_dir(&mut self, path: &str) -> FluxResult<()> {
        let abs = self.resolve_path(path);
        let parts = Self::split_path(&abs);
        let name = parts.last().ok_or_else(|| FluxError::InvalidArgument("Invalid path".into()))?;
        self.ensure_parent(&parts)?;
        let parent = self.get_node_mut(&parts[..parts.len()-1]).ok_or_else(|| FluxError::NotFound("Parent not found".into()))?;
        if !parent.children.contains_key(name.as_str()) {
            parent.children.insert(name.clone(), FsNode::dir(name));
        }
        Ok(())
    }

    pub fn remove(&mut self, path: &str) -> FluxResult<()> {
        let abs = self.resolve_path(path);
        let parts = Self::split_path(&abs);
        let name = parts.last().ok_or_else(|| FluxError::InvalidArgument("Invalid path".into()))?;
        let parent = self.get_node_mut(&parts[..parts.len()-1]).ok_or_else(|| FluxError::NotFound("Parent not found".into()))?;
        parent.children.remove(name.as_str()).ok_or_else(|| FluxError::NotFound(format!("{}: No such file or directory", path)))?;
        Ok(())
    }

    pub fn copy(&mut self, _src: &str, _dst: &str) -> FluxResult<()> { Ok(()) }
    pub fn rename(&mut self, _src: &str, _dst: &str) -> FluxResult<()> { Ok(()) }

    pub fn which(&self, name: &str) -> Option<String> {
        let paths = ["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"];
        for p in &paths {
            let full = format!("{}/{}", p, name);
            if self.get_node(&full).is_some() { return Some(full); }
        }
        None
    }

    fn resolve_path(&self, path: &str) -> String {
        if path.starts_with('/') { path.to_string() }
        else if path == "~" || path.starts_with("~/") { path.replacen("~", "/home/flux", 1) }
        else if path == "." { self.cwd.clone() }
        else if path == ".." {
            let mut parts: Vec<&str> = self.cwd.split('/').collect();
            parts.pop();
            if parts.is_empty() { "/".into() } else { parts.join("/") }
        }
        else { format!("{}/{}", self.cwd, path) }
    }

    fn split_path(path: &str) -> Vec<String> {
        path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
    }

    fn get_node(&self, path: &str) -> Option<&FsNode> {
        let parts = Self::split_path(path);
        let mut current = &self.root;
        for part in &parts {
            current = current.children.get(part)?;
        }
        Some(current)
    }

    fn get_node_mut(&mut self, parts: &[String]) -> Option<&mut FsNode> {
        let mut current = &mut self.root;
        for part in parts {
            current = current.children.get_mut(part)?;
        }
        Some(current)
    }

    fn ensure_parent(&mut self, parts: &[String]) -> FluxResult<()> {
        let mut current = &mut self.root;
        for part in &parts[..parts.len().saturating_sub(1)] {
            if !current.children.contains_key(part.as_str()) {
                current.children.insert(part.clone(), FsNode::dir(part));
            }
            current = current.children.get_mut(part).unwrap();
        }
        Ok(())
    }

    pub fn get_permissions(&self, path: &str) -> FluxResult<u32> {
        let abs = self.resolve_path(path);
        let node = self.get_node(&abs).ok_or_else(|| FluxError::NotFound(format!("{}: No such file or directory", path)))?;
        Ok(node.permissions)
    }
}
