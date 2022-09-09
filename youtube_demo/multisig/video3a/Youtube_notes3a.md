### MSC contract payable - interacting with Maiar exchange
### https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqd3e9rhvg2w3nk7h6smfcgugvp60wv08j3nlqxpx7a6/tokens


##### #########################################################
# make SC payable
# 
###### ########################################################
### works payable - but use the erdpy.json instead
### constructor
erdpy --verbose contract deploy --project=multisig --chain="D" --pem="wallets/Aadam/Aadam.pem" --arguments 0x03 0xc0441134557a04fbb906950593defb97fc9528e2840ee0362e29545e99de8cfe 0x0e5450b01a90317698dd784c7846835039e324fb3f499e271a2b2adae6f53fe6 0x074ca9079502b8e19ed7836d256e47557630cce6731f199ae06ef09c54448bbe 0x520c755a1afcd62f11d59c8101deb171fb2f0f156489b9350b5d86156f0c14fd --metadata-payable --metadata-payable-by-sc --gas-limit=120000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

###### ########################################################
# erdpy.json example
# ensure this installed in the same directory as the SC i.e. multisig
###### ########################################################
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
            "bytecode": "output/multisig.wasm",
            "recall-nonce": true,
            "pem": "../wallets/Aadam/Aadam.pem",
            "gas-limit": 60000000,
            "arguments": [
                "0x03", 
                "erd1cpzpzdz40gz0hwgxj5ze8hhmjl7f228zss8wqd3w9929axw73nlqxadmhj",
                "erd1pe29pvq6jqchdxxa0px8s35r2qu7xf8m8ayeufc69v4d4eh48lnq2fgs0z",
                "erd1qax2jpu4q2uwr8khsdkj2mj824mrpn8xwv03nxhqdmcfc4zy3wlqve0kzd",
                "erd12gx82ks6lntz7yw4njqsrh43w8aj7rc4vjymjdgttkrp2mcvzn7st7uwnh"
            ],
            "send": true,
            "outfile": "multisig.json",
            "metadata-payable": true
        }
     }
}

###### ########################################################
# 
# 
###### ########################################################
cd multisig
### make sure erdpy.json is in the same folder as the MSC contract
erdpy verbose contract deploy

### MSC address
erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h

### fund SC
###
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send; 

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send


####
### erdpy command
### ESDTTransfer WORKS
#### MSC create wrap egld

### unpayable version:
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  1000000000000000000 str:wrapEgld str:deposit --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send


### Payable version:
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  1000000000000000000 str:wrapEgld --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x01 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x01 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="performAction" --arguments 1 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### re-run the above commands to get another egld to covert to equal amounts of mex

### Payable version:
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  1000000000000000000 str:wrapEgld --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x02 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x02 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="performAction" --arguments 2 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

####
### erdpy command
### SWAP WEGLD -> MEX
### STRING VERSION WORKS
####

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy --function="ESDTTransfer" --arguments str:WEGLD-d7c6bb 1000000000000000000 str:swapTokensFixedInput str:MEX-dc289c 0x9daeadda246bd28317c7 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqd3e9rhvg2w3nk7h6smfcgugvp60wv08j3nlqxpx7a6 --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  1000000000000000000 str:wrapEgld --chain="D" --pem="wallets/Sirius/Sirius.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## personal constructor
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy --function="ESDTTransfer" --arguments str:WEGLD-d7c6bb 1000000000000000000 str:swapTokensFixedInput str:MEX-dc289c 0x9daeadda246bd28317c7 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### this one works wEGLD -> Mex
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 0 str:ESDTTransfer str:WEGLD-d7c6bb 1000000000000000000 str:swapTokensFixedInput str:MEX-dc289c 0x9daeadda246bd28317c7 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x03 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x03 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="performAction" --arguments 3 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

#### personal
### erdpy command 
### LP PAIR WORKS
#### constructor

erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments 0x00000000000000000500e7283876b9cebf5e885a63795bc8271543a5acfb7ceb 0x02 0x5745474c442d643763366262 0x 0x0de0b6b3a7640000 0x4d45582d646332383963 0x 0x9fab2ed1405aa15810e5 0x6164644c6971756964697479 0x0dbd2fc137a30000 0x9e126e59661c48b0c90b --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### string personal LP wrap

erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 1000000000000000000 str:MEX-dc289c 0x 1124889168154083382251133 str:addLiquidity 1000000000000000000 1124889168154083382251133 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### MSC test
mex = 1105871417711142184255111

### test1 - failed
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 0 str:ESDTTransfer str:WEGLD-d7c6bb 1000000000000000000 str:swapTokensFixedInput str:MEX-dc289c 1105871417711142184255111 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

### test2 - failure
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h 0 str:MultiESDTNFTTransfer erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 1000000000000000000 str:MEX-dc289c 0x 1105871417711142184255111 str:addLiquidity 1000000000000000000 1124889168154083382251133 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

### another personal wallet example to deconstruct and reverse engineer
erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 1000000000000000000 str:MEX-dc289c 0x 1124889168154083382251133 str:addLiquidity 1000000000000000000 1124889168154083382251133 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### test3 - failed
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h 0 str:MultiESDTNFTTransfer erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 1000000000000000000 str:MEX-dc289c 0x 1105871417711142184255111 str:addLiquidity 1000000000000000000 1105871417711142184255111 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

### test 4  --- This one works??? maybe slippage at the end???
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h 0 str:MultiESDTNFTTransfer erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 1000000000000000000 str:MEX-dc289c 0x 1105871417711142184255111 str:addLiquidity 990000000000000000 1105871417711142184255111 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x09 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x09 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="performAction" --arguments 9 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### #####################################################
### STAKE LP
###### #####################################################

### Constructor personal example
erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments 0x00000000000000000500d386b6cd9e6d3840cfae558fb5f2496fdf0d69757ceb 0x01 0x45474c444d45582d633239623065 0x 0x0de0b6b3a7640000 0x656e7465724661726d --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### Constructor MSC template 
erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments 0x00000000000000000500d386b6cd9e6d3840cfae558fb5f2496fdf0d69757ceb 0x01 0x45474c444d45582d633239623065 0x 0x0de0b6b3a7640000 0x656e7465724661726d --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### Constructor MSC template string version
erd1qqqqqqqqqqqqqpgq6wrtdnv7d5uypnaw2k8mtujfdl0s66t40n4sag5e7n
1274480052340830777

### MSC WORKS!!!
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h 0 str:MultiESDTNFTTransfer erd1qqqqqqqqqqqqqpgq6wrtdnv7d5uypnaw2k8mtujfdl0s66t40n4sag5e7n 1 str:EGLDMEX-c29b0e 0x 126364293235493352 str:enterFarm --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x09 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="sign" --arguments 0x09 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqlvx9e24h87xj2gkh6f6x0m7lg3eha6cuzxps4p8e9h --function="performAction" --arguments 9 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send
