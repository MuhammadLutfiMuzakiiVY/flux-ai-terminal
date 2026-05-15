//! Debian/Ubuntu-style apt package manager
use crate::{FluxResult, filesystem::VirtualFilesystem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod dpkg;
pub use dpkg::DpkgManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub size: u64,
    pub installed: bool,
    pub dependencies: Vec<String>,
    pub section: String,
}

pub struct AptPackageManager {
    pub packages: HashMap<String, Package>,
    pub installed: HashMap<String, Package>,
    pub sources: Vec<String>,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
    pub dpkg: DpkgManager,
}

impl AptPackageManager {
    pub fn new(_fs: &VirtualFilesystem) -> FluxResult<Self> {
        let dpkg = DpkgManager::new("/home/flux")?;
        let mut packages = HashMap::new();
        let repo = vec![
            ("coreutils", "1.0.0", "GNU core utilities", "admin"),
            ("bash", "5.2.0", "The GNU Bourne Again shell", "shells"),
            ("zsh", "5.9", "A shell designed for interactive use", "shells"),
            ("git", "2.40.0", "Fast, scalable, distributed revision control system", "vcs"),
            ("curl", "8.0.0", "Command line tool for transferring data with URL syntax", "net"),
            ("wget", "1.21.0", "Retrieves files from the web", "net"),
            ("python3", "3.11.4", "Interactive high-level object-oriented language", "python"),
            ("pip", "23.1", "The Python package installer", "python"),
            ("nodejs", "18.16.0", "Evented I/O for V8 javascript", "web"),
            ("npm", "9.5.1", "Package manager for Node.js", "web"),
            ("typescript", "5.0", "Typed superset of JavaScript", "web"),
            ("clang", "16.0.0", "C, C++ and Objective-C compiler", "devel"),
            ("gcc", "13.1.0", "GNU Compiler Collection", "devel"),
            ("make", "4.3", "Utility for directing compilation", "devel"),
            ("rustc", "1.72.0", "Rust Systems Programming Language", "devel"),
            ("cargo", "1.72.0", "Rust package manager", "devel"),
            ("golang", "1.21.0", "Go programming language compiler", "devel"),
            ("openjdk-17-jdk", "17.0", "OpenJDK Development Kit (JDK)", "devel"),
            ("kotlin", "1.9.0", "Statically typed programming language for the JVM", "devel"),
            ("swift", "5.8", "Swift programming language compiler", "devel"),
            ("dart", "3.0", "Dart SDK for Flutter and Web", "devel"),
            ("php", "8.2", "Server-side, HTML-embedded scripting language", "web"),
            ("ruby", "3.2.0", "Interpreter of object-oriented scripting language", "devel"),
            ("perl", "5.36", "Larry Wall's Practical Extraction and Report Language", "devel"),
            ("lua", "5.4", "Powerful, fast, lightweight, embeddable scripting language", "devel"),
            ("sqlite3", "3.41.0", "Command line interface for SQLite 3", "database"),
            ("postgresql-client", "15.0", "Front-end programs for PostgreSQL", "database"),
            ("redis-tools", "7.0", "Persistent key-value database with network interface", "database"),
            ("vim", "9.1.0", "Vi IMproved text editor", "editors"),
            ("neovim", "0.10.0", "Modern Vim fork", "editors"),
            ("nano", "7.2", "Simple text editor", "editors"),
            ("tmux", "3.4", "Terminal multiplexer", "utils"),
            ("screen", "4.9.1", "Terminal multiplexer", "utils"),
            ("htop", "3.3.0", "Interactive process viewer", "utils"),
            ("sqlite3", "3.45.2", "SQLite database", "database"),
            ("php", "8.3.4", "PHP interpreter", "web"),
            ("golang-go", "1.22.2", "Go programming language", "devel"),
            ("ruby", "3.3.0", "Ruby language", "devel"),
            ("perl", "5.38.2", "Perl language", "devel"),
            ("lua5.4", "5.4.6", "Lua language", "devel"),
            ("zip", "3.0", "Archive compressor", "utils"),
            ("unzip", "6.0", "Archive decompressor", "utils"),
            ("tar", "1.35", "Archive utility", "utils"),
            ("jq", "1.7.1", "JSON processor", "utils"),
            ("tree", "2.1.1", "Directory listing", "utils"),
            ("ripgrep", "14.1.0", "Fast search tool", "utils"),
            ("fd-find", "9.0.0", "Fast file finder", "utils"),
            ("bat", "0.24.0", "Cat with syntax highlighting", "utils"),
            ("fzf", "0.49.0", "Fuzzy finder", "utils"),
            ("docker.io", "24.0.7", "Container runtime", "admin"),
            ("nginx", "1.24.0", "Web server", "web"),
            ("redis-server", "7.2.4", "In-memory database", "database"),
            ("postgresql", "16.2", "PostgreSQL database", "database"),
        ];

        for (name, ver, desc, section) in repo {
            packages.insert(name.into(), Package {
                name: name.into(), version: ver.into(), description: desc.into(),
                size: 1024 * (rand::random::<u64>() % 50 + 1),
                installed: false, dependencies: Vec::new(), section: section.into(),
            });
        }

        Ok(Self {
            packages,
            installed: HashMap::new(),
            sources: vec!["deb https://repo.fluxai.app/debian stable main".into()],
            last_update: None,
            dpkg,
        })
    }

