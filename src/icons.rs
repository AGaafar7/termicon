use crate::model::IconDef;
use phf::phf_map;

// ANSI 256 Color Palette
pub const COLOR_RED: &str = "\x1b[38;5;196m";
pub const COLOR_ORANGE: &str = "\x1b[38;5;208m";
pub const COLOR_YELLOW: &str = "\x1b[38;5;220m";
pub const COLOR_GREEN: &str = "\x1b[38;5;40m";
pub const COLOR_BLUE: &str = "\x1b[38;5;39m";
pub const COLOR_PURPLE: &str = "\x1b[38;5;135m";
pub const COLOR_CYAN: &str = "\x1b[38;5;44m";
pub const COLOR_GRAY: &str = "\x1b[38;5;245m";
pub const COLOR_WHITE: &str = "\x1b[38;5;255m";
pub const COLOR_PINK: &str = "\x1b[38;5;211m";

// Default Fallbacks
pub const DEFAULT_FILE: IconDef = IconDef::new("\u{f15b}", "📄", COLOR_WHITE);
pub const DEFAULT_DIR: IconDef = IconDef::new("\u{f07b}", "📁", COLOR_BLUE);

/// Exact filename matches (checked first, case-insensitive)
pub static EXACT_MATCHES: phf::Map<&'static str, IconDef> = phf_map! {
    // Docker & Containers
    "dockerfile"             => IconDef::new("\u{f308}", "🐳", COLOR_BLUE),
    "docker-compose.yml"      => IconDef::new("\u{f308}", "🐳", COLOR_BLUE),
    "docker-compose.yaml"     => IconDef::new("\u{f308}", "🐳", COLOR_BLUE),
    ".dockerignore"          => IconDef::new("\u{f308}", "🐳", COLOR_GRAY),

    // Git
    ".gitignore"             => IconDef::new("\u{f1d3}", "🙈", COLOR_ORANGE),
    ".gitmodules"            => IconDef::new("\u{f1d3}", "🐙", COLOR_ORANGE),
    ".gitattributes"         => IconDef::new("\u{f1d3}", "🐙", COLOR_GRAY),

    // Rust
    "cargo.toml"             => IconDef::new("\u{e7a8}", "🦀", COLOR_ORANGE),
    "cargo.lock"             => IconDef::new("\u{e7a8}", "🔒", COLOR_GRAY),

    // Node / JS / Web Frameworks
    "package.json"           => IconDef::new("\u{e718}", "📦", COLOR_GREEN),
    "package-lock.json"      => IconDef::new("\u{e718}", "🔒", COLOR_GRAY),
    "pnpm-lock.yaml"         => IconDef::new("\u{e718}", "🔒", COLOR_YELLOW),
    "yarn.lock"              => IconDef::new("\u{e6a7}", "🔒", COLOR_BLUE),
    "bun.lockb"              => IconDef::new("\u{e718}", "🥟", COLOR_YELLOW),
    "tsconfig.json"          => IconDef::new("\u{e628}", "🔷", COLOR_BLUE),
    "vite.config.js"         => IconDef::new("\u{e6ac}", "⚡", COLOR_PURPLE),
    "vite.config.ts"         => IconDef::new("\u{e6ac}", "⚡", COLOR_PURPLE),
    "next.config.js"         => IconDef::new("\u{e76e}", "▲ ", COLOR_WHITE),
    "next.config.mjs"        => IconDef::new("\u{e76e}", "▲ ", COLOR_WHITE),
    "tailwind.config.js"     => IconDef::new("\u{e6bb}", "🎨", COLOR_CYAN),
    "tailwind.config.ts"     => IconDef::new("\u{e6bb}", "🎨", COLOR_CYAN),

    // Cloud & DevOps
    "terraform.tf"           => IconDef::new("\u{e69a}", "☁️ ", COLOR_PURPLE),
    ".terraform.lock.hcl"    => IconDef::new("\u{e69a}", "🔒", COLOR_PURPLE),
    "procfile"               => IconDef::new("\u{e607}", "⚙️ ", COLOR_PURPLE),
    "jenkinsfile"            => IconDef::new("\u{e767}", "👨‍", COLOR_RED),

    // Licenses & Build Files
    "license"                => IconDef::new("\u{f2c2}", "📜", COLOR_YELLOW),
    "license.md"             => IconDef::new("\u{f2c2}", "📜", COLOR_YELLOW),
    "makefile"               => IconDef::new("\u{e673}", "🛠️ ", COLOR_PURPLE),
    "cmakelists.txt"         => IconDef::new("\u{e673}", "🛠️ ", COLOR_GREEN),
    "gemfile"                => IconDef::new("\u{e21e}", "💎", COLOR_RED),
    ".env"                   => IconDef::new("\u{f462}", "🔑", COLOR_YELLOW),
    ".env.example"           => IconDef::new("\u{f462}", "🔑", COLOR_GRAY),
};

