#!/bin/bash

contract_id="105d9e650077c3ed4f0a566728a52a552498d36d4ac1ef90323d7a2019d2fc8c"
source_key="SAYDFQKU3YNKGLXKGL3Q2WRYCTRTQGOAQK7V5LS2HLJ2FXBZTCG4LYCN"

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