use std::path::Path;
use std::process::Stdio;
use tokio::process::{Child, Command};

/// Spawns a new terminal window running codex with the provided arguments.
///
/// This function detects the current terminal emulator and spawns a new instance
/// of codex in a new terminal window. Supports Linux terminals only (Ghostty, VS Code, etc.).
///
/// # Arguments
/// * `codex_args` - Arguments to pass to the codex binary
/// * `cwd` - Optional working directory for the new terminal
///
/// # Returns
/// * `Ok(Child)` - Handle to the spawned process
/// * `Err(std::io::Error)` - If spawning failed
pub fn spawn_terminal_with_codex(
    codex_args: &[String],
    cwd: Option<&Path>,
) -> std::io::Result<Child> {
    // Get the path to the current codex binary
    let codex_binary = std::env::current_exe()?;

    // Detect terminal type
    let terminal_type = detect_terminal_type();

    // Build the command based on terminal type
    let mut cmd = build_terminal_command(&terminal_type, &codex_binary, codex_args)?;

    // Set working directory if provided
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    // Spawn detached so it doesn't block
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TerminalType {
    Ghostty,
    VSCode,
    Alacritty,
    Kitty,
    WezTerm,
    GnomeTerminal,
    Konsole,
    Xterm,
    Unknown,
}

/// Detect the terminal emulator we're running in
fn detect_terminal_type() -> TerminalType {
    // Check TERM_PROGRAM first (most reliable)
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        match term_program.to_lowercase().as_str() {
            "ghostty" => return TerminalType::Ghostty,
            "vscode" => return TerminalType::VSCode,
            "wezterm" => return TerminalType::WezTerm,
            _ => {}
        }
    }

    // Check for VS Code specific env vars
    if std::env::var("VSCODE_GIT_IPC_HANDLE").is_ok()
        || std::env::var("TERM_PROGRAM").as_deref() == Ok("vscode") {
        return TerminalType::VSCode;
    }

    // Check for Ghostty specific env vars
    if std::env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return TerminalType::Ghostty;
    }

    // Check for WezTerm
    if std::env::var("WEZTERM_EXECUTABLE").is_ok() {
        return TerminalType::WezTerm;
    }

    // Check for Kitty
    if std::env::var("KITTY_WINDOW_ID").is_ok() {
        return TerminalType::Kitty;
    }

    // Check for Alacritty
    if std::env::var("ALACRITTY_SOCKET").is_ok()
        || std::env::var("TERM").as_deref() == Ok("alacritty") {
        return TerminalType::Alacritty;
    }

    // Check for Konsole
    if std::env::var("KONSOLE_VERSION").is_ok() {
        return TerminalType::Konsole;
    }

    // Check for GNOME Terminal
    if std::env::var("GNOME_TERMINAL_SCREEN").is_ok() {
        return TerminalType::GnomeTerminal;
    }

    TerminalType::Unknown
}

/// Build the command to spawn a new terminal with codex
fn build_terminal_command(
    terminal_type: &TerminalType,
    codex_binary: &Path,
    codex_args: &[String],
) -> std::io::Result<Command> {
    match terminal_type {
        TerminalType::Ghostty => {
            // Ghostty doesn't support tab spawning via CLI yet
            // Spawn a new window instead
            let mut cmd = Command::new("ghostty");
            cmd.arg(codex_binary);
            cmd.args(codex_args);
            Ok(cmd)
        }
        TerminalType::VSCode => {
            // VS Code terminal - spawn external terminal
            // Try to use gnome-terminal, konsole, or xterm as fallback
            if which::which("gnome-terminal").is_ok() {
                let mut cmd = Command::new("gnome-terminal");
                cmd.arg("--");
                cmd.arg(codex_binary);
                cmd.args(codex_args);
                Ok(cmd)
            } else if which::which("konsole").is_ok() {
                let mut cmd = Command::new("konsole");
                cmd.arg("-e");
                cmd.arg(codex_binary);
                cmd.args(codex_args);
                Ok(cmd)
            } else if which::which("xterm").is_ok() {
                let mut cmd = Command::new("xterm");
                cmd.arg("-e");
                cmd.arg(codex_binary);
                cmd.args(codex_args);
                Ok(cmd)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No suitable terminal emulator found for VS Code environment",
                ))
            }
        }
        TerminalType::Alacritty => {
            let mut cmd = Command::new("alacritty");
            cmd.arg("-e");
            cmd.arg(codex_binary);
            cmd.args(codex_args);
            Ok(cmd)
        }
        TerminalType::Kitty => {
            let mut cmd = Command::new("kitty");
            cmd.arg(codex_binary);
            cmd.args(codex_args);
            Ok(cmd)
        }
        TerminalType::WezTerm => {
            let mut cmd = Command::new("wezterm");
            cmd.arg("start");
            cmd.arg("--");
            cmd.arg(codex_binary);
            cmd.args(codex_args);
            Ok(cmd)
        }
        TerminalType::GnomeTerminal => {
            let mut cmd = Command::new("gnome-terminal");
            cmd.arg("--");
            cmd.arg(codex_binary);
            cmd.args(codex_args);
            Ok(cmd)
        }
        TerminalType::Konsole => {
            let mut cmd = Command::new("konsole");
            cmd.arg("-e");
            cmd.arg(codex_binary);
            cmd.args(codex_args);
            Ok(cmd)
        }
        TerminalType::Xterm => {
            let mut cmd = Command::new("xterm");
            cmd.arg("-e");
            cmd.arg(codex_binary);
            cmd.args(codex_args);
            Ok(cmd)
        }
        TerminalType::Unknown => {
            // Try common terminals in order of preference
            if which::which("gnome-terminal").is_ok() {
                let mut cmd = Command::new("gnome-terminal");
                cmd.arg("--");
                cmd.arg(codex_binary);
                cmd.args(codex_args);
                Ok(cmd)
            } else if which::which("konsole").is_ok() {
                let mut cmd = Command::new("konsole");
                cmd.arg("-e");
                cmd.arg(codex_binary);
                cmd.args(codex_args);
                Ok(cmd)
            } else if which::which("xterm").is_ok() {
                let mut cmd = Command::new("xterm");
                cmd.arg("-e");
                cmd.arg(codex_binary);
                cmd.args(codex_args);
                Ok(cmd)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No suitable terminal emulator found",
                ))
            }
        }
    }
}
