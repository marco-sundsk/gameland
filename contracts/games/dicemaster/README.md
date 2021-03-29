DiceMaster Smart Contract
==================

A dice game for gameland.  
three dices: 
| category | name | description | win-probobility | odds |
| - | - | - | - | - |
| 1 | big/small | [3 - 10] vs [11 - 36], except three identical dices | 48.61% | 1:1 |
| 2 | odd/even | odd vs even, except three identical dices | 48.61% | 1:1 |
| 3 | wei-tou | three identical dices and equal to player's guess | 0.46% | 1:150 |
| 4 | quan-tou | three identical dices | 2.80% | 1:24 |
| 5 | composition | player guesses two numbers of three dices | 13.90% | 1:5 |
| 6 | double-dice | player guesses one number that appears at least twice | 7.41% | 1:8 |


Play with this contract
========================
the contract is deployed at testnet with the name `dicemaster.testnet`

you can set it to env for later use:
```shell
export GAMEID=dicemaster.testnet
export GAS=100000000000000
export GLID=gameland.testnet
export COINID=playtoken.testnet
```

## Look around
```shell
# GAMELAND Standard View Interface
near view $GAMEID gl_metadata ''
near view $GAMEID gl_statistic ''
near view $GAMEID gl_pub_state ''
near view $GAMEID gl_user_state '{"user_id": "rb01.testnet"}'
# Game custom view functions
near view $GAMEID get_contract_info ''
near view $GAMEID get_win_history '{"from_index": 0, "limit": 100}'
```
## Let's play
```shell
# see how many playtoken we have
near view $COINID ft_balance_of '{"account_id": "rb01.testnet"}'
# purchase some if neccessary
near call $GLID buy_playtoken '' --account_id=rb01.testnet --amount=6
# play category 1, bet 1 coin with big, ie. set guess1 to 1 for big (2 for small)
near call $GAMEID gl_play \
  '{"amount": "1000000000000000000000000", 
  "op": "{\"category\": 1, \"guess1\": 1, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play category 2, bet 1 coin with odd, ie. set guess1 to 1 for odd (2 for even)
near call $GAMEID gl_play \
  '{"amount": "1000000000000000000000000", 
  "op": "{\"category\": 2, \"guess1\": 1, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play category 3, bet 1 coin for a hard 12, ie. set guess1 to 4 for 4-4-4.
near call $GAMEID gl_play \
  '{"amount": "1000000000000000000000000", 
  "op": "{\"category\": 3, \"guess1\": 4, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play category 4, bet 3 coins for three identical dice.
near call $GAMEID gl_play \
  '{"amount": "3000000000000000000000000", 
  "op": "{\"category\": 4, \"guess1\": 0, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play category 5, bet 6 coin with composition of 3-4
near call $GAMEID gl_play \
  '{"amount": "6000000000000000000000000", 
  "op": "{\"category\": 5, \"guess1\": 3, \"guess2\": 4, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play category 6, bet 8 coin for 3 appears at least twice. 
near call $GAMEID gl_play \
  '{"amount": "8000000000000000000000000", 
  "op": "{\"category\": 6, \"guess1\": 3, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
```

Build Deploy and Init
======================

Before you compile this code, you will need to install Rust with [correct target]


```shell
# building it
source ./build.sh
```

```shell
# deploy it
near deploy $GAMEID res/dicemaster.wasm --account_id=$GAMEID

# say it was deploy at $CONTRACTID, then init it, 
# set min bet is 1 coin and max bet is 10 coin
near call $GAMEID new \
  '{"owner_id": "humeng.testnet", 
  "min_bet": "1000000000000000000000000", 
  "max_bet": "10000000000000000000000000"}' \
  --account_id=$GAMEID

# sponsor jackpot with 200 token
near call $GAMEID gl_sponsor '{"amount": "200000000000000000000000000"}' \
  --account_id=rb01.testnet --gas=$GAS
```

Exploring The Code
==================

1. The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` script.
2. Tests: You can run smart contract tests with the `./test` script. This runs
   standard Rust tests using [cargo] with a `--nocapture` flag so that you
   can see any debug info you print to the console.


  [smart contract]: https://docs.near.org/docs/roles/developer/contracts/intro
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
