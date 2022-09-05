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
# z
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
## 1st arg = ??? number of quorum (wallets/members)

Aadam:
erd1ajnfuwue94naxg2xq8gna0rusk8w8msmleavl23wyz4vhwxz0ckqtgtraa
0x77d0d8f81c564e2e98786de5e5665317205f14fa8bed20b53b81b832929746e8

Candy:
erd185nu9z0m8m27rxtx4dmkhw73wsurlksqngkpapfj6yttntv5p7eqh5w8jg

George:
erd1zr4xnlr5nhc9wywh366we7d8j2rw5r2wmz5y2y7wvpe8q0glfs0qn74u27

Patty:
erd1wlgd37qu2e8zaxrcdhj72ejnzus9798630kjpdfmsxur9y5hgm5qwhm3mw

### contract deploy
erdpy --verbose contract deploy --chain="D" --project=multisig --pem="wallets/Aadam/Aadam.pem" --arguments 0x03 0xc0441134557a04fbb906950593defb97fc9528e2840ee0362e29545e99de8cfe 0x0e5450b01a90317698dd784c7846835039e324fb3f499e271a2b2adae6f53fe6 0x074ca9079502b8e19ed7836d256e47557630cce6731f199ae06ef09c54448bbe 0x520c755a1afcd62f11d59c8101deb171fb2f0f156489b9350b5d86156f0c14fd --gas-limit=120000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

erdpy --verbose contract deploy --chain="D" --project=multisig --pem="wallets/Aadam/Aadam.pem" --arguments 3 erd185nu9z0m8m27rxtx4dmkhw73wsurlksqngkpapfj6yttntv5p7eqh5w8jg erd1zr4xnlr5nhc9wywh366we7d8j2rw5r2wmz5y2y7wvpe8q0glfs0qn74u27 erd1wlgd37qu2e8zaxrcdhj72ejnzus9798630kjpdfmsxur9y5hgm5qwhm3mw --gas-limit=120000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

SC:
erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px
erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px

### reference non-payable MSC on devnet explorer:
#   https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px

###### ########################################################
# explore the MSC to figure out what functions to use
# /home/anon/Desktop/new-multisig-test/multisig/wasm-view/src/lib.rs
# all
# /home/anon/Desktop/new-multisig-test/multisig/wasm/src/lib.rs
# mandos as a ref
# /home/anon/Desktop/new-multisig-test/multisig/mandos
###### ########################################################
6) 

### getQuorum
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="getQuorum" --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### getNumBoardMembers
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="getNumBoardMembers" --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="getNumBoardMembers" --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ########################################################
# Deposit egld to Non-payable multisig SC wallet
# 
###### ########################################################
7) Send funds External wallet --> MSC

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send; 

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ########################################################
# test sending transaction
# proposeTransferExecute
###### ########################################################
8) 
### wallets
### users

### propose transfer from patty's
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="proposeTransferExecute" --arguments erd1ajnfuwue94naxg2xq8gna0rusk8w8msmleavl23wyz4vhwxz0ckqtgtraa 3000000000000000000 0x01 0x01 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send


### getActionSigners
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="getActionSigners" --arguments 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px  --function="sign" --arguments 0x01 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## getActionSignerCount
) getActionSignerCount
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="getActionSignerCount" --arguments 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - with 2 out of the 3 needed votes
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="performAction" --arguments 1 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - George
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px  --function="sign" --arguments 0x01 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Candy sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="performAction" --arguments 1 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## add a boardmember proposeAddBoardMember
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="proposeAddBoardMember" --arguments erd1ajnfuwue94naxg2xq8gna0rusk8w8msmleavl23wyz4vhwxz0ckqtgtraa --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px  --function="sign" --arguments 0x02 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - George;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px  --function="sign" --arguments 0x02 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### getActionSignerCount
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="getActionSignerCount" --arguments 0x01 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Candy sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="performAction" --arguments 2 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### propose transfer from Aadam - test newly add board member
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="proposeTransferExecute" --arguments erd1ajnfuwue94naxg2xq8gna0rusk8w8msmleavl23wyz4vhwxz0ckqtgtraa 3000000000000000000 0x01 0x01 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px  --function="sign" --arguments 0x03 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - George;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px  --function="sign" --arguments 0x03 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Aadam sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq6738xkmeptf6e34rlayvac2hxqnyajj20ckq59k4px --function="performAction" --arguments 3 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send
