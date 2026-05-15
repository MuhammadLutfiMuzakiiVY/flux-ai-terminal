//! Shell engine - bash/zsh/sh interpreter with command parsing, history, aliases

use crate::{FluxError, FluxResult, config::FluxConfig, filesystem::VirtualFilesystem, package::AptPackageManager};
use serde::{Deserialize, Serialize};
use futures::future::{BoxFuture, FutureExt};

#[path = "../emulator/language_runner.rs"]
pub mod language_runner;
pub use language_runner::{LanguageRunner, SupportedLanguage};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Supported shell types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShellType {
    Bash,
    Zsh,
    Sh,
}

impl Default for ShellType {
    fn default() -> Self { ShellType::Bash }
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellType::Bash => write!(f, "bash"),
            ShellType::Zsh => write!(f, "zsh"),
            ShellType::Sh => write!(f, "sh"),
        }
    }
}

/// A shell session with its own state
pub struct ShellSession {
    pub id: String,
    pub shell_type: ShellType,
    pub env: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub history: Vec<HistoryEntry>,
    pub cwd: String,
    pub user: String,
    pub hostname: String,
    pub exit_code: i32,
    pub prompt: String,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: DateTime<Utc>,
    pub exit_code: i32,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
}

/// Parsed command representation
#[derive(Debug, Clone)]
pub enum ParsedCommand {
    Simple { program: String, args: Vec<String> },
    Pipeline(Vec<ParsedCommand>),
    And(Box<ParsedCommand>, Box<ParsedCommand>),
    Or(Box<ParsedCommand>, Box<ParsedCommand>),
    Background(Box<ParsedCommand>),
    Redirect { cmd: Box<ParsedCommand>, target: String, append: bool },
    Builtin(BuiltinCommand),
}

#[derive(Debug, Clone)]
pub enum BuiltinCommand {
    Cd(String),
    Export(String, String),
    Unset(String),
    Alias(String, String),
    Unalias(String),
    Echo(Vec<String>),
    Pwd,
    Exit(i32),
    History,
    Clear,
    Source(String),
    Whoami,
    Hostname,
    Uname(Vec<String>),
    Cat(Vec<String>),
    Ls(Vec<String>),
    Mkdir(Vec<String>),
    Rm(Vec<String>),
    Cp(String, String),
    Mv(String, String),
    Touch(Vec<String>),
    Chmod(String, String),
    Grep(Vec<String>),
    Head(Vec<String>),
    Tail(Vec<String>),
    Wc(Vec<String>),
    Which(String),
    Env,
    Set,
    Help,
}

impl ShellSession {
    pub fn new(
        shell_type: ShellType,
        _fs: &VirtualFilesystem,
        config: &FluxConfig,
    ) -> FluxResult<Self> {
        let mut env = HashMap::new();
        env.insert("HOME".into(), "/home/flux".into());
        env.insert("USER".into(), "flux".into());
        env.insert("SHELL".into(), format!("/bin/{}", shell_type));
        env.insert("PATH".into(), "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin".into());
        env.insert("TERM".into(), "xterm-256color".into());
        env.insert("LANG".into(), "en_US.UTF-8".into());
        env.insert("EDITOR".into(), "vim".into());
        env.insert("HOSTNAME".into(), "flux".into());
        env.insert("PS1".into(), config.terminal.prompt.clone());

        let mut aliases = HashMap::new();
        aliases.insert("ll".into(), "ls -la".into());
        aliases.insert("la".into(), "ls -A".into());
        aliases.insert("l".into(), "ls -CF".into());
        aliases.insert("..".into(), "cd ..".into());
        aliases.insert("...".into(), "cd ../..".into());
        aliases.insert("cls".into(), "clear".into());
        aliases.insert("py".into(), "python3".into());
        aliases.insert("node".into(), "nodejs".into());

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            shell_type,
            env,
            aliases,
            history: Vec::new(),
            cwd: "/home/flux".into(),
            user: "flux".into(),
            hostname: "flux".into(),
            exit_code: 0,
            prompt: config.terminal.prompt.clone(),
            created_at: Utc::now(),
            is_active: true,
        })
    }

    /// Get the formatted prompt string
    pub fn get_prompt(&self) -> String {
        self.prompt
            .replace("\\u", &self.user)
            .replace("\\h", &self.hostname)
            .replace("\\w", &self.cwd)
            .replace("\\$", if self.user == "root" { "#" } else { "$" })
    }

    /// Add a command to history
    pub fn add_history(&mut self, entry: HistoryEntry) {
        self.history.push(entry);
    }

    /// Search history
    pub fn search_history(&self, query: &str) -> Vec<&HistoryEntry> {
        self.history.iter().filter(|e| e.command.contains(query)).collect()
    }

    /// Resolve aliases in command
    pub fn resolve_alias(&self, cmd: &str) -> String {
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        if let Some(alias_val) = self.aliases.get(parts[0]) {
            if parts.len() > 1 {
                format!("{} {}", alias_val, parts[1])
            } else {
                alias_val.clone()
            }
        } else {
            cmd.to_string()
        }
    }
}