    pub async fn handle_apt_command(&mut self, args: &[String]) -> FluxResult<(String, String, i32)> {
        if args.is_empty() {
            return Ok(("Usage: apt [update|upgrade|install|remove|search|list|show] ...".into(), String::new(), 1));
        }
        match args[0].as_str() {
            "update" => self.apt_update().await,
            "upgrade" => self.apt_upgrade().await,
            "install" => {
                if args.len() < 2 { return Ok(("".into(), "E: No package specified".into(), 1)); }
                self.apt_install(&args[1..]).await
            }
            "remove" | "purge" => {
                if args.len() < 2 { return Ok(("".into(), "E: No package specified".into(), 1)); }
                self.apt_remove(&args[1]).await
            }
            "search" => {
                if args.len() < 2 { return Ok(("".into(), "E: No search term".into(), 1)); }
                Ok((self.apt_search(&args[1]), String::new(), 0))
            }
            "list" => {
                let flag = args.get(1).map(|s| s.as_str()).unwrap_or("");
                Ok((self.apt_list(flag), String::new(), 0))
            }
            "show" => {
                if args.len() < 2 { return Ok(("".into(), "E: No package specified".into(), 1)); }
                self.apt_show(&args[1])
            }
            "autoremove" => Ok(("0 packages to remove.".into(), String::new(), 0)),
            _ => Ok(("".into(), format!("E: Invalid operation {}", args[0]), 1)),
        }
    }

    pub async fn handle_dpkg_command(&self, args: &[String]) -> FluxResult<(String, String, i32)> {
        if args.is_empty() { return Ok(("dpkg: need an action".into(), String::new(), 1)); }
        match args[0].as_str() {
            "-l" | "--list" => {
                let mut out = String::from("Desired=Unknown/Install/Remove\n| Status=Not/Inst\n||/ Name           Version      Description\n+++-==============-============-===================\n");
                for pkg in self.installed.values() {
                    out.push_str(&format!("ii  {:<15}{:<13}{}\n", pkg.name, pkg.version, pkg.description));
                }
                Ok((out, String::new(), 0))
            }
            "-s" | "--status" => {
                if let Some(name) = args.get(1) {
                    if let Some(pkg) = self.installed.get(name.as_str()) {
                        Ok((format!("Package: {}\nStatus: install ok installed\nVersion: {}\nDescription: {}\n", pkg.name, pkg.version, pkg.description), String::new(), 0))
                    } else {
                        Ok(("".into(), format!("dpkg-query: package '{}' is not installed", name), 1))
                    }
                } else { Ok(("".into(), "dpkg: need a package name".into(), 1)) }
            }
            _ => Ok(("".into(), format!("dpkg: unknown option {}", args[0]), 1)),
        }
    }

