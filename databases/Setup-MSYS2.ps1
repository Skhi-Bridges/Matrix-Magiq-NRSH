# Install MSYS2 and Lua environment
$ErrorActionPreference = 'Stop'

function Write-Status($message) {
    Write-Host "`n[STATUS] $message" -ForegroundColor Cyan
}

# Install Chocolatey if not present
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Status "Installing Chocolatey..."
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
}

# Install MSYS2
Write-Status "Installing MSYS2..."
choco install -y msys2

# Add MSYS2 to PATH
$env:Path = "C:\tools\msys64\usr\bin;C:\tools\msys64\mingw64\bin;" + $env:Path

# Create MSYS2 setup script
$msysScript = @"
#!/usr/bin/env bash

# Update package database
pacman -Syu --noconfirm

# Install development tools
pacman -S --noconfirm \
    mingw-w64-x86_64-gcc \
    mingw-w64-x86_64-lua \
    mingw-w64-x86_64-luarocks \
    mingw-w64-x86_64-cmake \
    git \
    make

# Install Lua packages
luarocks install luafilesystem
luarocks install luasocket
luarocks install lpeg
luarocks install penlight
"@

$msysScriptPath = "C:\tools\msys64\msys2_shell.cmd"
Set-Content -Path "C:\tools\msys64\setup_env.sh" -Value $msysScript
icacls "C:\tools\msys64\setup_env.sh" /grant Everyone:F

# Create activation script
$activateScript = @"
# Activate MSYS2 Lua environment
`$env:Path = "C:\tools\msys64\usr\bin;C:\tools\msys64\mingw64\bin;`$env:Path"
`$env:LUA_PATH = "?.lua;?\init.lua;`$PSScriptRoot\?.lua;`$PSScriptRoot\?\init.lua"
`$env:LUA_CPATH = "?.dll;C:\tools\msys64\mingw64\bin\?.dll"

function global:prompt {
    Write-Host "[Lua] " -NoNewline -ForegroundColor Cyan
    return `$PWD.Path + "> "
}

Write-Host "Lua environment activated. Available commands:" -ForegroundColor Green
Write-Host "  lua test_qsr.lua    - Run QSR tests" -ForegroundColor Yellow
Write-Host "  lua                 - Start Lua interpreter" -ForegroundColor Yellow
Write-Host "  luarocks            - Package manager" -ForegroundColor Yellow
"@

Set-Content -Path (Join-Path $PSScriptRoot "activate.ps1") -Value $activateScript

Write-Status @"
MSYS2 and Lua environment setup started!

To complete setup:
1. A MSYS2 terminal will open
2. Wait for it to complete the installation
3. Close the MSYS2 terminal
4. Run: .\activate.ps1

The environment includes:
- Lua 5.4
- LuaRocks package manager
- GCC compiler
- Git and Make

To test the setup:
1. .\activate.ps1
2. lua test_qsr.lua
"@

# Start MSYS2 setup
Start-Process -FilePath "C:\tools\msys64\msys2_shell.cmd" -ArgumentList "-mingw64", "-defterm", "-no-start", "-here", "-c", "./setup_env.sh"