/// Parse a command string into structured representation
pub fn parse_command(input: &str) -> FluxResult<ParsedCommand> {
    let input = input.trim();
    if input.is_empty() {
        return Err(FluxError::Shell("Empty command".into()));
    }

    // Handle pipes
    if input.contains(" | ") {
        let parts: Vec<&str> = input.split(" | ").collect();
        let cmds: Result<Vec<_>, _> = parts.iter().map(|p| parse_command(p)).collect();
        return Ok(ParsedCommand::Pipeline(cmds?));
    }

    // Handle && operator
    if input.contains(" && ") {
        let parts: Vec<&str> = input.splitn(2, " && ").collect();
        return Ok(ParsedCommand::And(
            Box::new(parse_command(parts[0])?),
            Box::new(parse_command(parts[1])?),
        ));
    }

    // Handle || operator
    if input.contains(" || ") {
        let parts: Vec<&str> = input.splitn(2, " || ").collect();
        return Ok(ParsedCommand::Or(
            Box::new(parse_command(parts[0])?),
            Box::new(parse_command(parts[1])?),
        ));
    }

    // Handle background &
    if input.ends_with(" &") || input.ends_with("&") {
        let cmd = input.trim_end_matches('&').trim();
        return Ok(ParsedCommand::Background(Box::new(parse_command(cmd)?)));
    }

    // Handle redirects
    if input.contains(" >> ") {
        let parts: Vec<&str> = input.splitn(2, " >> ").collect();
        return Ok(ParsedCommand::Redirect {
            cmd: Box::new(parse_command(parts[0])?),
            target: parts[1].trim().to_string(),
            append: true,
        });
    }
    if input.contains(" > ") {
        let parts: Vec<&str> = input.splitn(2, " > ").collect();
        return Ok(ParsedCommand::Redirect {
            cmd: Box::new(parse_command(parts[0])?),
            target: parts[1].trim().to_string(),
            append: false,
        });
    }

    // Parse simple command
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Err(FluxError::Shell("Empty command".into()));
    }

    // Check for builtins
    if let Some(builtin) = try_parse_builtin(&tokens) {
        return Ok(ParsedCommand::Builtin(builtin));
    }

    Ok(ParsedCommand::Simple {
        program: tokens[0].clone(),
        args: tokens[1..].to_vec(),
    })
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escape_next = false;

    for ch in input.chars() {
        if escape_next {
            current.push(ch);
            escape_next = false;
            continue;
        }
        match ch {
            '\\' if !in_single_quote => escape_next = true,
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn try_parse_builtin(tokens: &[String]) -> Option<BuiltinCommand> {
    match tokens[0].as_str() {
        "cd" => Some(BuiltinCommand::Cd(tokens.get(1).cloned().unwrap_or_else(|| "~".into()))),
        "pwd" => Some(BuiltinCommand::Pwd),
        "echo" => Some(BuiltinCommand::Echo(tokens[1..].to_vec())),
        "exit" => Some(BuiltinCommand::Exit(tokens.get(1).and_then(|s| s.parse().ok()).unwrap_or(0))),
        "history" => Some(BuiltinCommand::History),
        "clear" => Some(BuiltinCommand::Clear),
        "whoami" => Some(BuiltinCommand::Whoami),
        "hostname" => Some(BuiltinCommand::Hostname),
        "env" => Some(BuiltinCommand::Env),
        "set" => Some(BuiltinCommand::Set),
        "help" => Some(BuiltinCommand::Help),
        "cat" => Some(BuiltinCommand::Cat(tokens[1..].to_vec())),
        "ls" => Some(BuiltinCommand::Ls(tokens[1..].to_vec())),
        "mkdir" => Some(BuiltinCommand::Mkdir(tokens[1..].to_vec())),
        "rm" => Some(BuiltinCommand::Rm(tokens[1..].to_vec())),
        "touch" => Some(BuiltinCommand::Touch(tokens[1..].to_vec())),
        "which" => tokens.get(1).map(|s| BuiltinCommand::Which(s.clone())),
        "uname" => Some(BuiltinCommand::Uname(tokens[1..].to_vec())),
        "head" => Some(BuiltinCommand::Head(tokens[1..].to_vec())),
        "tail" => Some(BuiltinCommand::Tail(tokens[1..].to_vec())),
        "wc" => Some(BuiltinCommand::Wc(tokens[1..].to_vec())),
        "grep" => Some(BuiltinCommand::Grep(tokens[1..].to_vec())),
        "export" => {
            if let Some(arg) = tokens.get(1) {
                let parts: Vec<&str> = arg.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some(BuiltinCommand::Export(parts[0].into(), parts[1].into()))
                } else {
                    None
                }
            } else { None }
        }
        "alias" => {
            if let Some(arg) = tokens.get(1) {
                let parts: Vec<&str> = arg.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some(BuiltinCommand::Alias(parts[0].into(), parts[1].trim_matches('\'').into()))
                } else { None }
                } else { None }
        }
        "unalias" => tokens.get(1).map(|s| BuiltinCommand::Unalias(s.clone())),
        "unset" => tokens.get(1).map(|s| BuiltinCommand::Unset(s.clone())),
        "source" | "." => tokens.get(1).map(|s| BuiltinCommand::Source(s.clone())),
        "cp" => {
            if tokens.len() >= 3 {
                Some(BuiltinCommand::Cp(tokens[1].clone(), tokens[2].clone()))
            } else { None }
        }
        "mv" => {
            if tokens.len() >= 3 {
                Some(BuiltinCommand::Mv(tokens[1].clone(), tokens[2].clone()))
            } else { None }
        }
        "chmod" => {
            if tokens.len() >= 3 {
                Some(BuiltinCommand::Chmod(tokens[1].clone(), tokens[2].clone()))
            } else { None }
        }
        _ => None,
    }
}

