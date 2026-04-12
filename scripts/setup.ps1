# =============================================================================
# Workshop Setup Script — Windows (PowerShell)
# Make Invalid Data Unrepresentable: Rust's Type System for Data Engineering
# =============================================================================
# Run: powershell -ExecutionPolicy Bypass -File scripts\setup.ps1
# =============================================================================

$ErrorActionPreference = "Stop"

function Write-Info  { Write-Host "[INFO] $args" -ForegroundColor Blue }
function Write-Ok    { Write-Host "[OK]   $args" -ForegroundColor Green }
function Write-Warn  { Write-Host "[WARN] $args" -ForegroundColor Yellow }
function Write-Fail  { Write-Host "[FAIL] $args" -ForegroundColor Red }

$Errors = 0

Write-Host ""
Write-Host "╔══════════════════════════════════════════════════════════╗"
Write-Host "║  Workshop Setup: Make Invalid Data Unrepresentable       ║"
Write-Host "║  Rust's Type System for Data Engineering                 ║"
Write-Host "╚══════════════════════════════════════════════════════════╝"
Write-Host ""

# ─── Check Visual Studio Build Tools ─────────────────────────
Write-Info "Checking Visual Studio Build Tools..."
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsWhere) {
    $vsInstall = & $vsWhere -latest -property installationPath 2>$null
    if ($vsInstall) {
        Write-Ok "Visual Studio Build Tools found at $vsInstall"
    } else {
        Write-Warn "VS Build Tools not detected. Rust needs MSVC C++ build tools."
        Write-Warn "Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/"
        Write-Warn "During install, select 'Desktop development with C++'"
        $Errors++
    }
} else {
    Write-Warn "VS Build Tools not detected."
    Write-Warn "Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/"
    Write-Warn "During install, select 'Desktop development with C++'"
    $Errors++
}

# ─── Check Rust ───────────────────────────────────────────────
Write-Info "Checking Rust installation..."
try {
    $rustVersion = rustc --version 2>$null
    if ($rustVersion) {
        Write-Ok "Rust found: $rustVersion"
        $version = ($rustVersion -split ' ')[1]
        $minor = [int]($version -split '\.')[1]
        if ($minor -ge 70) {
            Write-Ok "Rust version is sufficient (>= 1.70)"
        } else {
            Write-Warn "Rust version is old. Run: rustup update stable"
        }
    }
} catch {
    Write-Warn "Rust not found. Installing via rustup..."
    Write-Info "Downloading rustup-init.exe..."

    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupExe = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupExe
    Start-Process -FilePath $rustupExe -ArgumentList "-y" -Wait

    # Refresh PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

    try {
        $rustVersion = rustc --version 2>$null
        Write-Ok "Rust installed: $rustVersion"
    } catch {
        Write-Fail "Rust installation failed. Please install manually: https://rustup.rs"
        Write-Fail "After installing, close and reopen this terminal."
        $Errors++
    }
}

# ─── Check Cargo ──────────────────────────────────────────────
Write-Info "Checking cargo..."
try {
    $cargoVersion = cargo --version 2>$null
    if ($cargoVersion) {
        Write-Ok "cargo found: $cargoVersion"
    }
} catch {
    Write-Fail "cargo not found. Is Rust installed correctly?"
    Write-Fail "Try closing and reopening your terminal."
    $Errors++
}

# ─── Check Git ────────────────────────────────────────────────
Write-Info "Checking git..."
try {
    $gitVersion = git --version 2>$null
    if ($gitVersion) {
        Write-Ok "git found: $gitVersion"
    }
} catch {
    Write-Warn "git not found."
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        Write-Info "Installing git via winget..."
        winget install --id Git.Git -e --source winget
    } else {
        Write-Fail "git not found. Download: https://git-scm.com/download/win"
        $Errors++
    }
}

# ─── Check Editor ─────────────────────────────────────────────
Write-Info "Checking for VS Code..."
try {
    $codeVersion = code --version 2>$null
    if ($codeVersion) {
        Write-Ok "VS Code found"
        $extensions = code --list-extensions 2>$null
        if ($extensions -match "rust-lang.rust-analyzer") {
            Write-Ok "rust-analyzer extension installed"
        } else {
            Write-Info "Installing rust-analyzer extension..."
            code --install-extension rust-lang.rust-analyzer 2>$null
            Write-Ok "rust-analyzer installed (restart VS Code to activate)"
        }
    }
} catch {
    Write-Warn "VS Code not found. Recommended: https://code.visualstudio.com"
}

# ─── Build Project ────────────────────────────────────────────
Write-Info "Building workshop project..."
$projectDir = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Push-Location $projectDir

if (Test-Path "Cargo.toml") {
    Write-Info "Running cargo build (first run downloads dependencies)..."
    try {
        cargo build 2>&1
        Write-Ok "Project builds successfully"
    } catch {
        Write-Fail "Build failed. Check error messages above."
        $Errors++
    }
} else {
    Write-Fail "Cargo.toml not found. Make sure you're in the workshop directory."
    $Errors++
}

Pop-Location

# ─── Summary ──────────────────────────────────────────────────
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════"
if ($Errors -eq 0) {
    Write-Host "  ✓ All checks passed! You're ready for the workshop." -ForegroundColor Green
} else {
    Write-Host "  ✗ $Errors check(s) failed. Please fix the issues above." -ForegroundColor Red
    Write-Host ""
    Write-Host "  Need help? File an issue at:"
    Write-Host "  https://github.com/dataorc/type-safety-workshop/issues"
}
Write-Host "═══════════════════════════════════════════════════════"
Write-Host ""
Write-Host "  Quick test — run this now:"
Write-Host "    cargo run --bin exercise_01_newtypes"
Write-Host ""
