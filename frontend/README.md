# GAMELAND UI

### gameland description
---
get gameland meta info
```shell
ear view gameland.testnet metadata ''
# return:
View call: gameland.testnet.metadata()
{
  version: '0.1.0',
  name: 'Game Land',
  logo_url: 'https://github.com/',
  thumb_url: 'https://github.com/',
  description: 'A platform for small games'
}
```

get gameland policy info
```shell
near view playtoken.testnet get_contract_info ''
# return:
View call: playtoken.testnet.get_contract_info()
{
  owner: 'humeng.testnet',
  sudoer: 'gameland.testnet',
  total_supply: '467000000000000000000000000',
  total_collateral: '46233000000000000000000000',
  account_num: '7',
  shop_num: '1',
  sudoer_profit: '480000000000000000000000',
  sudoer_fee_play: { numerator: 5, denominator: 1000 },
  sudoer_fee_win: { numerator: 10, denominator: 1000 },
  shop_fee_play: { numerator: 5, denominator: 1000 },
  shop_fee_win: { numerator: 10, denominator: 1000 },
  mint_price: 100,
  burn_ratio: { numerator: 10, denominator: 1000 }
}
```

### shop description
---
get shop list and detail:
```shell
near view gameland.testnet list_shops '{"from_index": 0, "limit": 100}'
# return:
View call: gameland.testnet.list_shops({"from_index": 0, "limit": 100})
[
  {
    shop_id: 'neardice.testnet',
    owner_id: 'humeng.testnet',
    flag: 1,
    refs: 'https://github.com/marco-sundsk/gameland/',
    height: '40641470',
    ts: '1616077383477711863',
    status: 2
  },
  [length]: 1
]
```

### shop status
---

```shell
near view neardice.testnet gl_metadata ''
# return: 
View call: neardice.testnet.gl_metadata()
{
  version: '0.1.0',
  name: 'Dice Master',
  logo_url: 'https://github.com/',
  thumb_url: 'https://github.com/',
  description: "Let's Rolling Dices"
}

near view neardice.testnet gl_pub_state ''
# return:
View call: neardice.testnet.gl_pub_state()
'Current jackpot is 4896411132812500000000000'

near view neardice.testnet gl_user_state '{"user_id": "player01.testnet"}'
# return:
View call: neardice.testnet.gl_user_state({"user_id": "player01.testnet"})
''
```


### user balance
---

```shell
near view playtoken.testnet ft_balance_of '{"account_id": "rb01.testnet"}'
# return:
View call: playtoken.testnet.ft_balance_of({"account_id": "rb01.testnet"})
'1000000000000000000000000'
```

