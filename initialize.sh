#!/bin/bash

source_key="SAYDFQKU3YNKGLXKGL3Q2WRYCTRTQGOAQK7V5LS2HLJ2FXBZTCG4LYCN"

contractID="$(
    soroban contract deploy \
    --wasm ./target/wasm32-unknown-unknown/release/data_types.wasm  \
    --source "$source_key" \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
    )"

    echo "contractID: $contractID"

    initialize="$(
        soroban contract invoke \
        --id "$contractID" \
        --wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
        --source "$source_key" \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        -- initialize \
        --contract_owner "GAI7LBGFJWXT5H2GKUUMAMLYOWY5GAWGTHP24Z7QPMUPSYSNOSVFE6KZ"
    )"

    echo "initialization complete"