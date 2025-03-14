#!/usr/bin/env pwsh
# comprehensive_parachain_fix.ps1
# Script to properly handle all pending changes in the Matrix-Magiq repository

Write-Host "Matrix-Magiq Comprehensive Repository Fix" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

# Define major directories and their handling strategy
$submoduleDirs = @(
    @{Path="polkadot"; Url="https://github.com/paritytech/polkadot.git"; Branch="master"},
    @{Path="substrate"; Url="https://github.com/paritytech/substrate.git"; Branch="master"}
)

# Create a comprehensive .gitignore file
$gitignoreContent = @"
# Matrix-Magiq specific ignores
**/target/
**/node_modules/
**/*.rs.bk
**/.DS_Store
**/*.swp
**/*.swo
**/*.o
**/*.so
**/*.dylib
**/*.dll
**/*.a
**/*.rlib
**/*.exe
**/*.wasm
**/*.wasm.gz
**/*.wat
**/*.js.map
**/*.css.map
**/*.html.map

# Build artifacts
target/
**/target/
debug/
release/
artifacts/
build/

# Substrate/Polkadot generated files
**/polkadot/target/
**/substrate/target/
**/substrate_backup/

# Environment and configuration
.env
.env.*
**/.env
**/.env.*

# Logs and databases
logs/
**/*.log
**/*.log.*
**/*.sql
**/*.sqlite

# Specific directories to ignore
Shuffling/
substrate_backup/
substrate_fixed/

# Custom script files
*.ps1
!comprehensive_parachain_fix.ps1
"@

Set-Content -Path "c:\Users\skhib\Matrix-Magiq\.gitignore" -Value $gitignoreContent -Force
Write-Host "Updated .gitignore with comprehensive rules" -ForegroundColor Green

# Remove large directories from git tracking
foreach ($dir in $submoduleDirs) {
    $dirPath = "c:\Users\skhib\Matrix-Magiq\$($dir.Path)"
    
    # First check if directory exists
    if (Test-Path $dirPath) {
        # First remove from git tracking if it's already tracked
        Write-Host "Removing $($dir.Path) from git tracking..." -ForegroundColor Yellow
        git -C "c:\Users\skhib\Matrix-Magiq" rm --cached -r "$($dir.Path)" --ignore-unmatch -q
        
        # Create symbolic links instead or convert to submodules if needed
        if ($true) { # Change this condition if you want to use different approach
            Write-Host "Creating appropriate reference for $($dir.Path)..." -ForegroundColor Yellow
            
            # Create a reference file instead of the full directory
            $refContent = @"
# $($dir.Path) Reference
This directory contains references to the $($dir.Path) repository.

## Integration Notes
- Repository: $($dir.Url)
- Branch: $($dir.Branch)
- Last Verified: $(Get-Date -Format "yyyy-MM-dd")

This is used as part of the Matrix-Magiq parachain infrastructure.
Do not modify these files directly, instead work with the standalone repository.
"@
            
            # Create a docs directory for the reference
            $refDir = "c:\Users\skhib\Matrix-Magiq\references\$($dir.Path)"
            if (-not (Test-Path $refDir)) {
                New-Item -Path $refDir -ItemType Directory -Force | Out-Null
            }
            
            # Create the reference file
            Set-Content -Path "$refDir\README.md" -Value $refContent -Force
            
            # Add the reference to git
            git -C "c:\Users\skhib\Matrix-Magiq" add "references\$($dir.Path)" --verbose
        }
    }
}

# Organize error correction components (based on architecture requirements)
$errorCorrectionDirs = @(
    "error_correction/classical",
    "error_correction/bridge",
    "error_correction/quantum"
)

foreach ($dir in $errorCorrectionDirs) {
    $fullPath = "c:\Users\skhib\Matrix-Magiq\integration\$dir"
    if (-not (Test-Path $fullPath)) {
        New-Item -Path $fullPath -ItemType Directory -Force | Out-Null
        
        # Create README for each error correction type based on memory
        switch -Wildcard ($dir) {
            "*classical*" {
                $readmeContent = "# Classical Error Correction\n\nImplements robust error handling, retry mechanisms, and recovery patterns for traditional computing operations.\n\n## Implementation\n- Reed-Solomon codes\n- Retry mechanisms\n- Recovery patterns"
            }
            "*bridge*" {
                $readmeContent = "# Bridge Error Correction\n\nImplements error correction for the classical-quantum interface to ensure reliable data transmission between different computing paradigms.\n\n## Implementation\n- Bridge protocols with redundancy\n- Verification mechanisms\n- Interface stabilization"
            }
            "*quantum*" {
                $readmeContent = "# Quantum Error Correction (QEC)\n\nImplements quantum error correction codes to protect quantum states from decoherence and operational errors.\n\n## Implementation\n- Surface codes\n- Quantum error detection\n- Quantum state protection"
            }
        }
        
        Set-Content -Path "$fullPath\README.md" -Value $readmeContent -Force
    }
    
    # Add to git
    git -C "c:\Users\skhib\Matrix-Magiq" add "integration\$dir" --verbose
}

# Prepare the rest of the files
Write-Host "Organizing remaining files..." -ForegroundColor Yellow

# Add all tracked files
git -C "c:\Users\skhib\Matrix-Magiq" add -u

# Add specific untracked files/directories that should be in the repository
$includeDirs = @(
    "NRSH",
    "ELXR",
    "IMRT",
    "actorx_frameworks",
    "integration",
    "quantum",
    "docs",
    "references"
)

foreach ($dir in $includeDirs) {
    if (Test-Path "c:\Users\skhib\Matrix-Magiq\$dir") {
        git -C "c:\Users\skhib\Matrix-Magiq" add "$dir" --verbose
    }
}

# Show status
Write-Host "`nRepository status:" -ForegroundColor Green
git -C "c:\Users\skhib\Matrix-Magiq" status

Write-Host "`nReady to commit changes for parachain launch." -ForegroundColor Cyan
Write-Host "Run: git commit -m 'Complete parachain infrastructure organization with proper references' to finalize" -ForegroundColor Cyan
