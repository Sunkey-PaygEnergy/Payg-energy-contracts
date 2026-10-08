$ErrorActionPreference = "Stop"

Write-Host "==============================================" -ForegroundColor Cyan
Write-Host "Sunkey-PaygEnergy: Soroban Testnet Deployment" -ForegroundColor Cyan
Write-Host "==============================================" -ForegroundColor Cyan

Write-Host "[1/6] Running unit test suite..." -ForegroundColor Yellow
cargo test

Write-Host "[2/6] Generating or funding deployer identity on testnet..." -ForegroundColor Yellow
$deployerExists = $false
try {
    $deployerAddr = (stellar keys address deployer 2>$null)
    if ($deployerAddr) { $deployerExists = $true }
} catch {
    $deployerExists = $false
}

if (-not $deployerExists) {
    Write-Host "Generating deployer key and funding with friendbot..."
    stellar keys generate deployer --network testnet --fund
} else {
    Write-Host "Deployer already exists: $deployerAddr"
    try {
        stellar keys fund deployer --network testnet
    } catch {}
}

$deployerAddr = (stellar keys address deployer).Trim()
Write-Host "Deployer Address: $deployerAddr" -ForegroundColor Green

Write-Host "[3/6] Building contract WASM bytecode..." -ForegroundColor Yellow
stellar contract build

$wasmPath = "target/wasm32-unknown-unknown/release/sunkey_contracts.wasm"
if (-not (Test-Path $wasmPath)) {
    $wasmPath = "target/wasm32v1-none/release/sunkey_contracts.wasm"
}

if (-not (Test-Path $wasmPath)) {
    Write-Error "Could not find compiled WASM file."
    exit 1
}

Write-Host "Using WASM file: $wasmPath" -ForegroundColor Green

Write-Host "[4/6] Deploying contract to Stellar Testnet..." -ForegroundColor Yellow
$contractId = (stellar contract deploy `
  --wasm $wasmPath `
  --source deployer `
  --network testnet `
  --alias sunkey).Trim()

Write-Host "Contract deployed with ID: $contractId" -ForegroundColor Green

Write-Host "[5/6] Initializing Sunkey contract with deployer as admin..." -ForegroundColor Yellow
stellar contract invoke `
  --id sunkey `
  --source deployer `
  --network testnet `
  -- `
  initialize --admin $deployerAddr

Write-Host "[6/6] Fetching native XLM contract ID on testnet..." -ForegroundColor Yellow
$nativeTokenId = (stellar contract id asset --asset native --network testnet).Trim()
Write-Host "Native XLM SAC Contract ID: $nativeTokenId" -ForegroundColor Green

if (-not (Test-Path "deployments")) {
    New-Item -ItemType Directory -Path "deployments" | Out-Null
}

$deploymentJson = @{
    network = "testnet"
    networkPassphrase = "Test SDF Network ; September 2015"
    rpcUrl = "https://soroban-testnet.stellar.org"
    contractAlias = "sunkey"
    contractId = $contractId
    adminAddress = $deployerAddr
    nativeTokenContractId = $nativeTokenId
    deployedAt = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
} | ConvertTo-Json -Depth 4

Set-Content -Path "deployments/testnet.json" -Value $deploymentJson
Write-Host "Deployment complete! State saved to deployments/testnet.json" -ForegroundColor Cyan
