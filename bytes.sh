#!/bin/bash

contract_id="85d550bf7aef54afd4bfd79a5249520b56d161b5e506f799345b4f7d5365a055"
source_key="SAYDFQKU3YNKGLXKGL3Q2WRYCTRTQGOAQK7V5LS2HLJ2FXBZTCG4LYCN"

get_bytesn="$(
soroban contract invoke \
--id "$contract_id" \
--wasm ./target/wasm32-unknown-unknown/release/data_types.wasm \
--source "$source_key" \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022' \
-- get_bytesn
)"
echo "get_bytesn: $get_bytesn"