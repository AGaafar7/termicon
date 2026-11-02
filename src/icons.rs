use crate::model::IconDef;
use phf::phf_map;

pub const COLOR_RED: &str = "\x1b[38;5;196m";
pub const COLOR_ORANGE: &str = "\x1b[38;5;208m";
pub const COLOR_YELLOW: &str = "\x1b[38;5;220m";
pub const COLOR_GREEN: &str = "\x1b[38;5;40m";
pub const COLOR_BLUE: &str = "\x1b[38;5;39m";
pub const COLOR_PURPLE: &str = "\x1b[38;5;135m";
pub const COLOR_CYAN: &str = "\x1b[38;5;44m";
pub const COLOR_GRAY: &str = "\x1b[38;5;245m";
pub const COLOR_WHITE: &str = "\x1b[38;5;255m";

// Default Fallbacks
pub const DEFAULT_FILE: IconDef = IconDef::new("\u{f15b}", "📄", COLOR_WHITE);
pub const DEFAULT_DIR: IconDef = IconDef::new("\u{f07b}", "📁", COLOR_BLUE);

/// Exact filename matches
pub static EXACT_MATCHES: phf::Map<&'static str, IconDef> = phf_map! {
    "dockerfile"         => IconDef::new("\u{f308}", "🐳", COLOR_BLUE),
    "docker-compose.yml"  => IconDef::new("\u{f308}", "🐳", COLOR_BLUE),
    "cargo.toml"         => IconDef::new("\u{e7a8}", "🦀", COLOR_ORANGE),
    "cargo.lock"         => IconDef::new("\u{e7a8}", "🔒", COLOR_GRAY),
    "package.json"       => IconDef::new("\u{e718}", "📦", COLOR_GREEN),
    "package-lock.json"  => IconDef::new("\u{e718}", "🔒", COLOR_GRAY),
    ".gitignore"         => IconDef::new("\u{f1d3}", "🙈", COLOR_ORANGE),
    ".gitmodules"        => IconDef::new("\u{f1d3}", "🐙", COLOR_ORANGE),
    "license"            => IconDef::new("\u{f2c2}", "📜", COLOR_YELLOW),
    "license.md"         => IconDef::new("\u{f2c2}", "📜", COLOR_YELLOW),
    "makefile"           => IconDef::new("\u{e673}", "🛠️ ", COLOR_PURPLE),
    ".env"               => IconDef::new("\u{f462}", "🔑", COLOR_YELLOW),
};

/// File extension matches
pub static EXTENSION_MATCHES: phf::Map<&'static str, IconDef> = phf_map! {
    // Systems & Native
    "rs"     => IconDef::new("\u{e7a8}", "🦀", COLOR_ORANGE),
    "go"     => IconDef::new("\u{e627}", "🐹", COLOR_CYAN),
    "c"      => IconDef::new("\u{e61e}", "🅒 ", COLOR_BLUE),
    "cpp"    => IconDef::new("\u{e61d}", "🅒 ", COLOR_BLUE),
    "swift"  => IconDef::new("\u{e755}", "🐦", COLOR_ORANGE),
    
    // Web & Scripting
    "js"     => IconDef::new("\u{e74e}", "🟨", COLOR_YELLOW),
    "ts"     => IconDef::new("\u{e628}", "🔷", COLOR_BLUE),
    "py"     => IconDef::new("\u{e73c}", "🐍", COLOR_YELLOW),
    "sh"     => IconDef::new("\u{e795}", "🐚", COLOR_GREEN),
    "zsh"    => IconDef::new("\u{e795}", "🐚", COLOR_GREEN),
    "toml"   => IconDef::new("\u{e6b2}", "⚙️ ", COLOR_GRAY),
    "yaml"   => IconDef::new("\u{e601}", "📝", COLOR_PURPLE),
    "yml"    => IconDef::new("\u{e601}", "📝", COLOR_PURPLE),
    "json"   => IconDef::new("\u{e60b}", "📋", COLOR_YELLOW),
    "md"     => IconDef::new("\u{e73e}", "📘", COLOR_BLUE),
    "png"    => IconDef::new("\u{f1c5}", "🖼️ ", COLOR_PURPLE),
    "jpg"    => IconDef::new("\u{f1c5}", "🖼️ ", COLOR_PURPLE),
    "zip"    => IconDef::new("\u{f1c6}", "🗜️ ", COLOR_YELLOW),
    "tar.gz" => IconDef::new("\u{f1c6}", "🗜️ ", COLOR_YELLOW),
};

/// Directory matches
pub static DIRECTORY_MATCHES: phf::Map<&'static str, IconDef> = phf_map! {
    ".git"          => IconDef::new("\u{e702}", "🐙", COLOR_ORANGE),
    "node_modules"  => IconDef::new("\u{e718}", "📦", COLOR_GREEN),
    ".github"       => IconDef::new("\u{f408}", "🐙", COLOR_WHITE),
    "src"           => IconDef::new("\u{f121}", "📂", COLOR_BLUE),
    "target"        => IconDef::new("\u{f013}", "🎯", COLOR_GRAY),
};