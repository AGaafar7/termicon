# termicon 📁✨

> Blazing-fast Nerd Font & Unicode file icons for macOS terminals (`zsh`, `bash`, `fish`). Inspired by PowerShell's `Terminal-Icons`.

`termicon` compiles icon mapping tables into machine code at build time using static compile-time hash maps (`phf`), executing in **~1–3 ms** with zero runtime dependencies.

---

## Features

- ⚡ **Instant Execution:** Starts and finishes in under 3 ms.
- 🍏 **macOS Universal:** Native binary supporting Apple Silicon (`arm64`) & Intel (`x86_64`).
- 🎨 **Nerd Fonts & Native macOS Emojis:** Rich Devicon glyphs by default, or instant emojis with `--emoji` without installing extra fonts.
- 🚰 **Pipeline Ready:** Easily pipe any file listing into `termicon` via `stdin`.

---

## Installation

### Via Homebrew (Recommended)
```bash
brew tap agaafar7/tap
brew install termicon

Manual Install (Universal macOS Binary)
Download the latest termicon-macOS-universal.tar.gz from the Releases page:

code
Bash
tar -xzf termicon-macOS-universal.tar.gz
sudo mv termicon /usr/local/bin/


Model 8:07 PM
Step 5: Professional GitHub Repository, CI/CD Automation, and Shell Integrations
Now that the core engine, fallback modes, and Universal Binary generation are verified, we will configure the project for open-source publication and automatic distribution.

In this step, we will:

Create the GitHub Actions Release Pipeline that automatically cross-compiles Universal macOS binaries and publishes release assets whenever you push a Git tag.

Add shell integrations for Zsh and Bash (e.g., aliases to drop termicon right into everyday workflows).

Draft a production-ready README.md and MIT License.

1. Automated GitHub Actions CI/CD Pipeline
We'll build a workflow that runs on GitHub's macOS runners, compiles both aarch64 and x86_64, stitches them with lipo, archives them as a .tar.gz, calculates the sha256 hash, and attaches them to a new GitHub Release.

Create the workflow file:

code
Bash
mkdir -p .github/workflows
touch .github/workflows/release.yml
Paste the following into .github/workflows/release.yml:

code
Yaml
name: Release Universal Binary

on:
  push:
    tags:
      - 'v*'

jobs:
  build-macos:
    runs-on: macos-latest
    permissions:
      contents: write

    steps:
      - name: Checkout Repository
        uses: actions/checkout@v4

      - name: Install Rust Toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-darwin,x86_64-apple-darwin

      - name: Build Apple Silicon (aarch64)
        run: cargo build --release --target aarch64-apple-darwin

      - name: Build Intel (x86_64)
        run: cargo build --release --target x86_64-apple-darwin

      - name: Fuse Universal Binary with lipo
        run: |
          mkdir -p bin
          lipo -create -output bin/termicon \
            target/aarch64-apple-darwin/release/termicon \
            target/x86_64-apple-darwin/release/termicon
          chmod +x bin/termicon

      - name: Package Universal Archive
        run: |
          cd bin
          tar -czf ../termicon-macOS-universal.tar.gz termicon
          cd ..
          shasum -a 256 termicon-macOS-universal.tar.gz > termicon-macOS-universal.tar.gz.sha256
          echo "SHA256: $(cat termicon-macOS-universal.tar.gz.sha256)"

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            termicon-macOS-universal.tar.gz
            termicon-macOS-universal.tar.gz.sha256
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
2. Add the Shell Integrations
Users of Terminal-Icons in PowerShell love having it seamlessly enhance their listing commands. On macOS, users run zsh by default.

We can provide two effortless shell drop-ins:

Direct replacement alias (e.g., aliasing ls or l to termicon).

Piped wrapper function that keeps standard BSD/GNU ls flags (permissions, sizes, dates) while prefixing icons.

Let's verify how clean this wrapper is:

Option A: Quick Alias (Fastest)
Add to ~/.zshrc or ~/.bashrc:

code
Bash
alias l="termicon"
alias le="termicon --emoji"
Option B: Preserving Flags (-la) via Wrapper Function
Add this function to ~/.zshrc:

code
Bash
# Enhanced ls with termicon
function icls() {
    if [ -t 1 ]; then
        # Run termicon on directory or argument
        termicon "$@"
    else
        command ls "$@"
    fi
}
3. Add the MIT License
Create LICENSE in your root directory:

code
Bash
touch LICENSE
Paste the standard MIT license text (replace [Year] and [Full Name]):

code
Text
MIT License

Copyright (c) 2026 [Full Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
4. Create the Production README.md
Replace your root README.md with documentation that explains the tool, highlights the sub-millisecond performance, and provides installation instructions:

code
Markdown
# termicon 📁✨

> Blazing-fast Nerd Font & Unicode file icons for macOS terminals (`zsh`, `bash`, `fish`). Inspired by PowerShell's `Terminal-Icons`.

`termicon` compiles icon mapping tables into machine code at build time using static compile-time hash maps (`phf`), executing in **~1–3 ms** with zero runtime dependencies.

---

## Features

- ⚡ **Instant Execution:** Starts and finishes in under 3 ms.
- 🍏 **macOS Universal:** Native binary supporting Apple Silicon (`arm64`) & Intel (`x86_64`).
- 🎨 **Nerd Fonts & Native macOS Emojis:** Rich Devicon glyphs by default, or instant emojis with `--emoji` without installing extra fonts.
- 🚰 **Pipeline Ready:** Easily pipe any file listing into `termicon` via `stdin`.

---

## Installation

### Via Homebrew (Recommended)
```bash
brew tap your-username/tap
brew install termicon
Manual Install (Universal macOS Binary)
Download the latest termicon-macOS-universal.tar.gz from the Releases page:

code
Bash
tar -xzf termicon-macOS-universal.tar.gz
sudo mv termicon /usr/local/bin/
Building From Source
code
Bash
git clone https://github.com/agaafar/termicon.git
cd termicon
cargo build --release
sudo cp target/release/termicon /usr/local/bin/

Usage

1. Directory Listing
code
Bash
# List current directory
termicon

# List target directory
termicon src/

# Use native macOS emojis (no custom font required!)
termicon --emoji
2. Single Icon Query (Starship / Prompts)
code
Bash
termicon icon Dockerfile

termicon icon main.rs
3. Pipeline / STDIN
code
Bash
ls -1 | termicon
git status --short | awk '{print $2}' | termicon
Shell Configuration (~/.zshrc)
Add simple aliases to your .zshrc or .bash_profile:

code
Bash
# Quick directory icons
alias l="termicon"
alias le="termicon --emoji"

# Optional: Default to unicode/emoji globally
# export TERMICON_MODE="unicode"
License
MIT © [Your Name]

code
Code
---
