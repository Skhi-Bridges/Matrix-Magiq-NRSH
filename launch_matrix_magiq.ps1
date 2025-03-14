#!/usr/bin/env pwsh
# launch_matrix_magiq.ps1
# Script to launch all Matrix-Magiq parachains from the monorepo structure

Write-Host "Matrix-Magiq Parachain Launch" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan

# Configuration
$baseDir = "c:\Users\skhib\Matrix-Magiq"
$polkadotPath = "$baseDir\polkadot"
$nrshPath = "$baseDir\NRSH"
$elxrPath = "$baseDir\ELXR"
$imrtPath = "$baseDir\IMRT"

# Function to launch a parachain node
function Launch-Parachain {
    param (
        [string]$chainName,
        [string]$chainPath,
        [int]$port,
        [int]$wsPort,
        [int]$rpcPort
    )
    
    Write-Host "Launching $chainName..." -ForegroundColor Green
    
    # Launch the node with appropriate ports
    # The exact binary name and path may vary based on your structure
    Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $chainPath && cargo run --release -- --chain=dev --validator --rpc-cors=all --port=$port --ws-port=$wsPort --rpc-port=$rpcPort --name=$chainName-validator" -WindowStyle Normal
    
    Write-Host "$chainName launched on ports: P2P=$port, WebSocket=$wsPort, RPC=$rpcPort" -ForegroundColor Green
}

# Function to launch the relay chain
function Launch-RelayChain {
    param (
        [int]$port,
        [int]$wsPort,
        [int]$rpcPort
    )
    
    Write-Host "Launching Polkadot Relay Chain..." -ForegroundColor Green
    
    # Launch the relay chain
    Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $polkadotPath && cargo run --release -- --chain=rococo-local --validator --rpc-cors=all --port=$port --ws-port=$wsPort --rpc-port=$rpcPort --name=relay-validator" -WindowStyle Normal
    
    Write-Host "Relay Chain launched on ports: P2P=$port, WebSocket=$wsPort, RPC=$rpcPort" -ForegroundColor Green
}

# Launch the relay chain first
Write-Host "Starting the Polkadot relay chain..." -ForegroundColor Yellow
Launch-RelayChain -port 30333 -wsPort 9944 -rpcPort 9933

# Wait for relay chain to initialize
Write-Host "Waiting for relay chain to initialize..." -ForegroundColor Yellow
Start-Sleep -Seconds 15

# Launch parachains
Write-Host "Starting NRSH (Nourish Chain)..." -ForegroundColor Yellow
Launch-Parachain -chainName "NRSH" -chainPath $nrshPath -port 30334 -wsPort 9945 -rpcPort 9934

Write-Host "Starting ELXR (Elixir Chain)..." -ForegroundColor Yellow
Launch-Parachain -chainName "ELXR" -chainPath $elxrPath -port 30335 -wsPort 9946 -rpcPort 9935

Write-Host "Starting IMRT (Immortality Chain)..." -ForegroundColor Yellow
Launch-Parachain -chainName "IMRT" -chainPath $imrtPath -port 30336 -wsPort 9947 -rpcPort 9936

Write-Host "`nAll Matrix-Magiq parachains have been launched!" -ForegroundColor Cyan
Write-Host "You can monitor them through their respective WebSocket ports:" -ForegroundColor Cyan
Write-Host "- Relay Chain: ws://127.0.0.1:9944" -ForegroundColor White
Write-Host "- NRSH Chain: ws://127.0.0.1:9945" -ForegroundColor White
Write-Host "- ELXR Chain: ws://127.0.0.1:9946" -ForegroundColor White
Write-Host "- IMRT Chain: ws://127.0.0.1:9947" -ForegroundColor White
