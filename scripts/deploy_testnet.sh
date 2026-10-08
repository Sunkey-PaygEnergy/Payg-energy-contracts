#!/usr/bin/env bash
set -euo pipefail

echo "=============================================="
echo "Sunkey-PaygEnergy: Soroban Testnet Deployment"
echo "=============================================="

echo "[1/6] Running unit test suite..."
cargo test

echo "[2/6] Generating or funding deployer identity on testnet..."
if ! stellar keys address deployer >/dev/null 2>&1; then
    echo "Generating new deployer key and funding with friendbot..."
    stellar keys generate deployer --network testnet --fund
else
    echo "Deployer key found: $(stellar keys address deployer)"
    echo "Ensuring deployer has funds on testnet..."
    stellar keys fund deployer --network testnet || true
fi

DEPLOYER_ADDR=$(stellar keys address deployer)
echo "Deployer Address: ${DEPLOYER_ADDR}"

echo "[3/6] Building contract WASM bytecode..."
stellar contract build

WASM_PATH="target/wasm32-unknown-unknown/release/sunkey_contracts.wasm"
if [ ! -f "${WASM_PATH}" ]; then
    WASM_PATH="target/wasm32v1-none/release/sunkey_contracts.wasm"
fi

if [ ! -f "${WASM_PATH}" ]; then
    echo "Error: Could not locate compiled wasm file."
    exit 1
fi

echo "Using WASM file: ${WASM_PATH}"

echo "[4/6] Deploying contract to Stellar Testnet..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "${WASM_PATH}" \
  --source deployer \
  --network testnet \
  --alias sunkey)

echo "Contract deployed with ID: ${CONTRACT_ID}"

echo "[5/6] Initializing Sunkey contract with deployer as admin..."
stellar contract invoke \
  --id sunkey \
  --source deployer \
  --network testnet \
  -- \
  initialize --admin "${DEPLOYER_ADDR}"

echo "[6/6] Fetching native XLM contract ID on testnet..."
NATIVE_TOKEN_ID=$(stellar contract id asset --asset native --network testnet)
echo "Native XLM SAC Contract ID: ${NATIVE_TOKEN_ID}"

# Output state to JSON file
mkdir -p deployments
cat <<EOF > deployments/testnet.json
{
  "network": "testnet",
  "networkPassphrase": "Test SDF Network ; September 2015",
  "rpcUrl": "https://soroban-testnet.stellar.org",
  "contractAlias": "sunkey",
  "contractId": "${CONTRACT_ID}",
  "adminAddress": "${DEPLOYER_ADDR}",
  "nativeTokenContractId": "${NATIVE_TOKEN_ID}",
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo "Deployment complete! State saved to deployments/testnet.json."
