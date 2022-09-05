### MSC - non-payable - interacting and sending coins and esdt tokens
### https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqdxgx4rcv3azy4uvxld0rkwphtfkrs9xt3nlq43sce4

###### ########################################################
# Set up vs code and some helpful shortcuts
# 
###### ########################################################
0) setting up new folder in vscode
File > Preferences > Keyboard Shortcuts [Ctrl+k Ctrl+S]
File:New Folder
Alt + Shift + n

Navigating folders
< = up dirtectory
ctrl+` = terminal
ctrl+pageup pagedown

###### ########################################################
# create 4 test wallets
# collect faucet   https://devnet-wallet.elrond.com/
###### ########################################################
1) create 4 wallets
wallet=$(erdpy wallet new | cut -d" " -f2-25); printf "$wallet" | erdpy wallet derive --mnemonic ./wallets/"$wallet".pem

Login to devnet web wallet to collect faucet:
https://devnet-wallet.elrond.com/unlock/keystore

###### ########################################################
# Setup elrond workspace
# 
###### ########################################################
2) setup elrond workspace
ctrl+shift+p

###### ########################################################
# download multisig SC
# 
###### ########################################################
3a) load multisig SC from Elrond Workspace Explorer

3b) git clone
https://github.com/ElrondNetwork
searched for examples
https://github.com/ElrondNetwork/sc-examples
https://github.com/ElrondNetwork/elrond-wasm-rs/tree/master/contracts/examples
https://github.com/ElrondNetwork/elrond-wasm-rs/tree/master/contracts/examples/multisig

git clone https://github.com/ElrondNetwork/elrond-wasm-rs.git multisig-git

mkdir multisig-git2
mv multisig-git/contracts/examples/multisig/ ./multisig-git2/
cp multisig-git/contracts/examples/multisig/ ./multisig-git2/

3c) erdpy command method
erdpy contract new --template multisig multisig-cmd

###### ########################################################
# Build contract
# 
###### ########################################################
4) erdpy --verbose contract build multisig/

###### ########################################################
# Deploy contract
# http://207.244.241.38/elrond-converters/
###### ########################################################
5a) Constructing the command to deploy MS SC

Aadam
erd1z4y3lten76yz6syk4mk3xla3ag4zw5nv7yatlrq9gn80925qzxps4sx50m

Candy
erd1kets4zcgm09sejgj68k4qnjxmve6kr79cpf5yk5xumzdmk3tmydsxr2a6f

George
erd19n9u9yrsztg8xx9hvszsgxrp62xwv0mj6u3jvxy88vqp68a0d3tsr0ae2e

Patty
erd10myyrg89fzvs7l67p5d38xnmvfxt7gkk0vtwj3jlgerddd856eysrr2a24

### contract deploy
erdpy --verbose contract deploy --chain="D" --project=multisig --pem="wallets/Aadam/Aadam.pem" --arguments 3 erd1kets4zcgm09sejgj68k4qnjxmve6kr79cpf5yk5xumzdmk3tmydsxr2a6f erd19n9u9yrsztg8xx9hvszsgxrp62xwv0mj6u3jvxy88vqp68a0d3tsr0ae2e  erd10myyrg89fzvs7l67p5d38xnmvfxt7gkk0vtwj3jlgerddd856eysrr2a24 --gas-limit=120000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

SC Address:
erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf

### exp ref:
### https://devnet-explorer.elrond.com/transactions/325156efa4db284c75292faa1f19c505e28575523b1b587dbccaec16b00f28dd
### https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf

###### ########################################################
# explore the MSC to figure out what functions to use
# /home/anon/Desktop/new-multisig-test/multisig/wasm-view/src/lib.rs
# all
# /home/anon/Desktop/new-multisig-test/multisig/wasm/src/lib.rs
# mandos as a ref
# /home/anon/Desktop/new-multisig-test/multisig/mandos
###### ########################################################
6) 

### getQuorum;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="getQuorum" --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### getNumBoardMembers;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="getNumBoardMembers" --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="getNumBoardMembers" --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ########################################################
# Deposit egld to Non-payable multisig SC wallet
# 
###### ########################################################
7) Send funds External wallet --> MSC

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send; 

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ########################################################
# exporing the commands in multisig contract
#  test sending transaction
# proposeTransferExecute
###### ########################################################
8) 
### wallets
### users

### propose transfer from Aadam's
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="proposeTransferExecute" --arguments erd1z4y3lten76yz6syk4mk3xla3ag4zw5nv7yatlrq9gn80925qzxps4sx50m 3000000000000000000 0x01 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### propose transfer from patty's
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="proposeTransferExecute" --arguments erd1z4y3lten76yz6syk4mk3xla3ag4zw5nv7yatlrq9gn80925qzxps4sx50m 3000000000000000000 0x01 0x01 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### getActionSigners
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="getActionSigners" --arguments 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Aadam
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x01 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## getActionSignerCount
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="getActionSignerCount" --arguments 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - with 2 out of the 3 needed votes
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="performAction" --arguments 1 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="performAction" --arguments 1 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - George
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x01 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## getActionSignerCount
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="getActionSignerCount" --arguments 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Candy sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="performAction" --arguments 1 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ########################################################
## add a boardmember proposeAddBoardMember
###### ########################################################
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="proposeAddBoardMember" --arguments erd1z4y3lten76yz6syk4mk3xla3ag4zw5nv7yatlrq9gn80925qzxps4sx50m --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 2 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;


# SIGN - George;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 2 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Candy sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="performAction" --arguments 2 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### Aadam tests proposal to send tx
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="proposeTransferExecute" --arguments erd1z4y3lten76yz6syk4mk3xla3ag4zw5nv7yatlrq9gn80925qzxps4sx50m 3000000000000000000 0x01 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x03 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;


# SIGN - George;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x03 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Candy sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="performAction" --arguments 3 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

