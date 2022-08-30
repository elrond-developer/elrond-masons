#!/bin/bash

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq8hqpkgtudcxl6m30q46786ycq0nhqz5vk7hq7nfxv8 --function="ping" --value="6000000000000000000" --chain="D" --pem="/home/anon/Desktop/dapp/wallet/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

sleep 16

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq8hqpkgtudcxl6m30q46786ycq0nhqz5vk7hq7nfxv8 --function="pong"  --chain="D" --pem="/home/anon/Desktop/dapp/wallet/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

sleep 2
