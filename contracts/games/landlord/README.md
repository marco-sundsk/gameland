GameLand Game Smart Contract
==================

Game AngleLand.

Play with this contract
========================
the contract is deployed at testnet with the name `angleland.testnet`

you can set it to env for later use:
```shell
export CONTRACTID=angleland.testnet
```

## Look around
```shell
# return info
near view $CONTRACTID get_contract_info ''
# return current round game info
near view $CONTRACTID get_market_info ''
# return win history list
near view $CONTRACTID get_win_history '{"from_index": 0, "limit": 100}'
# return metadata
near view $CONTRACTID gl_metadata ''
near view $CONTRACTID gl_pub_state ''
near view $CONTRACTID gl_user_state '{"user_id": "rb01.testnet"}'
```
## Let's play
```shell
# see how many playtoken we have
near view playtoken.testnet ft_balance_of '{"account_id": "rb01.testnet"}'
# purchase some if neccessary
near call gameland.testnet buy_playtoken '' --account_id=rb01.testnet --amount=6
# play the game
near call angleland.testnet gl_play '{"amount": "1000000000000000000000000", "op": "1"}' --account_id=rb01.testnet --gas=50000000000000

```

Build Deploy and Init
======================

Before you compile this code, you will need to install Rust with [correct target]


```shell
# building it
srouce ./build.sh
```

```shell
near deploy angleland.testnet res/gl_landlord.wasm --account_id=angleland.testnet
near call angleland.testnet new '{"owner_id": "humeng.testnet", "house_count": 36, "play_fee": "1000000000000000000000000"}' --account_id=angleland.testnet

### register this game
near call gameland.testnet register_shop '{"reg_form": {"flag": 1, "shop_id": "angleland.testnet", "owner_id": "humeng.testnet", "refs": "https://github.com/marco-sundsk/gameland/", "height": "0", "ts": "0", "status": 0}}' --account_id=humeng.testnet
near view gameland.testnet list_registers '{"from_index": 0, "limit": 100}'
near call gameland.testnet resovle_register '{"shop_id": "angleland.testnet", "pass": true, "new_status": 2}' --account_id=humeng.testnet --gas=20000000000000

### sponsor this game with 5 tokensgit 
near call angleland.testnet gl_sponsor '{"amount": "5000000000000000000000000"}' --account_id=humeng.testnet --gas=30000000000000
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