/// Execute a command in the Flux environment
pub async fn execute_command(
    input: &str,
    fs: &mut VirtualFilesystem,
    pkg: &mut AptPackageManager,
    config: &FluxConfig,
) -> FluxResult<CommandOutput> {
    let start = std::time::Instant::now();
    let parsed = parse_command(input)?;

    let (stdout, stderr, exit_code) = execute_parsed(&parsed, fs, pkg, config).await?;

    Ok(CommandOutput {
        stdout,
        stderr,
        exit_code,
        duration_ms: start.elapsed().as_millis() as u64,
    })
}

fn execute_parsed<'a>(
    cmd: &'a ParsedCommand,
    fs: &'a mut VirtualFilesystem,
    pkg: &'a mut AptPackageManager,
    config: &'a FluxConfig,
) -> BoxFuture<'a, FluxResult<(String, String, i32)>> {
    async move {
        match cmd {
            ParsedCommand::Builtin(builtin) => execute_builtin(builtin, fs, config).await,
            ParsedCommand::Simple { program, args } => {
                // Handle apt/sudo/dpkg commands
                match program.as_str() {
                    "apt" | "apt-get" => pkg.handle_apt_command(args).await,
                    "dpkg" => pkg.handle_dpkg_command(args).await,
                    "sudo" => {
                        if args.is_empty() {
                            return Ok(("".into(), "sudo: no command specified".into(), 1));
                        }
                        // Re-parse without sudo prefix
                        let remaining = args.join(" ");
                        let reparsed = parse_command(&remaining)?;
                        execute_parsed(&reparsed, fs, pkg, config).await
                    }
                    _ => {
                        // Complex multi-language parsing via Native LanguageRunner
                        let runner = LanguageRunner::new();
                        let lang_opt = match program.as_str() {
                            "python" | "python3" => Some(SupportedLanguage::Python),
                            "node" | "nodejs" => Some(SupportedLanguage::NodeJs),
                            "java" => Some(SupportedLanguage::Java),
                            "rustc" => Some(SupportedLanguage::Rust),
                            "go" => Some(SupportedLanguage::Go),
                            "gcc" => Some(SupportedLanguage::C),
                            "g++" => Some(SupportedLanguage::Cpp),
                            "ruby" => Some(SupportedLanguage::Ruby),
                            "php" => Some(SupportedLanguage::PHP),
                            "dart" => Some(SupportedLanguage::Dart),
                            "swift" => Some(SupportedLanguage::Swift),
                            "kotlinc" => Some(SupportedLanguage::Kotlin),
                            "lua" => Some(SupportedLanguage::Lua),
                            "perl" => Some(SupportedLanguage::Perl),
                            _ => None,
                        };

                        if let Some(lang) = lang_opt {
                            if args.is_empty() {
                                return Ok((format!("Flux {}: Native execution engine ready. Provide a file.", program), String::new(), 0));
                            }
                            // Execute natively without bash
                            match runner.execute_code(lang, &args[0]).await {
                                Ok(output) => return Ok((output, String::new(), 0)),
                                Err(e) => return Ok((String::new(), e.to_string(), 1)),
                            }
                        }

                        // Check if the program is installed
                        if fs.which(program).is_some() || is_system_command(program) {
                            Ok((format!("flux: executed {}", program), String::new(), 0))
                        } else {
                            Ok((String::new(), format!("bash: {}: command not found", program), 127))
                        }
                    }
                }
            }
            ParsedCommand::Pipeline(cmds) => {
                let mut last_output = String::new();
                for c in cmds {
                    let (out, err, code) = execute_parsed(c, fs, pkg, config).await?;
                    if code != 0 {
                        return Ok((last_output, err, code));
                    }
                    last_output = out;
                }
                Ok((last_output, String::new(), 0))
            }
            ParsedCommand::And(left, right) => {
                let (out1, err1, code1) = execute_parsed(left, fs, pkg, config).await?;
                if code1 != 0 {
                    return Ok((out1, err1, code1));
                }
                let (out2, err2, code2) = execute_parsed(right, fs, pkg, config).await?;
                Ok((format!("{}\n{}", out1, out2), err2, code2))
            }
            ParsedCommand::Or(left, right) => {
                let (out1, err1, code1) = execute_parsed(left, fs, pkg, config).await?;
                if code1 == 0 {
                    return Ok((out1, err1, code1));
                }
                execute_parsed(right, fs, pkg, config).await
            }
            ParsedCommand::Background(inner) => {
                tracing::info!("Background execution requested");
                let (out, _, _) = execute_parsed(inner, fs, pkg, config).await?;
                Ok((format!("[1] running in background\n{}", out), String::new(), 0))
            }
            ParsedCommand::Redirect { cmd: inner, target, append: _ } => {
                let (out, err, code) = execute_parsed(inner, fs, pkg, config).await?;
                fs.write_file(target, out.as_bytes())?;
                Ok((String::new(), err, code))
            }
        }
    }.boxed()
}

