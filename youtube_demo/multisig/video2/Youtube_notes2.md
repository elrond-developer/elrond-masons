### MSC - non-payable - interacting and sending coins and esdt tokens
### https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqdxgx4rcv3azy4uvxld0rkwphtfkrs9xt3nlq43sce4


###### ########################################################
# have MSC mint coins
# 
###### ########################################################

### CONSTRUCTOR extracted from web wallet mint esdt feature 
issue@44616e67@44414e47@115eec47f6cf7e35000000@12@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e4d696e74@74727565@63616e4275726e@74727565@63616e4368616e67654f776e6572@74727565@63616e55706772616465@74727565


Convert hexadecimal encoded string to string
Dang
44616e67

Convert hexadecimal encoded string to string
DANG
44414e47

Convert hexadecimal to decimal
21000000000000000000000000
115eec47f6cf7e35000000

Convert hexadecimal to decimal
18
12

Convert hexadecimal encoded string to string
canFreeze
63616e467265657a65

Convert hexadecimal encoded string to string
true
74727565

Convert hexadecimal encoded string to string
canWipe
63616e57697065

Convert hexadecimal encoded string to string
true
74727565

Convert hexadecimal encoded string to string
canPause
63616e5061757365

Convert hexadecimal encoded string to string
true
74727565

Convert hexadecimal encoded string to string
canMint
63616e4d696e74

Convert hexadecimal encoded string to string
true
74727565

Convert hexadecimal encoded string to string
canBurn
63616e4275726e

Convert hexadecimal encoded string to string
true
74727565

Convert hexadecimal encoded string to string
canChangeOwner
63616e4368616e67654f776e6572

Convert hexadecimal encoded string to string
true
74727565

Convert hexadecimal encoded string to string
canUpgrade
63616e55706772616465

Convert hexadecimal encoded string to string
true
74727565



issue
0x44616e67
0x44414e47
0x115eec47f6cf7e35000000
0x12
0x63616e467265657a65
0x74727565
0x63616e57697065
0x74727565
0x63616e5061757365
0x74727565
0x63616e4d696e74
0x74727565
0x63616e4275726e
0x74727565
0x63616e4368616e67654f776e6572
0x74727565
0x63616e55706772616465
0x74727565

## personal wallet working command as reference
erdpy --verbose contract call erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u --function="issue" --arguments str:ulog str:ULOG 21000000000000000000000000 18 str:canFreeze str:true str:canWipe str:true str:canPause str:true str:canMint str:true str:canBurn str:true str:canChangeOwner str:true str:canUpgrade str:true --value=50000000000000000 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### 1st hex version
DANG-f25bb3

### 2nd human readable version
ULOG-911e7e

### MSC attempt

###### ##########################################################
### proposeTransferExecute - SCM mint tokens works~~
###### ##########################################################

### Convert from Bech32 address to Hex address 
erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u = 000000000000000000010000000000000000000000000000000000000002ffff

### THIS WON WORKS! ~~~ mint mint mint away -- hex version
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="proposeTransferExecute" --arguments 0x000000000000000000010000000000000000000000000000000000000002ffff 50000000000000000 0x6973737565 0x44616e67 0x44414e47 0x115eec47f6cf7e35000000 0x12 0x63616e467265657a65 0x74727565 0x63616e57697065 0x74727565 0x63616e5061757365 0x74727565 0x63616e4d696e74 0x74727565 0x63616e4275726e 0x74727565 0x63616e4368616e67654f776e6572 0x74727565 0x63616e55706772616465 0x74727565 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

### human readable version

erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="proposeTransferExecute" --arguments erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u 50000000000000000 str:issue str:ulog str:ULOG 21000000000000000000000000 18 str:canFreeze str:true str:canWipe str:true str:canPause str:true str:canMint str:true str:canBurn str:true str:canChangeOwner str:true str:canUpgrade str:true --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x05 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;


# SIGN - George;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x05 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Candy sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="performAction" --arguments 5 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ########################################################
# MSC send tokens
# example: https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqk7y2h43dfltcn647mf8vfg80nkmlcpa4ggysauzfd8
### https://devnet-explorer.elrond.com/transactions/2512f51786f84ba094880b33cec8416bacbf9d40d8d48408127754395541ff06#smart
#
### from MSC -> board member personal wallet
###### ########################################################

### Send to Board member personal wallet
erd1z4y3lten76yz6syk4mk3xla3ag4zw5nv7yatlrq9gn80925qzxps4sx50m
0x15491faf33f6882d4096aeed137fb1ea2a27526cf13abf8c0544cef2aa801183

proposeAsyncCall@26d39bfadd7c08a3e55b970e3e625f011fdad454f6abd49bdcf9c0fd55b50fc5@@455344545472616e73666572@313233342d616663663165@8ac7230489e80000


### example from community ###### WORKS !!
### https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqk7y2h43dfltcn647mf8vfg80nkmlcpa4ggysauzfd8
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="proposeAsyncCall" --arguments erd1z4y3lten76yz6syk4mk3xla3ag4zw5nv7yatlrq9gn80925qzxps4sx50m 0 str:ESDTTransfer str:DANG-91e9ab 1000000000000000000000000  --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send

# SIGN - Candy;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x06 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send;


# SIGN - George;
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf  --function="sign" --arguments 0x06 --chain="D" --pem="wallets/George/George.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

## performAction - Candy sends it off
erdpy --verbose contract call  erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="performAction" --arguments 6 --chain="D" --pem="wallets/Candy/Candy.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send

###### ########################################################
# SENDING ESDT TOKENS FROM PERSONAL -> MSC
# 
### from MSC -> board member personal wallet
###### ########################################################
ESDTTransfer@44414e472d663235626233@d3c21bcecceda1000000

#### this works but the MSC has to be payable
# https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqk7y2h43dfltcn647mf8vfg80nkmlcpa4ggysauzfd8

# https://devnet-explorer.elrond.com/transactions/2512f51786f84ba094880b33cec8416bacbf9d40d8d48408127754395541ff06#text
erd1qqqqqqqqqqqqqpgqk7y2h43dfltcn647mf8vfg80nkmlcpa4ggysauzfd8

### this one works SENDING ESDT TOKENS FROM PERSONAL -> MSC
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="ESDTTransfer" --arguments str:DANG-f25bb3 1000000000000000000000000 str:deposit --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### test a failure (missing deposit)
### this one works SENDING ESDT TOKENS FROM PERSONAL -> MSC
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="ESDTTransfer" --arguments str:ULOG-911e7e 1000000000000000000000000 --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### try again and it works
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgqrglfe8k475aaj2uxcl5p8jjwjumkh58xzxps5h3klf --function="ESDTTransfer" --arguments str:ULOG-911e7e 1000000000000000000000000 str:deposit --chain="D" --pem="wallets/Aadam/Aadam.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com"  --recall-nonce --send 

### demo sneak peak at payable SC
erdpy contract -h
erdpy contract deploy -h