/// File extension matches (checked second, lowercase)
pub static EXTENSION_MATCHES: phf::Map<&'static str, IconDef> = phf_map! {
    // Systems & Native Languages
    "rs"         => IconDef::new("\u{e7a8}", "🦀", COLOR_ORANGE), // Rust
    "go"         => IconDef::new("\u{e627}", "🐹", COLOR_CYAN),   // Go
    "c"          => IconDef::new("\u{e61e}", "🅒 ", COLOR_BLUE),   // C
    "h"          => IconDef::new("\u{e61e}", "🅗 ", COLOR_PURPLE), // C Header
    "cpp"        => IconDef::new("\u{e61d}", "🅒 ", COLOR_BLUE),   // C++
    "hpp"        => IconDef::new("\u{e61d}", "🅗 ", COLOR_PURPLE),
    "swift"      => IconDef::new("\u{e755}", "🐦", COLOR_ORANGE), // Swift
    "zig"        => IconDef::new("\u{e6a9}", "⚡", COLOR_YELLOW), // Zig
    "nim"        => IconDef::new("\u{e677}", "👑", COLOR_YELLOW), // Nim

    // JVM & Functional Languages
    "java"       => IconDef::new("\u{e738}", "☕", COLOR_RED),    // Java
    "kt"         => IconDef::new("\u{e634}", "🅺 ", COLOR_PURPLE), // Kotlin
    "kts"        => IconDef::new("\u{e634}", "🅺 ", COLOR_PURPLE),
    "scala"      => IconDef::new("\u{e737}", "🔴", COLOR_RED),    // Scala
    "clj"        => IconDef::new("\u{e768}", "☯ ", COLOR_GREEN),  // Clojure
    "ex"         => IconDef::new("\u{e62d}", "💧", COLOR_PURPLE), // Elixir
    "exs"        => IconDef::new("\u{e62d}", "💧", COLOR_PURPLE),
    "erl"        => IconDef::new("\u{e7b1}", "🔴", COLOR_RED),    // Erlang
    "hs"         => IconDef::new("\u{e777}", "λ ", COLOR_PURPLE), // Haskell
    "ml"         => IconDef::new("\u{e67a}", "🐪", COLOR_ORANGE), // OCaml

    // Web & Scripting
    "js"         => IconDef::new("\u{e74e}", "🟨", COLOR_YELLOW), // JS
    "mjs"        => IconDef::new("\u{e74e}", "🟨", COLOR_YELLOW),
    "ts"         => IconDef::new("\u{e628}", "🔷", COLOR_BLUE),   // TS
    "jsx"        => IconDef::new("\u{e7ba}", "⚛️ ", COLOR_CYAN),   // React JSX
    "tsx"        => IconDef::new("\u{e7ba}", "⚛️ ", COLOR_CYAN),   // React TSX
    "vue"        => IconDef::new("\u{fd42}", "💚", COLOR_GREEN),  // Vue
    "svelte"     => IconDef::new("\u{e697}", "🔥", COLOR_ORANGE), // Svelte
    "astro"      => IconDef::new("\u{e6b3}", "🚀", COLOR_PURPLE), // Astro
    "html"       => IconDef::new("\u{e736}", "🌐", COLOR_ORANGE), // HTML
    "css"        => IconDef::new("\u{e749}", "🎨", COLOR_BLUE),   // CSS
    "scss"       => IconDef::new("\u{e74b}", "🎨", COLOR_PINK),   // SCSS
    "sass"       => IconDef::new("\u{e74b}", "🎨", COLOR_PINK),
    "py"         => IconDef::new("\u{e73c}", "🐍", COLOR_YELLOW), // Python
    "rb"         => IconDef::new("\u{e21e}", "💎", COLOR_RED),    // Ruby
    "php"        => IconDef::new("\u{e73d}", "🐘", COLOR_PURPLE), // PHP
    "lua"        => IconDef::new("\u{e620}", "🌙", COLOR_BLUE),   // Lua
    "dart"       => IconDef::new("\u{e798}", "🎯", COLOR_CYAN),   // Dart

    // Shell & Terminals
    "sh"         => IconDef::new("\u{e795}", "🐚", COLOR_GREEN),
    "zsh"        => IconDef::new("\u{e795}", "🐚", COLOR_GREEN),
    "bash"       => IconDef::new("\u{e795}", "🐚", COLOR_GREEN),
    "fish"       => IconDef::new("\u{e795}", "🐟", COLOR_CYAN),

    // Cloud, Infra & Config
    "tf"         => IconDef::new("\u{e69a}", "☁️ ", COLOR_PURPLE), // Terraform
    "tfvars"     => IconDef::new("\u{e69a}", "☁️ ", COLOR_PURPLE),
    "toml"       => IconDef::new("\u{e6b2}", "⚙️ ", COLOR_GRAY),
    "yaml"       => IconDef::new("\u{e601}", "📝", COLOR_PURPLE),
    "yml"        => IconDef::new("\u{e601}", "📝", COLOR_PURPLE),
    "json"       => IconDef::new("\u{e60b}", "📋", COLOR_YELLOW),
    "jsonc"      => IconDef::new("\u{e60b}", "📋", COLOR_YELLOW),
    "xml"        => IconDef::new("\u{e796}", "📰", COLOR_ORANGE),
    "ini"        => IconDef::new("\u{e615}", "⚙️ ", COLOR_GRAY),

    // Databases & Query Languages
    "sql"        => IconDef::new("\u{e706}", "🗄️ ", COLOR_YELLOW),
    "prisma"     => IconDef::new("\u{e684}", "💎", COLOR_CYAN),
    "graphql"    => IconDef::new("\u{e662}", "🕸️ ", COLOR_PINK),
    "gql"        => IconDef::new("\u{e662}", "🕸️ ", COLOR_PINK),
    "sqlite"     => IconDef::new("\u{e7c4}", "🗄️ ", COLOR_BLUE),

    // Documents & Media
    "md"         => IconDef::new("\u{e73e}", "📘", COLOR_BLUE),
    "pdf"        => IconDef::new("\u{f1c1}", "📕", COLOR_RED),
    "png"        => IconDef::new("\u{f1c5}", "🖼️ ", COLOR_PURPLE),
    "jpg"        => IconDef::new("\u{f1c5}", "🖼️ ", COLOR_PURPLE),
    "jpeg"       => IconDef::new("\u{f1c5}", "🖼️ ", COLOR_PURPLE),
    "svg"        => IconDef::new("\u{f1c5}", "📐", COLOR_ORANGE),
    "webp"       => IconDef::new("\u{f1c5}", "🖼️ ", COLOR_PURPLE),
    "gif"        => IconDef::new("\u{f1c5}", "🎞️ ", COLOR_GREEN),
    "mp4"        => IconDef::new("\u{f03d}", "🎬", COLOR_BLUE),
    "mov"        => IconDef::new("\u{f03d}", "🎬", COLOR_BLUE),
    "mp3"        => IconDef::new("\u{f001}", "🎵", COLOR_CYAN),
    "wav"        => IconDef::new("\u{f001}", "🎵", COLOR_CYAN),

    // Archives
    "zip"        => IconDef::new("\u{f1c6}", "🗜️ ", COLOR_YELLOW),
    "tar"        => IconDef::new("\u{f1c6}", "🗜️ ", COLOR_YELLOW),
    "gz"         => IconDef::new("\u{f1c6}", "🗜️ ", COLOR_YELLOW),
    "tar.gz"     => IconDef::new("\u{f1c6}", "🗜️ ", COLOR_YELLOW),
    "7z"         => IconDef::new("\u{f1c6}", "🗜️ ", COLOR_YELLOW),
};

