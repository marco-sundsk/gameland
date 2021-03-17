# GAMELAND Contracts
In GAMELAND, we have several types of contracts:
* Platform contract, say gameland.testnet;
* Gamecoin contract, say playtoken.testnet;
* Game contracts, say neardice.test is one of them;
* Governance token contract, say gltoken.testnet;

### pre-requisite
---
Each contract has been successfully deployed and init correctly. The init processes are like:
```shell
# init gamecoin
near call playtoken.testnet new '{"owner_id": "playtoken_owner.testnet", "sudoer_id": "gameland.testnet"}' --account_id=playtoken.testnet
# init platform
near call gameland.testnet new '{"owner_id": "gameland_owner.testnet"}' --account_id=gameland.testnet
# init game shop
near call neardice.testnet new '{"owner_id": "neardice_owner.testnet", "dice_number": 1, "rolling_fee": "1000000000000000000000000"}' --account_id=neardice.testnet
```


### buy & sell gamecoin
---
Platform is the only place that you can buy and/or sell gamecoins. Behind these buy and sell actions, it is actually the platform that mints and burns gamecoins for you.
```shell
near call gameland.testnet buy_playtoken '' --account_id=player01.testnet --amount=1

near call gameland.testnet sell_playtoken '{"amount": "6000000000000000000000000"}' --account_id=player01.testnet
```

You can check balance of anyone:
```shell
near view playtoken.testnet ft_balance_of '{"account_id": "player01.testnet"}'
```

as well as the total supply:
```shell
near view playtoken.testnet ft_total_supply ''
```

### register games
---
The whole register process has some off-line parts, which can be upgraded to online-governance using governance token in the near future.  

Frist, The game owner fills out register form 
* shop_id (each game acts as a game shop from platform view)
* owner_id
* flag, 1 for reg, 2 for unreg
* refs, url to an online application form that have detailed info, such as game description, code repo, contactors, and etc.


and submit like this:
```shell
near call gameland.testnet register_shop '{"reg_form": {"flag": 1, "shop_id": "neardice.testnet", "owner_id": "neardice_owner.testnet", "refs": "https://github.com/marco-sundsk/gameland/", "height": "0", "ts": "0", "status": 0}}' --account_id=neardice_onwer.testnet
```

Then, the platform governance committee will collect all registers time to time:
```shell
near view gameland.testnet list_registers '{"from_index": 0, "limit": 100}'
```

After carefully evaluation, the committee would make a pass to some register:
```shell
near call gameland.testnet resovle_register '{"shop_id": "neardice.testnet", "pass": true, "new_status": 2}' --account_id=gameland_owner.testnet --gas=20000000000000
```

The last thing before a game shop can run publicly, may be to prepare initial reward pool of the game. That can be down through a sponsor action to a shop:
```shell
near call gameland.testnet sponsor '{"shop_id": "neardice.testnet", "amount": "2000000000000000000000000"}' --account_id=player01.testnet --gas=40000000000000
```
*note: The sponsor action may be required during register proccess*

### play game
---
User can learn information of a game in these ways:
```shell
near view neardice.testnet gl_metadata ''
near view neardice.testnet gl_pub_state ''
near view neardice.testnet gl_user_state '{"user_id": "player01.testnet"}'
```

And then play like this:
```shell
near call gameland.testnet play '{"shop_id": "neardice.testnet", "amount": "1000000000000000000000000", "op": "1"}' --account_id=player01.testnet --gas=60000000000000
```
