# OpenAPI Field Explorer - Installation Script for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/antikkorps/openapi_explorer/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

# Configuration
$Repo = "antikkorps/openapi_explorer"
$BinaryName = "openapi-explorer.exe"
$InstallDir = "$env:USERPROFILE\.local\bin"

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Blue
Write-Host "║   OpenAPI Field Explorer - Installation       ║" -ForegroundColor Blue
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Blue
Write-Host ""

# Check if Rust is installed
try {
    $rustVersion = (cargo --version).Split(" ")[1]
    Write-ColorOutput Green "✓ Rust $rustVersion detected"
} catch {
    Write-ColorOutput Yellow "⚠  Rust is not installed."
    Write-Host "   OpenAPI Explorer requires Rust to build from source."
    Write-Host ""

    $install = Read-Host "   Would you like to install Rust now? (y/N)"
    if ($install -match '^[Yy]') {
        Write-ColorOutput Blue "→ Downloading Rust installer..."

        # Download and run rustup-init
        $rustupInit = "$env:TEMP\rustup-init.exe"
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupInit

        Write-ColorOutput Blue "→ Installing Rust..."
        Start-Process -FilePath $rustupInit -ArgumentList "-y" -Wait

        # Reload PATH
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

        Write-ColorOutput Green "✓ Rust installed successfully"
    } else {
        Write-ColorOutput Red "✗ Installation cancelled. Please install Rust first:"
        Write-Host "   https://rustup.rs/"
        exit 1
    }
}

Write-Host ""

# Choose installation method
Write-ColorOutput Blue "Choose installation method:"
Write-Host "  1) Build from source (latest, requires ~2min)"
Write-Host "  2) Download pre-built binary (faster, if available)"
Write-Host ""
$choice = Read-Host "Enter choice [1-2]"

$binaryPath = ""

switch ($choice) {
    "2" {
        # Try to download pre-built binary
        Write-ColorOutput Blue "→ Detecting system..."
        $arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
        $target = "$arch-pc-windows-msvc"

        Write-ColorOutput Blue "→ Downloading binary for $target..."

        $latestUrl = "https://github.com/$Repo/releases/latest/download/$BinaryName"
        $tempBinary = "$env:TEMP\$BinaryName"

        try {
            Invoke-WebRequest -Uri $latestUrl -OutFile $tempBinary
            Write-ColorOutput Green "✓ Binary downloaded"
            $binaryPath = $tempBinary
        } catch {
            Write-ColorOutput Yellow "⚠  Pre-built binary not available for $target"
            Write-Host "   Falling back to building from source..."
            $choice = "1"
        }
    }
}

# Build from source if needed
if ($choice -eq "1") {
    Write-ColorOutput Blue "→ Cloning repository..."
    $tempDir = "$env:TEMP\openapi-explorer-$(Get-Random)"

    try {
        git clone "https://github.com/$Repo.git" $tempDir 2>$null
        Write-ColorOutput Green "✓ Repository cloned"
    } catch {
        Write-ColorOutput Red "✗ Failed to clone repository"
        exit 1
    }

    Write-Host ""
    Write-ColorOutput Blue "→ Building release binary..."
    Write-ColorOutput Yellow "   This may take a few minutes..."

    Push-Location $tempDir
    try {
        cargo build --release --quiet
        $binaryPath = "$tempDir\target\release\$BinaryName"
        Write-ColorOutput Green "✓ Build successful"
    } catch {
        Write-ColorOutput Red "✗ Build failed"
        Pop-Location
        exit 1
    } finally {
        Pop-Location
    }
}

# Create installation directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Install binary
Write-ColorOutput Blue "→ Installing to $InstallDir..."
Copy-Item $binaryPath "$InstallDir\$BinaryName" -Force
Write-ColorOutput Green "✓ Binary installed"

# Check if install directory is in PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    Write-Host ""
    Write-ColorOutput Yellow "⚠  $InstallDir is not in your PATH"
    Write-Host ""
    $addPath = Read-Host "   Would you like to add it now? (y/N)"

    if ($addPath -match '^[Yy]') {
        $newPath = "$userPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path += ";$InstallDir"
        Write-ColorOutput Green "✓ PATH updated"
    } else {
        Write-Host ""
        Write-Host "   To add manually, run:"
        Write-ColorOutput Blue '   $env:Path += ";' + $InstallDir + '"'
        Write-Host ""
    }
}

# Cleanup
if ($choice -eq "1" -and (Test-Path $tempDir)) {
    Remove-Item -Recurse -Force $tempDir
}

# Verify installation
Write-Host ""
if (Test-Path "$InstallDir\$BinaryName") {
    try {
        $version = & "$InstallDir\$BinaryName" --version 2>$null
    } catch {
        $version = "unknown"
    }

    Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║   ✓ Installation Complete!                    ║" -ForegroundColor Green
    Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    Write-Host "  Run: " -NoNewline
    Write-ColorOutput Blue "openapi-explorer examples\petstore.json"
    Write-Host "  Help: " -NoNewline
    Write-ColorOutput Blue "openapi-explorer --help"
    Write-Host ""
} else {
    Write-ColorOutput Red "✗ Installation verification failed"
    exit 1
}
