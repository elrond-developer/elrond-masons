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
erdpy contract deploy

### MSC address
erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70

### fund SC
###
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send; 

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="deposit" --value=3000000000000000000 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send


####
### erdpy command
### ESDTTransfer WORKS
#### MSC create wrap egld

### unpayable version:
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  2000000000000000000 str:wrapEgld str:deposit --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send


### Payable version:
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  1000000000000000000 str:wrapEgld --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x02 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x02 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 2 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### re-run the above commands to get another egld to covert to equal amounts of mex

### Payable version:
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  1000000000000000000 str:wrapEgld --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x02 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x02 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 2 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

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
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 0 str:ESDTTransfer str:WEGLD-d7c6bb 1000000000000000000 str:swapTokensFixedInput str:MEX-dc289c 0x9daeadda246bd28317c7 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x03 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x03 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 3 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

#### personal
### erdpy command 
### LP PAIR WORKS
#### constructor

erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments 0x00000000000000000500e7283876b9cebf5e885a63795bc8271543a5acfb7ceb 0x02 0x5745474c442d643763366262 0x 0x0de0b6b3a7640000 0x4d45582d646332383963 0x 0x9fab2ed1405aa15810e5 0x6164644c6971756964697479 0x0dbd2fc137a30000 0x9e126e59661c48b0c90b --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### string personal LP wrap

erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 1000000000000000000 str:MEX-dc289c 0x 1124889168154083382251133 str:addLiquidity 1000000000000000000 1124889168154083382251133 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### another personal wallet example to deconstruct and reverse engineer
erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 1000000000000000000 str:MEX-dc289c 0x 1124889168154083382251133 str:addLiquidity 1000000000000000000 1124889168154083382251133 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### youtube live personal maiar example constructor
MultiESDTNFTTransfer@00000000000000000500e7283876b9cebf5e885a63795bc8271543a5acfb7ceb@02@5745474c442d643763366262@@0dd6523bb741342c@4d45582d646332383963@@ea3d5d8d9235648e08c6@6164644c6971756964697479@0db2e5e41424644a@e7e5b637ace80f11bbe2

## MSC example
MultiESDTNFTTransfer@00000000000000000500e7283876b9cebf5e885a63795bc8271543a5acfb7ceb@02@5745474c442d643763366262@@0de0b6b3a7640000@4d45582d646332383963@@176d6f43a6b3c00e2dad@6164644c6971756964697479@0dbd2fc137a30000@176d6f43a6b3c00e2dad

### MSC test
mex = 110633141645088782167469
110633141645088782167469

### test 4  --- This one works??? maybe slippage at the end???
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 0 str:MultiESDTNFTTransfer erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 997074783940195372 str:MEX-dc289c 0x 1106165749584968524105926 str:addLiquidity 987104036100793418 1095104092089118838864866 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

### using personal constructor below to make above arguments - focused on slippage ratio

MultiESDTNFTTransfer@00000000000000000500e7283876b9cebf5e885a63795bc8271543a5acfb7ceb@02@5745474c442d643763366262@@0dd6523bb741342c@4d45582d646332383963@@ea3d5d8d9235648e08c6@6164644c6971756964697479@0db2e5e41424644a@e7e5b637ace80f11bbe2

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x17 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x17 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 23 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### #####################################################
### STAKE LP
###### #####################################################

### Constructor personal example
erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments 0x00000000000000000500d386b6cd9e6d3840cfae558fb5f2496fdf0d69757ceb 0x01 0x45474c444d45582d633239623065 0x 0x0de0b6b3a7640000 0x656e7465724661726d --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### Constructor MSC template 
erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="MultiESDTNFTTransfer" --arguments 0x00000000000000000500d386b6cd9e6d3840cfae558fb5f2496fdf0d69757ceb 0x01 0x45474c444d45582d633239623065 0x 0x0de0b6b3a7640000 0x656e7465724661726d --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### Constructor MSC template string version
erd1qqqqqqqqqqqqqpgq6wrtdnv7d5uypnaw2k8mtujfdl0s66t40n4sag5e7n
1263810389063402734

### MSC WORKS!!!
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 0 str:MultiESDTNFTTransfer erd1qqqqqqqqqqqqqpgq6wrtdnv7d5uypnaw2k8mtujfdl0s66t40n4sag5e7n 1 str:EGLDMEX-c29b0e 0x 1263810389063402734 str:enterFarm --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x18 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x18 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 24 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

#### exit?

#### EXAMPLE
### erdpy command UNSTAKE (REMOVE) LP WORKS 
#### PERSONAL
erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="ESDTNFTTransfer" --arguments 0x45474c444d4558464c2d613332396236 0x1e 0x0de0b6b3a7640000 0x00000000000000000500d386b6cd9e6d3840cfae558fb5f2496fdf0d69757ceb 0x657869744661726d --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# STRING VERSION
#  0x2e i got from 46 from the explorer of transaction

