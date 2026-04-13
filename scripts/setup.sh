#!/usr/bin/env bash
# =============================================================================
# Workshop Setup Script — Make Invalid Data Unrepresentable
# Supports: macOS, Linux (Ubuntu/Debian/Fedora/Arch), WSL
# =============================================================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
fail()  { echo -e "${RED}[FAIL]${NC} $1"; }

echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║  Workshop Setup: Make Invalid Data Unrepresentable       ║"
echo "║  Rust's Type System for Data Engineering                 ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

ERRORS=0

# ─── Detect OS ────────────────────────────────────────────────
detect_os() {
    case "$(uname -s)" in
        Linux*)   OS="linux" ;;
        Darwin*)  OS="macos" ;;
        MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
        *)        OS="unknown" ;;
    esac

    if [[ "$OS" == "linux" ]]; then
        if grep -qi "microsoft" /proc/version 2>/dev/null; then
            OS="wsl"
        fi
        if [ -f /etc/os-release ]; then
            DISTRO=$(grep ^ID= /etc/os-release | cut -d= -f2 | tr -d '"')
        else
            DISTRO="unknown"
        fi
    fi

    info "Detected OS: $OS $([ -n "${DISTRO:-}" ] && echo "($DISTRO)" || echo "")"
}

# ─── Check Rust ───────────────────────────────────────────────
check_rust() {
    info "Checking Rust installation..."

    if command -v rustc &>/dev/null; then
        RUST_VERSION=$(rustc --version | awk '{print $2}')
        ok "Rust $RUST_VERSION found"

        # Check minimum version (1.70+ required for edition 2021 features we use)
        MAJOR=$(echo "$RUST_VERSION" | cut -d. -f1)
        MINOR=$(echo "$RUST_VERSION" | cut -d. -f2)
        if [ "$MAJOR" -ge 1 ] && [ "$MINOR" -ge 70 ]; then
            ok "Rust version is sufficient (>= 1.70)"
        else
            warn "Rust version $RUST_VERSION is old. Updating..."
            rustup update stable
        fi
    else
        warn "Rust not found. Installing via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        source "$HOME/.cargo/env"

        if command -v rustc &>/dev/null; then
            ok "Rust $(rustc --version | awk '{print $2}') installed"
        else
            fail "Rust installation failed. Please install manually: https://rustup.rs"
            ERRORS=$((ERRORS + 1))
        fi
    fi
}

# ─── Check Cargo ──────────────────────────────────────────────
check_cargo() {
    info "Checking cargo..."

    if command -v cargo &>/dev/null; then
        ok "cargo $(cargo --version | awk '{print $2}') found"
    else
        fail "cargo not found. Is Rust installed correctly?"
        fail "Try: source \$HOME/.cargo/env"
        ERRORS=$((ERRORS + 1))
    fi
}

# ─── Check Git ────────────────────────────────────────────────
check_git() {
    info "Checking git..."

    if command -v git &>/dev/null; then
        ok "git $(git --version | awk '{print $3}') found"
    else
        warn "git not found. Installing..."
        case "$OS" in
            macos)
                if command -v brew &>/dev/null; then
                    brew install git
                else
                    fail "git not found. Install Xcode CLI tools: xcode-select --install"
                    ERRORS=$((ERRORS + 1))
                    return
                fi
                ;;
            linux|wsl)
                case "${DISTRO:-}" in
                    ubuntu|debian|pop)  apt-get update &&  apt-get install -y git ;;
                    fedora|rhel|centos)  dnf install -y git ;;
                    arch|manjaro)  pacman -S --noconfirm git ;;
                    *) fail "Please install git manually"; ERRORS=$((ERRORS + 1)); return ;;
                esac
                ;;
        esac
        ok "git installed"
    fi
}

# ─── Check Editor ─────────────────────────────────────────────
check_editor() {
    info "Checking for a code editor..."

    if command -v code &>/dev/null; then
        ok "VS Code found"
        info "Checking rust-analyzer extension..."
        if code --list-extensions 2>/dev/null | grep -q "rust-lang.rust-analyzer"; then
            ok "rust-analyzer extension installed"
        else
            warn "rust-analyzer not found. Installing..."
            code --install-extension rust-lang.rust-analyzer 2>/dev/null || true
            ok "rust-analyzer installed (restart VS Code to activate)"
        fi
    elif command -v vim &>/dev/null || command -v nvim &>/dev/null; then
        ok "vim/nvim found (ensure you have rust.vim or rust-analyzer LSP configured)"
    else
        warn "No known editor found. We recommend VS Code with rust-analyzer extension."
        warn "Download: https://code.visualstudio.com"
    fi
}

# ─── OS-Specific Dependencies ─────────────────────────────────
check_os_deps() {
    info "Checking OS-specific build dependencies..."

    case "$OS" in
        macos)
            if xcode-select -p &>/dev/null; then
                ok "Xcode CLI tools installed"
            else
                warn "Installing Xcode CLI tools (may show a dialog)..."
                xcode-select --install 2>/dev/null || true
                warn "If a dialog appeared, click Install and re-run this script after."
            fi
            ;;
        linux|wsl)
            # Check for essential build tools
            if command -v cc &>/dev/null; then
                ok "C compiler found (needed for some Rust crates)"
            else
                warn "Installing build essentials..."
                case "${DISTRO:-}" in
                    ubuntu|debian|pop)  apt-get update &&  apt-get install -y build-essential pkg-config libssl-dev ;;
                    fedora|rhel|centos)  dnf groupinstall -y "Development Tools" &&  dnf install -y openssl-devel ;;
                    arch|manjaro)  pacman -S --noconfirm base-devel openssl ;;
                    *) warn "Please install build tools (gcc, make, pkg-config, openssl-dev) manually" ;;
                esac
            fi
            ;;
    esac
}

# ─── Build Workshop Project ───────────────────────────────────
build_project() {
    info "Building workshop project..."

    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

    cd "$PROJECT_DIR"

    if [ -f "Cargo.toml" ]; then
        info "Running cargo build (this may take a minute on first run)..."
        if cargo build 2>&1; then
            ok "Project builds successfully"
        else
            fail "Project build failed. Check error messages above."
            ERRORS=$((ERRORS + 1))
        fi

        info "Running cargo test (verifying test harness)..."
        if cargo test --no-run 2>&1; then
            ok "Tests compile successfully"
        else
            warn "Test compilation had issues (this may be expected for TODO exercises)"
        fi
    else
        fail "Cargo.toml not found. Are you in the workshop directory?"
        ERRORS=$((ERRORS + 1))
    fi
}

# ─── Summary ──────────────────────────────────────────────────
summary() {
    echo ""
    echo "═══════════════════════════════════════════════════════"
    if [ $ERRORS -eq 0 ]; then
        echo -e "${GREEN}  ✓ All checks passed! You're ready for the workshop.${NC}"
    else
        echo -e "${RED}  ✗ $ERRORS check(s) failed. Please fix the issues above.${NC}"
        echo ""
        echo "  Need help? Reach out to the workshop facilitator or file"
        echo "  an issue at: https://github.com/dataorc/type-safety-workshop/issues"
    fi
    echo "═══════════════════════════════════════════════════════"
    echo ""
    echo "  Quick test — run this now:"
    echo "    cargo run --bin exercise_01_newtypes"
    echo ""
}

# ─── Run ──────────────────────────────────────────────────────
detect_os
check_git
check_rust
check_cargo
check_os_deps
check_editor
build_project
summary