/// Directory matches by folder name
pub static DIRECTORY_MATCHES: phf::Map<&'static str, IconDef> = phf_map! {
    ".git"          => IconDef::new("\u{e702}", "🐙", COLOR_ORANGE),
    ".github"       => IconDef::new("\u{f408}", "🐙", COLOR_WHITE),
    ".vscode"       => IconDef::new("\u{e70c}", "💻", COLOR_BLUE),
    "node_modules"  => IconDef::new("\u{e718}", "📦", COLOR_GREEN),
    "src"           => IconDef::new("\u{f121}", "📂", COLOR_BLUE),
    "target"        => IconDef::new("\u{f013}", "🎯", COLOR_GRAY),
    "dist"          => IconDef::new("\u{f013}", "📦", COLOR_GRAY),
    "build"         => IconDef::new("\u{f013}", "🏗️ ", COLOR_GRAY),
    "tests"         => IconDef::new("\u{f0c3}", "🧪", COLOR_GREEN),
    "test"          => IconDef::new("\u{f0c3}", "🧪", COLOR_GREEN),
    "docs"          => IconDef::new("\u{f02d}", "📚", COLOR_YELLOW),
    "doc"           => IconDef::new("\u{f02d}", "📚", COLOR_YELLOW),
    "assets"        => IconDef::new("\u{f03e}", "🎨", COLOR_PURPLE),
    "public"        => IconDef::new("\u{f0ac}", "🌐", COLOR_CYAN),
    "bin"           => IconDef::new("\u{e6b1}", "⚙️ ", COLOR_RED),
};