erdpy --verbose contract call  erd1cm4fukzun6t96nfwv4wf8np7pc27egawjxfwnhlqr8qjdlf8k7hqpxa0c6 --function="ESDTNFTTransfer" --arguments str:EGLDMEXFL-a329b6 0x2e 1274480052340830777 erd1qqqqqqqqqqqqqpgq6wrtdnv7d5uypnaw2k8mtujfdl0s66t40n4sag5e7n str:exitFarm --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## MSC TEST -  WORKS!!!~~~
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 0 str:ESDTNFTTransfer str:EGLDMEXFL-a329b6 47 1263810389063402734 erd1qqqqqqqqqqqqqpgq6wrtdnv7d5uypnaw2k8mtujfdl0s66t40n4sag5e7n str:exitFarm --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

### look for this number on the identifier on the successful enter farm ID data
### https://devnet-explorer.elrond.com/transactions/d62fed10664c192bee776a5e0a877adb9843f77207c1d0aa05bcdc40710febdf#ec3aa345ee8656c76d248c2ddd545e8e408adb2993b2b8c7b3fb674e02de7d21
### 47

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x1d --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x1d --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 29 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### DECOUPLE (REMOVE) LP WORKS
#### PERSONAL
### erdpy WORKS
#### EXAMPLE

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy --function="ESDTTransfer" --arguments 0x45474c444d45582d633239623065 0x0de0b6b3a7640000 0x72656d6f76654c6971756964697479 0x0d24a3e9928cafea 0x973754cc14a0077750b3 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### CONVERT TO STRING
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy --function="ESDTTransfer" --arguments str:EGLDMEX-c29b0e
 1000000000000000000 str:removeLiquidity 947062045236047850 714098020118842440110259 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### MSC constructor
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 0 str:MultiESDTNFTTransfer erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 2 str:WEGLD-d7c6bb 0x 997074783940195372 str:MEX-dc289c 0x 1106165749584968524105926 str:addLiquidity 987104036100793418 1095104092089118838864866 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

### board member personal wallet example from maiar exchange
ESDTTransfer@45474c444d45582d633239623065@1189f4da579284ee@72656d6f76654c6971756964697479@0db2e68fd77b8352@e7e5aadc06ccaa0d111f

### Test 3 - WORKS!!!~~~ had to go on to figure out the maiar exchange rate i guess???
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 0 str:ESDTTransfer str:EGLDMEX-c29b0e 1251172285172768707 str:removeLiquidity 987104773817467730 1095103273658724951855391 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x26 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x26 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 38 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ###
### SWAP MEX -> EGLD
### SAMPLE
###### ###

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqd3e9rhvg2w3nk7h6smfcgugvp60wv08j3nlqxpx7a6 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 0 str:ESDTTransfer str:WEGLD-d7c6bb 1000000000000000000 str:swapTokensFixedInput str:MEX-dc289c 744633798644718756894663 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

### PERSONAL
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy --function="ESDTTransfer" --arguments 0x4d45582d646332383963 0x019e14f3dd8b94e83554 0x73776170546f6b656e7346697865644f7574707574 0x5745474c442d643763366262 0x2386f26fc10000 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### PERSONAL TO STRING
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy --function="ESDTTransfer" --arguments str:MEX-dc289c 7638461840407422055764 str:swapTokensFixedOutput str:WEGLD-d7c6bb 10000000000000000 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### THIS WON WORKS!!!~~~
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy 0 str:ESDTTransfer str:MEX-dc289c 1095268940524644250411428 str:swapTokensFixedInput str:WEGLD-d7c6bb 10000000000000000 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x27 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Patty;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x27 --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 39 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

######
### SWAP WEGLD --> EGLD
######
3974249711834405279

erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqd3e9rhvg2w3nk7h6smfcgugvp60wv08j3nlqxpx7a6 --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh  1000000000000000000 str:wrapEgld --chain="D" --pem="wallets/Sirius/Sirius.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### PERSONAL CONSTRUCTOR
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh --function="ESDTTransfer" --arguments 0x5745474c442d643763366262 0x0de0b6b3a7640000 0x756e7772617045676c64 --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

## PERSONAL TO STRING
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh --function="ESDTTransfer" --arguments str:WEGLD-d7c6bb 1000000000000000000 str:unwrapEgld --chain="D" --pem="/home/anon/Desktop/elrond-dev/maiar.erdpy.demo/test.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### test WORKS!!!~~~
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="proposeAsyncCall" --arguments erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh 0 str:ESDTTransfer str:WEGLD-d7c6bb 3974249711834405279 str:unwrapEgld --chain="D" --pem="wallets/Patty/Patty.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x28 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;

# SIGN - Aadam;
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="sign" --arguments 0x28 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# performAction; 
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrsat4xqpmtzt7g5wrk9fr7xglp4athq0zxps8u9r70 --function="performAction" --arguments 40 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send
