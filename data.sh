#!/bin/bash

contract_id="e4a98fc1931b73c46a4e7e2037ad72d4fc6c2bebc71c1a7c080b529b2d3d4fb0"
source_key="SAYDFQKU3YNKGLXKGL3Q2WRYCTRTQGOAQK7V5LS2HLJ2FXBZTCG4LYCN"

owner="$(
        soroban contract invoke \
        --id "$contract_id" \
        --wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
        --source "$source_key" \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        -- get_b
    )"
    echo "owner: $owner"

current_address="$(
        soroban contract invoke \
        --id "$contract_id" \
        --wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
        --source "$source_key" \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        -- get_address
    )"
    echo "address: $current_address"

timestamp="$(
        soroban contract invoke \
        --id "$contract_id" \
        --wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
        --source "$source_key" \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        -- get_timestamp
    )"
    echo "timestamp: $timestamp"

current_ledger_sequence="$(
    soroban contract invoke \
        --id "$contract_id" \
        --wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
        --source "$source_key" \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        -- get_sequence
    )"
    echo "current_ledger_sequence: $current_ledger_sequence"