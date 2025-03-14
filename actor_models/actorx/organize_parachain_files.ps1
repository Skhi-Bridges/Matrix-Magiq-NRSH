#!/usr/bin/env pwsh
# organize_parachain_files.ps1
# Script to organize and commit parachain files for Matrix-Magiq launch

Write-Host "Matrix-Magiq Parachain Launch Organization" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

# Define directories to be tracked
$chainDirs = @("NRSH", "ELXR", "IMRT", "actorx_frameworks", "integration", "quantum")
$docsDir = "docs"
$infrastructureDirs = @("polkadot", "substrate")

# Create .gitignore for large directories
$gitignoreContent = @"
# Matrix-Magiq specific ignores
target/
substrate_backup/
**/target/
**/*.rs.bk
Shuffling/

# Dependency directories
node_modules/

# Environment files
.env
"@

Set-Content -Path "c:\Users\skhib\Matrix-Magiq\.gitignore" -Value $gitignoreContent -Force

# Function to add directories with git add
function Add-GitDirectory {
    param (
        [string]$directory
    )
    
    if (Test-Path "c:\Users\skhib\Matrix-Magiq\$directory") {
        Write-Host "Adding $directory files to git..." -ForegroundColor Yellow
        git -C "c:\Users\skhib\Matrix-Magiq" add "$directory" --verbose
        return $true
    } else {
        Write-Host "Directory $directory not found, skipping..." -ForegroundColor Red
        return $false
    }
}

# Add main chain directories
foreach ($dir in $chainDirs) {
    Add-GitDirectory -directory $dir
}

# Add docs directory
Add-GitDirectory -directory $docsDir

# Add infrastructure directories
foreach ($dir in $infrastructureDirs) {
    Add-GitDirectory -directory $dir
}

# Prepare commit
Write-Host "Preparing commit for parachain launch..." -ForegroundColor Green
git -C "c:\Users\skhib\Matrix-Magiq" status

Write-Host "Ready to commit files for parachain launch!"
Write-Host "Now run: git commit -m 'Prepare parachain files for launch' to commit changes"
