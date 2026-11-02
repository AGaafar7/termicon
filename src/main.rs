mod model;
mod icons;
mod lookup;
mod meta;
mod tree;
mod init;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub struct Config {
    pub use_unicode: bool,
    pub long_listing: bool,
    pub show_all: bool,
    pub tree_mode: bool,
    pub max_depth: usize,
}

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();

    // 1. Fast path: Shell hook generation (must run in < 1ms)
    if let [cmd, shell] = raw_args.as_slice() {
        if cmd == "init" {
            init::print_init(shell);
            return;
        }
    }

    let mut use_unicode = env::var("TERMICON_MODE")
        .map(|v| v.to_lowercase() == "unicode" || v.to_lowercase() == "emoji")
        .unwrap_or(false);

    let mut long_listing = false;
    let mut show_all = false;
    let mut tree_mode = false;
    let mut max_depth = 3;
    let mut targets: Vec<String> = Vec::new();
    let mut command: Option<String> = None;

    let mut iter = raw_args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                println!("termicon v{}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "--emoji" | "--unicode" => {
                use_unicode = true;
            }
            "-l" => long_listing = true,
            "-a" | "--all" => show_all = true,
            "-la" | "-al" => {
                long_listing = true;
                show_all = true;
            }
            "-t" | "--tree" => tree_mode = true,
            "-L" | "--level" => {
                if let Some(level_str) = iter.next() {
                    if let Ok(lvl) = level_str.parse::<usize>() {
                        max_depth = lvl;
                    }
                }
            }
            "icon" => {
                command = Some("icon".to_string());
                if let Some(target) = iter.next() {
                    targets.push(target);
                }
            }
            arg if !arg.starts_with('-') => {
                targets.push(arg.to_string());
            }
            _ => {}
        }
    }

    let config = Config {
        use_unicode,
        long_listing,
        show_all,
        tree_mode,
        max_depth,
    };

    // 2. Pipe / STDIN Mode
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

    // 3. Query Mode: `termicon icon <target>`
    if command.as_deref() == Some("icon") {
        if let Some(target) = targets.first() {
            let path = Path::new(target);
            let is_dir = path.is_dir();
            let icon = lookup::resolve_icon(path, is_dir);
            print!("{}", icon.render(config.use_unicode));
        }
        return;
    }

    // 4. Tree Mode or Standard Directory Listing
    let run_targets = if targets.is_empty() {
        vec![".".to_string()]
    } else {
        targets
    };

    for target in &run_targets {
        let path = Path::new(target);
        if config.tree_mode {
            tree::print_tree(path, &config, config.max_depth);
        } else if path.is_dir() {
            if run_targets.len() > 1 {
                println!("\n{}:", target);
            }
            list_directory(path, &config);
        } else {
            let icon = lookup::resolve_icon(path, false);
            println!("{}  {}", icon.render(config.use_unicode), target);
        }
    }
}

fn list_directory(dir: &Path, config: &Config) {
    let mut entries = match fs::read_dir(dir) {
        Ok(read_dir) => read_dir
            .filter_map(|e| e.ok())
            .filter(|e| {
                if config.show_all {
                    true
                } else {
                    !e.file_name().to_string_lossy().starts_with('.')
                }
            })
            .collect::<Vec<_>>(),
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

        if config.long_listing {
            if let Ok(metadata) = entry.metadata() {
                let meta_info = meta::format_metadata(&metadata);
                let _ = writeln!(
                    handle,
                    "{} {:>8} {:>8} {:>8}  {}  {}  {}",
                    meta_info.permissions,
                    meta_info.user,
                    meta_info.group,
                    meta_info.size,
                    meta_info.modified,
                    icon.render(config.use_unicode),
                    file_name
                );
                continue;
            }
        }

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
    termicon [FLAGS] [PATH]...
    termicon icon <PATH>
    termicon init <zsh|bash|fish>
    <command> | termicon

FLAGS:
    -l                          Use long listing format (permissions, size, date)
    -a, --all                   Do not ignore hidden entries starting with .
    -la, -al                    Combined long listing and hidden files
    -t, --tree                  Recurse into directories as a tree
    -L, --level <NUM>           Max display depth of the directory tree (default: 3)
    --emoji, --unicode          Use standard macOS emojis instead of Nerd Fonts
    -h, --help                  Print help information
    -v, --version               Print version information"
    );
}