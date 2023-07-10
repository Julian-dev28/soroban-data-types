#!/bin/bash

contract_id="d01123d82e68de3bb745665a92566dee993fb79b0f928238a62e20503589f270"
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