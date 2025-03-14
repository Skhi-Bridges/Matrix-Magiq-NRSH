#!/usr/bin/env pwsh
# fix_parachain_repos.ps1
# Script to properly handle embedded repositories and organize files for the Matrix-Magiq parachain launch

Write-Host "Matrix-Magiq Parachain Launch - Repository Fixes" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# First, unstage everything to start fresh
Write-Host "Resetting staged changes..." -ForegroundColor Yellow
git -C "c:\Users\skhib\Matrix-Magiq" reset

# Fix embedded repositories
Write-Host "Fixing embedded repositories..." -ForegroundColor Yellow

# Define embedded repos
$embeddedRepos = @("ELXR", "polkadot", "substrate")

foreach ($repo in $embeddedRepos) {
    $repoPath = "c:\Users\skhib\Matrix-Magiq\$repo"
    if (Test-Path "$repoPath\.git") {
        Write-Host "Converting $repo to proper content..." -ForegroundColor Green
        # Remove .git directory to convert from embedded repo to regular directory
        Remove-Item -Path "$repoPath\.git" -Recurse -Force
    }
}

# Update .gitignore for better organization
$gitignoreContent = @"
# Matrix-Magiq specific ignores
**/target/
**/node_modules/
**/*.rs.bk
**/.DS_Store
**/*.swp
**/*.swo

# Specific directories to ignore
substrate_backup/
Shuffling/

# Build artifacts
**/target/
**/*.o
**/*.so
**/*.dylib
**/*.dll
**/*.a

# Environment files
.env
.env.local
"@

Set-Content -Path "c:\Users\skhib\Matrix-Magiq\.gitignore" -Value $gitignoreContent -Force

# Create organizational directories if they don't exist
$ensureDirs = @(
    "c:\Users\skhib\Matrix-Magiq\NRSH\src",
    "c:\Users\skhib\Matrix-Magiq\NRSH\runtime",
    "c:\Users\skhib\Matrix-Magiq\NRSH\telemetry",
    "c:\Users\skhib\Matrix-Magiq\NRSH\contracts",
    "c:\Users\skhib\Matrix-Magiq\ELXR\src",
    "c:\Users\skhib\Matrix-Magiq\ELXR\runtime",
    "c:\Users\skhib\Matrix-Magiq\ELXR\contracts",
    "c:\Users\skhib\Matrix-Magiq\IMRT\src",
    "c:\Users\skhib\Matrix-Magiq\integration\error_correction"
)

foreach ($dir in $ensureDirs) {
    if (-not (Test-Path $dir)) {
        New-Item -Path $dir -ItemType Directory -Force | Out-Null
        Write-Host "Created directory: $dir" -ForegroundColor Green
    }
}

# Stage important files with proper organization
Write-Host "Staging parachain files..." -ForegroundColor Yellow

# Add core components
git -C "c:\Users\skhib\Matrix-Magiq" add .gitignore
git -C "c:\Users\skhib\Matrix-Magiq" add NRSH --verbose
git -C "c:\Users\skhib\Matrix-Magiq" add ELXR --verbose
git -C "c:\Users\skhib\Matrix-Magiq" add IMRT --verbose
git -C "c:\Users\skhib\Matrix-Magiq" add actorx_frameworks --verbose
git -C "c:\Users\skhib\Matrix-Magiq" add integration --verbose
git -C "c:\Users\skhib\Matrix-Magiq" add quantum --verbose
git -C "c:\Users\skhib\Matrix-Magiq" add docs --verbose

# Show status
Write-Host "`nRepository status:" -ForegroundColor Green
git -C "c:\Users\skhib\Matrix-Magiq" status

Write-Host "`nReady to commit changes for parachain launch." -ForegroundColor Cyan
Write-Host "Run: git commit -m 'Prepare parachain infrastructure for launch' to complete" -ForegroundColor Cyan
