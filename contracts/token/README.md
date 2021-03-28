Play Token Contract
==================

A token contract for gameland platform.

Play with this contract
========================
the contract is deployed at testnet with the name `playtoken.testnet`

you can set it to env for later use:
```shell
export GAMECOINID=playtoken.testnet
```

## Look around
```shell
# return info
near view $GAMECOINID get_contract_info ''

```
## Let's play
```shell

```

Build Deploy and Init
======================

Before you compile this code, you will need to install Rust with [correct target]


```shell
# building it
srouce ./build.sh
```

```shell
# deploy and init it
near deploy $GAMECOINID res/play_token.wasm --account_id=$GAMECOINID
near call $GAMECOINID new '{"owner_id": "humeng.testnet", "sudoer_id": "gameland.testnet"}' --account_id=$GAMECOINID

# adjust fee
near call $GAMECOINID update_fee_ratio '{"category": "shop_fee_play", "ratio": {"numerator": 0, "denominator": 1000}}' --account_id=humeng.testnet

near call $GAMECOINID update_fee_ratio \
  '{"category": "sudoer_fee_play", 
  "ratio": {"numerator": 0, "denominator": 1000}}' \
  --account_id=humeng.testnet

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
