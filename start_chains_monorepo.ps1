#!/usr/bin/env pwsh
# start_chains_monorepo.ps1
# Simple script to start Matrix-Magiq chains from the monorepo structure

Write-Host "Starting Matrix-Magiq Chains (Monorepo)" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan

$baseDir = "c:\Users\skhib\Matrix-Magiq"
$polkadotDir = "$baseDir\polkadot"
$nrshDir = "$baseDir\NRSH"
$elxrDir = "$baseDir\ELXR" 
$imrtDir = "$baseDir\IMRT"

# First, start the relay chain (polkadot)
Write-Host "Starting Polkadot relay chain..." -ForegroundColor Yellow
Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $polkadotDir && cargo run --release -- --chain=rococo-local --validator --rpc-cors=all --port=30333 --ws-port=9944 --rpc-port=9933" -WindowStyle Normal

# Wait for relay chain startup
Write-Host "Waiting for relay chain to initialize (15 seconds)..." -ForegroundColor Yellow
Start-Sleep -Seconds 15

# Start NRSH
Write-Host "Starting NRSH chain..." -ForegroundColor Yellow
Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $nrshDir && cargo run --release -- --chain=dev --validator --rpc-cors=all --port=30334 --ws-port=9945 --rpc-port=9934" -WindowStyle Normal

# Start ELXR
Write-Host "Starting ELXR chain..." -ForegroundColor Yellow
Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $elxrDir && cargo run --release -- --chain=dev --validator --rpc-cors=all --port=30335 --ws-port=9946 --rpc-port=9935" -WindowStyle Normal

# Start IMRT
Write-Host "Starting IMRT chain..." -ForegroundColor Yellow
Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "cd $imrtDir && cargo run --release -- --chain=dev --validator --rpc-cors=all --port=30336 --ws-port=9947 --rpc-port=9936" -WindowStyle Normal

Write-Host "`nAll Matrix-Magiq chains started!" -ForegroundColor Green
Write-Host "WebSocket endpoints:" -ForegroundColor White
Write-Host "- Relay Chain: ws://127.0.0.1:9944" -ForegroundColor White
Write-Host "- NRSH: ws://127.0.0.1:9945" -ForegroundColor White
Write-Host "- ELXR: ws://127.0.0.1:9946" -ForegroundColor White
Write-Host "- IMRT: ws://127.0.0.1:9947" -ForegroundColor White
