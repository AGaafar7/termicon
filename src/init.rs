pub fn print_init(shell: &str) {
    match shell {
        "zsh" => print!("{}", ZSH_SCRIPT),
        "bash" => print!("{}", BASH_SCRIPT),
        "fish" => print!("{}", FISH_SCRIPT),
        _ => {
            eprintln!(
                "termicon: unsupported shell '{}'. Supported shells: zsh, bash, fish",
                shell
            );
            std::process::exit(1);
        }
    }
}

const ZSH_SCRIPT: &str = r#"
# --- termicon zsh initialization ---

# Core shortcuts
alias l="termicon"
alias ll="termicon -l"
alias la="termicon -la"
alias lt="termicon -t"

# Interactive wrapper for ls (only formats when printing to a terminal)
function ls() {
    if [ -t 1 ]; then
        termicon "$@"
    else
        command ls "$@"
    fi
}
"#;

const BASH_SCRIPT: &str = r#"
# --- termicon bash initialization ---

# Core shortcuts
alias l="termicon"
alias ll="termicon -l"
alias la="termicon -la"
alias lt="termicon -t"

# Interactive wrapper for ls
function ls() {
    if [ -t 1 ]; then
        termicon "$@"
    else
        command ls "$@"
    fi
}
"#;

const FISH_SCRIPT: &str = r#"
# --- termicon fish initialization ---

# Core shortcuts
alias l="termicon"
alias ll="termicon -l"
alias la="termicon -la"
alias lt="termicon -t"

# Interactive wrapper for ls
function ls --wraps ls
    if isatty stdout
        termicon $argv
    else
        command ls $argv
    end
end
"#;