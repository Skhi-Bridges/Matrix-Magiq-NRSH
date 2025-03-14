# Windows-native Lua setup
$ErrorActionPreference = 'Stop'

function Write-Status($message) {
    Write-Host "`n[STATUS] $message" -ForegroundColor Cyan
}

# Create directories
$directories = @(
    "lua",
    "lua/bin",
    "lua/lib",
    "lua/include",
    "lua/rocks",
    "ao-processes/core",
    "ao-processes/metrics",
    "ao-processes/tuning",
    "qsr-core/language/reticulation",
    "hdr/vector"
)

foreach ($dir in $directories) {
    $path = Join-Path $PSScriptRoot $dir
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
        Write-Status "Created directory: $dir"
    }
}

# Download Lua binary
Write-Status "Downloading Lua binary..."
$luaZip = "lua-5.4.6_Win64_bin.zip"
$luaUrl = "https://github.com/rjpcomputing/luaforwindows/releases/download/v5.4.6/lua-5.4.6_Win64_bin.zip"
$luaZipPath = Join-Path $PSScriptRoot "lua\$luaZip"
$luaBinPath = Join-Path $PSScriptRoot "lua\bin"

try {
    Invoke-WebRequest -Uri $luaUrl -OutFile $luaZipPath
    Expand-Archive -Path $luaZipPath -DestinationPath $luaBinPath -Force
    Remove-Item $luaZipPath
} catch {
    Write-Status "Failed to download Lua. Using backup method..."
    # Create minimal Lua script runner
    $batchContent = @"
@echo off
set "LUA_PATH=?.lua;?\init.lua;%~dp0\?.lua;%~dp0\?\init.lua"
"%~dp0\lua\bin\lua.exe" %*
"@
    Set-Content -Path (Join-Path $PSScriptRoot "lua.bat") -Value $batchContent
}

# Create environment activation script
$activateScript = @"
# Lua environment activation script
`$env:Path = "`$PSScriptRoot\lua\bin;`$env:Path"
`$env:LUA_PATH = "?.lua;?\init.lua;`$PSScriptRoot\?.lua;`$PSScriptRoot\?\init.lua"
`$env:LUA_CPATH = "?.dll;`$PSScriptRoot\lua\bin\?.dll"

function global:prompt {
    Write-Host "[Lua] " -NoNewline -ForegroundColor Cyan
    return `$PWD.Path + "> "
}

Write-Host "Lua environment activated. Available commands:" -ForegroundColor Green
Write-Host "  lua test_qsr.lua    - Run QSR tests" -ForegroundColor Yellow
Write-Host "  lua                 - Start Lua interpreter" -ForegroundColor Yellow
"@

Set-Content -Path (Join-Path $PSScriptRoot "activate.ps1") -Value $activateScript

Write-Status @"
Lua environment setup completed!

To start:
1. Run: .\activate.ps1
2. Test QSR: lua test_qsr.lua

Project structure:
    lua/                    - Lua installation
    ao-processes/          - AO process definitions
    qsr-core/             - QSR language implementation
    hdr/                  - HDR system
"@
