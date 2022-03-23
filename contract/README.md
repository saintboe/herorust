HeroRust Smart Contract
==================
A HeroRust demo contract 


Play With Contract
===========
the contract is deployed on testnet with the name 

you can set it to environment for next step: herorust-alpha.herorust.testnet

```shell
export CONTRACTID = {YOU CONTRACT NAME}
```


## REVEAL 
```shell
# Attached 1 NEAR for reveal hero and return one type of hero (HUMAN INHUMAN or HERO)
near call $CONTRACTID reveal_hero '' --account_id player.testnet --deposit 1
# Return a Reward Pool
near view $CONTRACTID get_contract_pool ''
# Return a type of hero player got
near view $CONTRACTID get_hero_info '{"account_id":"player.testnet"}'
```
## RUMBLE
```shell
# Rumble sent hero for battle will return result eg.If return WIN can be runble again
near call $CONTRACTID hero_rumble '' --account_id player.testnet
```
## HISTORY
```shell
# Ruturn rumble history
near view $CONTRACTID get_rumble_history '{"from_index": 0, "limit": 100}' 
# Retun reveal history
near view $CONTRACTID get_revealhero_history '{"from_index": 0, "limit": 100}' 
```


Build Deploy and Init
======================
Please install Rust before compile contract

```shell
# build
cd contract
sh build.sh
```

```shell
# deploy a contract
near deploy --wasmFile res/herorust.wasm --accountId $CONTRACTID
```

```shell
#init contact for first time
near call $CONTRACTID new '' --account_id=$CONTRACTID
```

Exploring The Code
==================

1. The main smart contract code lives in `src/lib.rs`. 
2. Tests: You can run smart contract tests with the comand 'yarn test'

  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
