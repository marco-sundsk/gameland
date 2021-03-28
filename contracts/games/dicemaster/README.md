DiceMaster Smart Contract
==================

A dice game for gameland.

Play with this contract
========================
the contract is deployed at testnet with the name `dicemaster.testnet`

you can set it to env for later use:
```shell
export GAMEID=dicemaster.testnet
export GAS=100000000000000
```

## Look around
```shell
# return playground info
near view $GAMEID gl_metadata ''
near view $GAMEID gl_statistic ''
near view $GAMEID gl_pub_state ''
near view $GAMEID gl_user_state '{"user_id": "rb01.testnet"}'

near view $GAMEID get_contract_info ''
near view $GAMEID get_win_history '{"from_index": 0, "limit": 100}'
```
## Let's play
```shell
# see how many playtoken we have
near view playtoken.testnet ft_balance_of '{"account_id": "rb01.testnet"}'
# purchase some if neccessary
near call gameland.testnet buy_playtoken '' --account_id=rb01.testnet --amount=6
# play, bet 1 coin with big
near call $GAMEID gl_play \
  '{"amount": "1000000000000000000000000", 
  "op": "{\"category\": 1, \"guess1\": 1, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play, bet 1 coin with odd
near call $GAMEID gl_play \
  '{"amount": "1000000000000000000000000", 
  "op": "{\"category\": 2, \"guess1\": 1, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play, bet 1 coin with weitou
near call $GAMEID gl_play \
  '{"amount": "1000000000000000000000000", 
  "op": "{\"category\": 3, \"guess1\": 3, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play, bet 3 coin with quantou
near call $GAMEID gl_play \
  '{"amount": "3000000000000000000000000", 
  "op": "{\"category\": 4, \"guess1\": 0, \"guess2\": 0, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play, bet 6 coin with composition
near call $GAMEID gl_play \
  '{"amount": "6000000000000000000000000", 
  "op": "{\"category\": 5, \"guess1\": 3, \"guess2\": 4, \"guess3\": 0}"}' \
  --account_id=rb01.testnet --gas=$GAS
# play, bet 8 coin with composition
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

# say it was deploy at $CONTRACTID, then init it 
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
