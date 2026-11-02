mod model;
mod icons;
mod lookup;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

struct Config {
    use_unicode: bool,
}

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();

    // Check if user set the mode via environment variable or flag
    let mut use_unicode = env::var("TERMICON_MODE")
        .map(|v| v.to_lowercase() == "unicode" || v.to_lowercase() == "emoji")
        .unwrap_or(false);

    // Filter out our optional --emoji / --unicode flag
    let args: Vec<String> = raw_args
        .into_iter()
        .filter(|arg| {
            if arg == "--emoji" || arg == "--unicode" {
                use_unicode = true;
                false
            } else {
                true
            }
        })
        .collect();

    let config = Config { use_unicode };

    // 1. Pipe / STDIN Mode
    if atty_check_stdin() {
        let stdin = io::stdin();
        let mut stdout = io::BufWriter::new(io::stdout());
        
        for line in stdin.lock().lines().flatten() {
            let path = Path::new(&line);
            let is_dir = path.is_dir();
            let icon = lookup::resolve_icon(path, is_dir);
            let _ = writeln!(stdout, "{}  {}", icon.render(config.use_unicode), line);
        }
        return;
    }

    // 2. CLI Arguments
    match args.as_slice() {
        [] => list_directory(Path::new("."), &config),

        [cmd, target] if cmd == "icon" => {
            let path = Path::new(target);
            let is_dir = path.is_dir();
            let icon = lookup::resolve_icon(path, is_dir);
            print!("{}", icon.render(config.use_unicode));
        }

        [flag] if flag == "--help" || flag == "-h" => {
            print_help();
        }

        [flag] if flag == "--version" || flag == "-v" => {
            println!("termicon v{}", env!("CARGO_PKG_VERSION"));
        }

        [target] => {
            let path = Path::new(target);
            if path.is_dir() {
                list_directory(path, &config);
            } else {
                let icon = lookup::resolve_icon(path, false);
                println!("{}  {}", icon.render(config.use_unicode), target);
            }
        }

        _ => {
            print_help();
        }
    }
}

fn list_directory(dir: &Path, config: &Config) {
    let mut entries = match fs::read_dir(dir) {
        Ok(read_dir) => read_dir.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(err) => {
            eprintln!("termicon: {}: {}", dir.display(), err);
            return;
        }
    };

    entries.sort_by_key(|e| e.file_name());

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    for entry in entries {
        let file_type = entry.file_type();
        let is_dir = file_type.map(|t| t.is_dir()).unwrap_or(false);
        let path = entry.path();
        let icon = lookup::resolve_icon(&path, is_dir);
        let file_name = entry.file_name().to_string_lossy().to_string();
        
        let _ = writeln!(handle, "{}  {}", icon.render(config.use_unicode), file_name);
    }
}

fn atty_check_stdin() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) == 0 }
}

fn print_help() {
    println!(
        "termicon - High-speed file icon engine for macOS

USAGE:
    termicon [PATH]             List directory contents with icons
    termicon icon <PATH>        Print only the colored icon glyph
    <command> | termicon        Prefix lines received from STDIN with icons

OPTIONS:
    --emoji, --unicode          Use standard macOS emojis instead of Nerd Fonts
    -h, --help                  Print help information
    -v, --version               Print version information

ENV VARIABLES:
    TERMICON_MODE=unicode       Default to emoji/unicode icons permanently"
    );
}