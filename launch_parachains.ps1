#!/usr/bin/env pwsh
# launch_parachains.ps1
# Script to launch all Matrix-Magiq parachains

Write-Host "Matrix-Magiq Parachain Launch" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan

# Configuration
$baseDir = "c:\Users\skhib"
$nrshPath = "$baseDir\Matrix-Magiq-NRSH"
$elxrPath = "$baseDir\Matrix-Magiq-ELXR"
$imrtPath = "$baseDir\Matrix-Magiq-IMRT"
$relayChainPath = "$baseDir\Matrix-Magiq\polkadot"

# Function to launch a parachain
function Launch-Parachain {
    param (
        [string]$chainName,
        [string]$chainPath,
        [int]$port,
        [int]$wsPort,
        [int]$rpcPort
    )
    
    Write-Host "Launching $chainName..." -ForegroundColor Green
    
    # Check if chain node exists
    if (-not (Test-Path "$chainPath\target\release\node.exe")) {
        Write-Host "Building $chainName node..." -ForegroundColor Yellow
        Push-Location $chainPath
        cargo build --release
        Pop-Location
    }
    
    # Launch the node with appropriate ports
    Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $chainPath && target\release\node.exe --chain=dev --validator --rpc-cors=all --port=$port --ws-port=$wsPort --rpc-port=$rpcPort --name=$chainName-validator" -WindowStyle Normal
    
    Write-Host "$chainName launched on ports: P2P=$port, WebSocket=$wsPort, RPC=$rpcPort" -ForegroundColor Green
}

# Function to launch a relay chain
function Launch-RelayChain {
    param (
        [int]$port,
        [int]$wsPort,
        [int]$rpcPort
    )
    
    Write-Host "Launching Relay Chain..." -ForegroundColor Green
    
    # Check if polkadot binary exists
    if (-not (Test-Path "$relayChainPath\target\release\polkadot.exe")) {
        Write-Host "Building Polkadot relay chain..." -ForegroundColor Yellow
        Push-Location $relayChainPath
        cargo build --release
        Pop-Location
    }
    
    # Launch the relay chain
    Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $relayChainPath && target\release\polkadot.exe --chain=rococo-local --validator --rpc-cors=all --port=$port --ws-port=$wsPort --rpc-port=$rpcPort --name=relay-validator" -WindowStyle Normal
    
    Write-Host "Relay Chain launched on ports: P2P=$port, WebSocket=$wsPort, RPC=$rpcPort" -ForegroundColor Green
}

# Launch the relay chain first
Launch-RelayChain -port 30333 -wsPort 9944 -rpcPort 9933

# Wait for relay chain to initialize
Write-Host "Waiting for relay chain to initialize..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Launch parachains
Launch-Parachain -chainName "NRSH" -chainPath $nrshPath -port 30334 -wsPort 9945 -rpcPort 9934
Launch-Parachain -chainName "ELXR" -chainPath $elxrPath -port 30335 -wsPort 9946 -rpcPort 9935
Launch-Parachain -chainName "IMRT" -chainPath $imrtPath -port 30336 -wsPort 9947 -rpcPort 9936

Write-Host "All parachains launched successfully!" -ForegroundColor Cyan
Write-Host "You can monitor them through their respective WebSocket ports:" -ForegroundColor Cyan
Write-Host "- Relay Chain: ws://127.0.0.1:9944" -ForegroundColor White
Write-Host "- NRSH Chain: ws://127.0.0.1:9945" -ForegroundColor White
Write-Host "- ELXR Chain: ws://127.0.0.1:9946" -ForegroundColor White
Write-Host "- IMRT Chain: ws://127.0.0.1:9947" -ForegroundColor White
