use std::path::Path;
use crate::icons::*;
use crate::model::IconDef;

/// Resolves the corresponding IconDef for a given path or name.
/// Evaluation precedence:
/// 1. Directory check (if path exists and is a dir, or matches known dir names)
/// 2. Exact filename match (case-insensitive, e.g. "Dockerfile", "Cargo.toml")
/// 3. File extension match (lowercase, e.g. "rs", "ts", "tar.gz")
/// 4. Fallback defaults
pub fn resolve_icon(path: &Path, is_directory: bool) -> IconDef {
    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return if is_directory { DEFAULT_DIR } else { DEFAULT_FILE },
    };

    let lower_name = file_name.to_ascii_lowercase();

    // 1. Directory Matching
    if is_directory {
        if let Some(icon) = DIRECTORY_MATCHES.get(lower_name.as_str()) {
            return *icon;
        }
        return DEFAULT_DIR;
    }

    // 2. Exact Filename Matching
    if let Some(icon) = EXACT_MATCHES.get(lower_name.as_str()) {
        return *icon;
    }

    // 3. Multi-part extension matching (e.g., .tar.gz)
    if let Some(ext) = get_extension(&lower_name) {
        if let Some(icon) = EXTENSION_MATCHES.get(ext) {
            return *icon;
        }
    }

    // 4. Default Fallback
    DEFAULT_FILE
}

/// Extracts the extension; handles compound extensions like `.tar.gz`
fn get_extension(lower_name: &str) -> Option<&str> {
    if lower_name.ends_with(".tar.gz") {
        return Some("tar.gz");
    }
    Path::new(lower_name)
        .extension()
        .and_then(|ext| ext.to_str())
}