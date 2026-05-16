use crate::{FluxResult, filesystem::VirtualFilesystem};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono;
use rand;

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

/// 🧠 Advanced Dependency Graph for complex package resolution
pub struct DependencyGraph {
    pub nodes: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn add_package(&mut self, name: &str, deps: Vec<String>) {
        self.nodes.insert(name.to_string(), deps);
    }

    pub fn resolve_order(&self, start_nodes: &[String]) -> Result<Vec<String>, String> {
        let mut order = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        for node in start_nodes {
            self.visit(node, &mut visited, &mut visiting, &mut order)?;
        }

        Ok(order)
    }

    fn visit(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), String> {
        if visiting.contains(node) {
            return Err(format!("Circular dependency detected: {}", node));
        }
        if !visited.contains(node) {
            visiting.insert(node.to_string());
            if let Some(deps) = self.nodes.get(node) {
                for dep in deps {
                    self.visit(dep, visited, visiting, order)?;
                }
            }
            visiting.remove(node);
            visited.insert(node.to_string());
            order.push(node.to_string());
        }
        Ok(())
    }
}

pub struct AptPackageManager {
    pub packages: HashMap<String, Package>,
    pub installed: HashMap<String, Package>,
    pub sources: Vec<String>,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
    pub dpkg: DpkgManager,
    pub dep_graph: DependencyGraph,
}

impl AptPackageManager {
    pub fn new(_fs: &VirtualFilesystem) -> FluxResult<Self> {
        let dpkg = DpkgManager::new("/home/flux")?;
        let mut packages = HashMap::new();
        let mut dep_graph = DependencyGraph::new();
        
        let repo = vec![
            ("coreutils", "1.0.0", "GNU core utilities", "admin", vec![]),
            ("bash", "5.2.0", "The GNU Bourne Again shell", "shells", vec!["coreutils".to_string()]),
            ("python3", "3.11.4", "Interactive high-level object-oriented language", "python", vec!["coreutils".to_string()]),
            ("pip", "23.1", "The Python package installer", "python", vec!["python3".to_string()]),
            ("nodejs", "18.16.0", "Evented I/O for V8 javascript", "web", vec!["coreutils".to_string()]),
            ("rustc", "1.78.0", "Rust compiler", "devel", vec!["coreutils".to_string()]),
        ];

        for (name, ver, desc, section, deps) in repo {
            packages.insert(name.into(), Package {
                name: name.into(), version: ver.into(), description: desc.into(),
                size: 1024 * (rand::random::<u64>() % 50 + 1),
                installed: false, dependencies: deps.clone(), section: section.into(),
            });
            dep_graph.add_package(name, deps);
        }

        Ok(Self {
            packages,
            installed: HashMap::new(),
            sources: vec!["deb https://repo.fluxai.app/debian stable main".into()],
            last_update: None,
            dpkg,
            dep_graph,
        })
    }

    pub async fn handle_apt_command(&mut self, args: &[String]) -> FluxResult<(String, String, i32)> {
        if args.is_empty() {
            return Ok(("Usage: apt [update|upgrade|install|remove|search|list|show] ...".into(), String::new(), 1));
        }
        match args[0].as_str() {
            "update" => self.apt_update().await,
            "install" => {
                if args.len() < 2 { return Ok(("".into(), "E: No package specified".into(), 1)); }
                self.apt_install(&args[1..]).await
            }
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
        Ok(("Reading package lists... Done\nBuilding dependency tree... Done\n".into(), String::new(), 0))
    }

    async fn apt_install(&mut self, names: &[String]) -> FluxResult<(String, String, i32)> {
        let order = match self.dep_graph.resolve_order(names) {
            Ok(o) => o,
            Err(e) => return Ok(("".into(), format!("E: Dependency error: {}", e), 1)),
        };

        let mut output = format!("The following extra packages will be installed:\n  {:?}\n", order);
        for name in order {
            if let Some(pkg) = self.packages.get(&name).cloned() {
                output.push_str(&format!("Setting up {} ({}) ...\n", pkg.name, pkg.version));
                self.installed.insert(name, pkg);
            }
        }
        Ok((output, String::new(), 0))
    }
}
