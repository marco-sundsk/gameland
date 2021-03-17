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
take positions
```

### buy & sell gamecoin
---
Platform is the only place that you can buy and/or sell gamecoins. Behind these buy and sell actions, it is actually the platform that mints and burns gamecoins for you.
```shell
take positions
```

You can check balance of anyone:
```shell
take positions
```

as well as the total supply:
```shell
take positions
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
take positions
```

Then, the platform governance committee will collect all registers time to time:
```shell
take positions
```

After carefully evaluation, the committee would make a pass to some register:
```shell
take positions
```

The last thing before a game shop can run publicly, may be to prepare initial reward pool of the game. That can be down through a sponsor action to a shop:
```shell
take positions
```
*note: The sponsor action may be required during register proccess*

### play game
---
User can learn information of a game in these ways:
```shell
take positions
```

And then play like this:
```shell
take positions
```