    async fn apt_update(&mut self) -> FluxResult<(String, String, i32)> {
        self.last_update = Some(chrono::Utc::now());
        let mut out = String::new();
        for src in &self.sources {
            out.push_str(&format!("Hit:1 {} amd64 Packages\n", src));
        }
        out.push_str(&format!("Reading package lists... Done\nBuilding dependency tree... Done\n{} packages can be upgraded.\n", self.packages.len()));
        Ok((out, String::new(), 0))
    }

    async fn apt_upgrade(&mut self) -> FluxResult<(String, String, i32)> {
        let count = self.installed.len();
        Ok((format!("{} upgraded, 0 newly installed, 0 to remove.\nAll packages are up to date.", count), String::new(), 0))
    }

    async fn apt_install(&mut self, names: &[String]) -> FluxResult<(String, String, i32)> {
        let mut output = String::new();
        for name in names {
            let name = name.trim_start_matches('-');
            if self.installed.contains_key(name) {
                output.push_str(&format!("{} is already the newest version.\n", name));
                continue;
            }
            if let Some(pkg) = self.packages.get(name).cloned() {
                output.push_str(&format!(
                    "Reading package lists... Done\nBuilding dependency tree... Done\n\
                     The following NEW packages will be installed:\n  {}\n\
                     0 upgraded, 1 newly installed, 0 to remove.\n\
                     Need to get {} kB of archives.\n\
                     Get:1 https://repo.fluxai.app/debian stable/main {} {} [{}kB]\n\
                     Selecting previously unselected package {}.\n\
                     Setting up {} ({}) ...\n",
                    pkg.name, pkg.size, pkg.name, pkg.version, pkg.size, pkg.name, pkg.name, pkg.version
                ));
                self.installed.insert(name.to_string(), pkg);
            } else {
                output.push_str(&format!("E: Unable to locate package {}\n", name));
                return Ok((output, String::new(), 100));
            }
        }
        Ok((output, String::new(), 0))
    }

    async fn apt_remove(&mut self, name: &str) -> FluxResult<(String, String, i32)> {
        if self.installed.remove(name).is_some() {
            Ok((format!("Removing {} ...\nProcessing triggers ...\n", name), String::new(), 0))
        } else {
            Ok(("".into(), format!("E: Package '{}' is not installed", name), 1))
        }
    }

    fn apt_search(&self, query: &str) -> String {
        let mut out = String::new();
        for pkg in self.packages.values() {
            if pkg.name.contains(query) || pkg.description.to_lowercase().contains(&query.to_lowercase()) {
                let marker = if self.installed.contains_key(&pkg.name) { "[installed]" } else { "" };
                out.push_str(&format!("{}/{} {} {}\n  {}\n\n", pkg.name, pkg.section, pkg.version, marker, pkg.description));
            }
        }
        if out.is_empty() { out = format!("No packages found matching '{}'", query); }
        out
    }

    fn apt_list(&self, flag: &str) -> String {
        let mut out = String::from("Listing...\n");
        let iter: Box<dyn Iterator<Item = &Package>> = match flag {
            "--installed" => Box::new(self.installed.values()),
            _ => Box::new(self.packages.values()),
        };
        for pkg in iter {
            let status = if self.installed.contains_key(&pkg.name) { "[installed]" } else { "" };
            out.push_str(&format!("{}/{} {} {} {}\n", pkg.name, pkg.section, pkg.version, std::env::consts::ARCH, status));
        }
        out
    }

    fn apt_show(&self, name: &str) -> FluxResult<(String, String, i32)> {
        let pkg = self.packages.get(name).or_else(|| self.installed.get(name));
        match pkg {
            Some(p) => Ok((format!(
                "Package: {}\nVersion: {}\nSection: {}\nInstalled-Size: {} kB\nDescription: {}\nMaintainer: Muhammad Lutfi Muzaki Dev\n",
                p.name, p.version, p.section, p.size, p.description
            ), String::new(), 0)),
            None => Ok(("".into(), format!("E: No packages found for {}", name), 1)),
        }
    }
}
