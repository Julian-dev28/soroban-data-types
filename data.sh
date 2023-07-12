#!/bin/bash

contract_id="105d9e650077c3ed4f0a566728a52a552498d36d4ac1ef90323d7a2019d2fc8c"
source_key="SAYDFQKU3YNKGLXKGL3Q2WRYCTRTQGOAQK7V5LS2HLJ2FXBZTCG4LYCN"

ExampleEnum_A="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_a_address
)"
echo "ExampleEnum_A: $ExampleEnum_A"

ExampleEnum_B="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_b_address
)"
echo "ExampleEnum_B: $ExampleEnum_B"

ExampleEnum_C="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_c_u32 \
--slot 0
)"

echo "ExampleEnum_C: $ExampleEnum_C"

ExampleEnum_D="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_d_symbol
)"

echo "ExampleEnum_D: $ExampleEnum_D"

get_vec_address="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_vec_address
)"

echo "VectorAddress: $get_vec_address"

current_address="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_address
)"
echo "contract address: $current_address"

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

deadline="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_deadline
)"
echo "deadline: $deadline"

get_status="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_status
)"
echo "status: $get_status"

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