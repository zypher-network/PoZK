# PoZK contracts

Interaction with the contract.

## Miner
### front-end
- [stake/unstake](./Stake.md#minerstake)
- [claim unstaking](./Stake.md#claim)
- [collect reward](./Reward.md#minerbatchcollect)
- [set controller](./Controller.md#add)

### back-end (miner management)
- [download game docker image](./Prover.md#version)
- [accept task](./Task.md#accept)
- [submit task](./Task.md#submit)

## Player
- [stake/unstake](./Stake.md#playerstake)
- [claim unstaking](./Stake.md#claim)
- [collect reward](./Reward.md#playerbatchcollect)
- [set controller](./Controller.md#add)

## Game developer
- [game register](./Prover.md#register)
- [stake/unstake](./Stake.md#gamestake)
- [claim unstaking](./Stake.md#claim)
- [create task (integrated)](./Task.md#create)

## Subgraph
- [game list](./Prover.md#registergame)
- [staking list](./Stake.md#gamestakechange)
- [reward list](./Reward.md#minercollect)
