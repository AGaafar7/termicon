use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::lookup;
use crate::Config;

pub fn print_tree(root: &Path, config: &Config, max_depth: usize) {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let is_dir = root.is_dir();
    let icon = lookup::resolve_icon(root, is_dir);
    let root_name = root
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_else(|| root.to_string_lossy());

    // Print root header
    let _ = writeln!(handle, "{}  {}", icon.render(config.use_unicode), root_name);

    if is_dir {
        let mut ancestors_last = Vec::new();
        walk_tree(root, &mut ancestors_last, config, 1, max_depth, &mut handle);
    }
}

fn walk_tree<W: Write>(
    dir: &Path,
    ancestors_last: &mut Vec<bool>,
    config: &Config,
    current_depth: usize,
    max_depth: usize,
    writer: &mut W,
) {
    if current_depth > max_depth {
        return;
    }

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
        Err(_) => return,
    };

    entries.sort_by_key(|e| e.file_name());
    let count = entries.len();

    for (index, entry) in entries.into_iter().enumerate() {
        let is_last = index == count - 1;
        let file_type = entry.file_type();
        let is_dir = file_type.map(|t| t.is_dir()).unwrap_or(false);
        let path = entry.path();
        let icon = lookup::resolve_icon(&path, is_dir);
        let file_name = entry.file_name().to_string_lossy().to_string();

        // Build the tree indentation guides
        let mut prefix = String::new();
        for &is_parent_last in ancestors_last.iter() {
            if is_parent_last {
                prefix.push_str("    ");
            } else {
                prefix.push_str("│   ");
            }
        }

        let branch = if is_last { "└── " } else { "├── " };
        let _ = writeln!(
            writer,
            "{}{}{}  {}",
            prefix,
            branch,
            icon.render(config.use_unicode),
            file_name
        );

        if is_dir {
            ancestors_last.push(is_last);
            walk_tree(&path, ancestors_last, config, current_depth + 1, max_depth, writer);
            ancestors_last.pop();
        }
    }
}