async fn execute_builtin(
    cmd: &BuiltinCommand,
    fs: &mut VirtualFilesystem,
    _config: &FluxConfig,
) -> FluxResult<(String, String, i32)> {
    match cmd {
        BuiltinCommand::Pwd => Ok((fs.cwd().to_string(), String::new(), 0)),
        BuiltinCommand::Cd(path) => {
            let target = if path == "~" { "/home/flux" } else { path.as_str() };
            fs.change_dir(target)?;
            Ok((String::new(), String::new(), 0))
        }
        BuiltinCommand::Echo(args) => Ok((args.join(" "), String::new(), 0)),
        BuiltinCommand::Clear => Ok(("\x1b[2J\x1b[H".into(), String::new(), 0)),
        BuiltinCommand::Whoami => Ok(("flux".into(), String::new(), 0)),
        BuiltinCommand::Hostname => Ok(("flux".into(), String::new(), 0)),
        BuiltinCommand::Uname(args) => {
            let flag = args.first().map(|s| s.as_str()).unwrap_or("-s");
            let out = match flag {
                "-a" => "Linux flux 6.1.0-flux #1 SMP PREEMPT aarch64 GNU/Linux",
                "-r" => "6.1.0-flux",
                "-m" => "aarch64",
                "-n" => "flux",
                _ => "Linux",
            };
            Ok((out.into(), String::new(), 0))
        }
        BuiltinCommand::Ls(args) => {
            let path = args.last().map(|s| s.as_str()).unwrap_or(".");
            let entries = fs.list_dir(path)?;
            let show_all = args.iter().any(|a| a.contains('a'));
            let show_long = args.iter().any(|a| a.contains('l'));
            let mut output = String::new();
            for entry in &entries {
                if !show_all && entry.name.starts_with('.') { continue; }
                if show_long {
                    output.push_str(&format!(
                        "{} {:>4} flux flux {:>8} {} {}\n",
                        entry.permissions, entry.links, entry.size, entry.modified, entry.name
                    ));
                } else {
                    output.push_str(&entry.name);
                    output.push_str("  ");
                }
            }
            if !show_long && !output.is_empty() { output.push('\n'); }
            Ok((output, String::new(), 0))
        }
        BuiltinCommand::Cat(files) => {
            let mut output = String::new();
            for file in files {
                match fs.read_file(file) {
                    Ok(content) => output.push_str(&String::from_utf8_lossy(&content)),
                    Err(e) => return Ok((String::new(), format!("cat: {}: {}", file, e), 1)),
                }
            }
            Ok((output, String::new(), 0))
        }
        BuiltinCommand::Mkdir(args) => {
            for dir in args {
                if dir.starts_with('-') { continue; }
                fs.create_dir(dir)?;
            }
            Ok((String::new(), String::new(), 0))
        }
        BuiltinCommand::Touch(files) => {
            for file in files {
                fs.create_file(file)?;
            }
            Ok((String::new(), String::new(), 0))
        }
        BuiltinCommand::Rm(args) => {
            for path in args {
                if path.starts_with('-') { continue; }
                fs.remove(path)?;
            }
            Ok((String::new(), String::new(), 0))
        }
        BuiltinCommand::Cp(src, dst) => {
            fs.copy(src, dst)?;
            Ok((String::new(), String::new(), 0))
        }
        BuiltinCommand::Mv(src, dst) => {
            fs.rename(src, dst)?;
            Ok((String::new(), String::new(), 0))
        }
        BuiltinCommand::Which(name) => {
            match fs.which(name) {
                Some(path) => Ok((path, String::new(), 0)),
                None => Ok((String::new(), format!("which: no {} in PATH", name), 1)),
            }
        }
        BuiltinCommand::Help => {
            let help = format!(
                "Flux AI Terminal v{}\nCreated by {}\n\n\
                 Built-in commands:\n  \
                 cd, pwd, ls, cat, echo, mkdir, rm, cp, mv, touch,\n  \
                 chmod, grep, head, tail, wc, which, whoami, hostname,\n  \
                 uname, env, set, export, unset, alias, unalias,\n  \
                 source, history, clear, exit, help\n\n\
                 Package management:\n  \
                 apt update, apt upgrade, apt install <pkg>,\n  \
                 apt remove <pkg>, apt search <pkg>, dpkg -l\n\n\
                 AI assistant:\n  \
                 flux-ai chat, flux-ai code, flux-ai explain\n",
                crate::VERSION, crate::AUTHOR
            );
            Ok((help, String::new(), 0))
        }
        BuiltinCommand::History => Ok(("(history listing)".into(), String::new(), 0)),
        BuiltinCommand::Exit(code) => {
            Ok((format!("exit {}", code), String::new(), *code))
        }
        BuiltinCommand::Env | BuiltinCommand::Set => Ok(("(env vars)".into(), String::new(), 0)),
        _ => Ok((String::new(), String::new(), 0)),
    }
}

fn is_system_command(name: &str) -> bool {
    matches!(name,
        "python3" | "python" | "pip" | "pip3" | "node" | "npm" | "npx" |
        "rustc" | "cargo" | "git" | "ssh" | "scp" | "curl" | "wget" |
        "gcc" | "g++" | "clang" | "make" | "cmake" | "vim" | "nvim" |
        "nano" | "tmux" | "screen" | "htop" | "top" | "ps" | "kill" |
        "tar" | "gzip" | "gunzip" | "zip" | "unzip" | "find" | "xargs" |
        "awk" | "sed" | "sort" | "cut" | "tr" | "tee" | "diff" | "patch" |
        "sqlite3" | "php" | "go" | "java" | "javac" | "ruby" | "perl" |
        "man" | "info" | "less" | "more" | "date" | "cal" | "df" | "du" |
        "free" | "uptime" | "id" | "groups" | "w" | "last" | "dmesg"
    )
}
