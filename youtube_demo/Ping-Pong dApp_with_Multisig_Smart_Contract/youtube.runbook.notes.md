######
#
######
set elrond workspace in vscode
ctrl+shift+p = elrond workspace
######
#
######
git clone https://github.com/ElrondNetwork/ping-pong-smart-contract contract
cd contract/ping-pong
erdpy contract build

/home/anon/Desktop/elrond-dev/youtube_demos/explore_dapp/erdpy.json

{
    "configurations": {
        "default": {
            "proxy": "https://devnet-api.elrond.com",
            "chainID": "D"
        }
    },
    "contract":{
        "deploy":{
            "verbose": true,
            "bytecode": "output/ping-pong.wasm",
            "recall-nonce": true,
            "pem": "../../wallets/Aadam/Aadam.pem",
            "gas-limit": 59999999,
            "arguments": [1000000000000000000, 600],
            "send": true,
            "outfile": "deploy-testnet.interaction.json"
        }
     }
}

erdpy contract deploy

######
#
######
git clone https://github.com/ElrondNetwork/dapp-template dapp

/home/anon/Desktop/elrond-dev/youtube_demos/explore_dapp/dapp/src/config.tsx
### SC address:
erd1qqqqqqqqqqqqqpgqsp6l662tpt2nxky0g4s6jz855yz7kcmuzxpsl2gxee
erd1qqqqqqqqqqqqqpgqrkf0pchwcfv08dzx2q56m7qxrxlf0v6uzxpsp9ep74

### check ping on-click dapp webpage
/home/anon/Desktop/elrond-dev/youtube_demos/explore_dapp/dapp/src/pages/Dashboard/Actions/index.tsx

### add gas for pong button when testing

cd ../../dapp
npm install
npm run start

### MULTISIG ADDRESS:
/home/anon/Desktop/elrond-dev/youtube_demos/explore_dapp/dapp/src/config.tsx
erd1qqqqqqqqqqqqqpgqsjf3up6wlk4tuu8j8kdul5hnawv2cw5nzxps67cjxg


### Editing dApp dashboard buttons to peform different smart contract actions
/home/anon/Desktop/elrond-dev/youtube_demos/explore_dapp/dapp/src/pages/Dashboard/Actions/index.tsx
lines 85-91