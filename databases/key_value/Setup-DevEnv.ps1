$ErrorActionPreference = 'Stop'

$devRoot = $PSScriptRoot
$luaVersion = "5.4"

function Write-Status($message) {
    Write-Host "`n[STATUS] $message" -ForegroundColor Cyan
}

# Create directory structure
$directories = @(
    "nix",
    "nix/modules",
    "lua/zerobrane",
    "lua/custom_modules",
    "ao-processes/core",
    "ao-processes/metrics",
    "ao-processes/tuning",
    "databases/quantum_db",
    "databases/key_value",
    "databases/vector_stores",
    "hdr/vector",
    "qsr-core/language/reticulation",
    "config",
    ".luarocks",
    "local-cache"
)

foreach ($dir in $directories) {
    $path = Join-Path $devRoot $dir
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path | Out-Null
        Write-Status "Created directory: $dir"
    }
}

# Check if WSL and Nix are installed
Write-Status "Checking WSL and Nix installation..."
$wsl = wsl --list
if ($LASTEXITCODE -ne 0) {
    Write-Status "Please run Setup-Nix.ps1 first to install WSL and Nix"
    exit
}

# Initialize Nix development environment in WSL
Write-Status "Initializing Nix development environment..."
wsl -d Ubuntu bash -c "
    cd /mnt/c/Users/skhib/WindSurf
    . ~/.nix-profile/etc/profile.d/nix.sh
    
    # Build Lua environment
    nix build .#qsr-core .#ao-processes
    
    # Enter development shell
    nix develop
"

Write-Status @"
Lua development environment setup completed!

Available commands in WSL:
1. Lua Development:
   - lua              # Start Lua REPL
   - luarocks        # Package manager
   - zbstudio        # IDE

2. Package Management:
   - nix build .#qsr-core     # Build QSR core
   - nix build .#ao-processes # Build AO processes

3. Development Shell:
   - nix develop     # Enter development environment

Project structure:
- qsr-core/         # QSR language implementation
- ao-processes/     # AO process definitions
- lua/              # Custom Lua modules
- config/           # LuaRocks configuration

To start developing:
1. Open WSL: wsl
2. Navigate to project: cd /mnt/c/Users/skhib/WindSurf
3. Enter Nix shell: nix develop
4. Start ZeroBrane: zbstudio
"@
