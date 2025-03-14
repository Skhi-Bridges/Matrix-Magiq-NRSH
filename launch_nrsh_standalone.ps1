#!/usr/bin/env pwsh
# launch_nrsh_standalone.ps1
# Script to build and launch the NRSH (Nourish Chain) from its standalone repository

$ErrorActionPreference = "Stop"

Write-Host "Matrix-Magiq NRSH - Build and Launch" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

$repoDir = "c:\Users\skhib\Matrix-Magiq-NRSH"
Set-Location $repoDir

# Check if Rust is installed
try {
    $rustVersion = rustc --version
    Write-Host "Using $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Rust is not installed or not in PATH. Please install Rust and try again." -ForegroundColor Red
    exit 1
}

# Create build directory if it doesn't exist
$buildDir = Join-Path -Path $repoDir -ChildPath "build"
if (-not (Test-Path $buildDir)) {
    New-Item -Path $buildDir -ItemType Directory -Force | Out-Null
    Write-Host "Created build directory" -ForegroundColor Yellow
}

# Check if Cargo.toml exists
$cargoToml = Join-Path -Path $repoDir -ChildPath "Cargo.toml"
if (-not (Test-Path $cargoToml)) {
    Write-Host "Cargo.toml not found in $repoDir. Cannot build NRSH." -ForegroundColor Red
    exit 1
}

Write-Host "Building NRSH parachain..." -ForegroundColor Green
try {
    # Build in release mode
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo build failed with exit code $LASTEXITCODE"
    }
    Write-Host "Build completed successfully" -ForegroundColor Green
} catch {
    Write-Host "Build failed: $_" -ForegroundColor Red
    exit 1
}

# Set up environment variables for NRSH
$env:RUST_LOG = "info,nrsh=debug"
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/nrsh"

# Check if compiled binary exists
$nrshBinary = Join-Path -Path $repoDir -ChildPath "target\release\nrsh.exe"
if (-not (Test-Path $nrshBinary)) {
    Write-Host "NRSH binary not found at $nrshBinary" -ForegroundColor Red
    exit 1
}

# Start PostgreSQL if needed for the holographic DB
try {
    $pgService = Get-Service postgresql* -ErrorAction SilentlyContinue
    if ($pgService -and $pgService.Status -ne 'Running') {
        Write-Host "Starting PostgreSQL service..." -ForegroundColor Yellow
        Start-Service $pgService
        Start-Sleep -Seconds 5  # Give PostgreSQL time to start
    }
} catch {
    Write-Host "Failed to check/start PostgreSQL: $_" -ForegroundColor Yellow
    Write-Host "You may need to start PostgreSQL manually if NRSH requires it." -ForegroundColor Yellow
}

# Launch NRSH node
Write-Host "Launching NRSH node..." -ForegroundColor Green
Write-Host "Press Ctrl+C to stop the node" -ForegroundColor Yellow

try {
    & $nrshBinary --dev --tmp
} catch {
    Write-Host "Failed to launch NRSH node: $_" -ForegroundColor Red
    exit 1
}
