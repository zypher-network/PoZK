# Solidity API

## Reward

Reward Contract for reward distribute and claim, miner & player can get reward,
when they play game and prove zkp, all of them can get reward,
use cobb-douglas function for work and labor

### StakerLabor

The unit struct of stake and labor

```solidity
struct StakerLabor {
  uint256 staking;
  uint256 working;
}
```

### ProverPool

The struct of prover pool

```solidity
struct ProverPool {
  uint256 totalWorking;
  uint256 totalStaking;
  uint256 totalMinerStaking;
  uint256 totalPlayerStaking;
  uint256 totalMinerReward;
  uint256 totalPlayerReward;
  uint256 unclaimReward;
  uint256 unclaimLabor;
  mapping(address => struct Reward.StakerLabor) minerLabor;
  mapping(address => struct Reward.StakerLabor) playerLabor;
}
```

### RunningProver

The struct of a prover status

```solidity
struct RunningProver {
  uint256 unclaim;
  address[] provers;
  mapping(address => uint256) index;
}
```

### EpochPool

The struct of the epoch status

```solidity
struct EpochPool {
  uint256 unclaim;
  uint256 totalProverStaking;
  mapping(address => struct Reward.ProverPool) proverPools;
  mapping(address => struct Reward.RunningProver) minerUnclaimedProvers;
  mapping(address => struct Reward.RunningProver) playerUnclaimedProvers;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### alphaNumerator

```solidity
int256 alphaNumerator
```

The numerator of Percentage of the prover stake and labor (1-alpha) in the total

### alphaDenominator

```solidity
int256 alphaDenominator
```

The denominator of the alpha

### betaNumerator

```solidity
int256 betaNumerator
```

The numerator of Percentage of the miner stake and labor (1-beta) in the total

### betaDenominator

```solidity
int256 betaDenominator
```

The denominator of the beta

### gammaNumerator

```solidity
int256 gammaNumerator
```

The numerator of Percentage of the player stake and labor (1-beta) in the total

### gammaDenominator

```solidity
int256 gammaDenominator
```

The denominator of the gamma

### minerMaxPer

```solidity
uint256 minerMaxPer
```

The miner max percent of reward

### minerMinPer

```solidity
uint256 minerMinPer
```

The miner min percent of reward

### playerMaxNum

```solidity
uint256 playerMaxNum
```

The player max games number when reach minerMaxPer

### Alpha

```solidity
event Alpha(int256 alphaNumerator, int256 alphaDenominator)
```

Emitted when update the alpha for cobb-douglas function

### Beta

```solidity
event Beta(int256 betaNumerator, int256 betaDenominator)
```

Emitted when update the alpha for cobb-douglas function

### Gamma

```solidity
event Gamma(int256 gammaNumerator, int256 gammaDenominator)
```

Emitted when update the alpha for cobb-douglas function

### MinerPlayerPer

```solidity
event MinerPlayerPer(uint256 minerMaxPer, uint256 minerMinPer, uint256 playerMaxNum)
```

Emitted when update the percent of miner and player

### MinerLabor

```solidity
event MinerLabor(uint256 epoch, address prover, address miner, uint256 work)
```

Emitted when add Labor(reward) for current pool

### PlayerLabor

```solidity
event PlayerLabor(uint256 epoch, address prover, address player, uint256 play)
```

Emitted when add Labor(reward) for current pool

### MinerCollect

```solidity
event MinerCollect(uint256 epoch, address prover, address miner, uint256 amount)
```

Emitted when collect reward (stake) from pool

### PlayerCollect

```solidity
event PlayerCollect(uint256 epoch, address prover, address player, uint256 amount)
```

Emitted when collect reward (stake) from pool

### initialize

```solidity
function initialize(address _addresses) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |

### setAlpha

```solidity
function setAlpha(int256 _alphaNumerator, int256 _alphaDenominator) public
```

Update the alpha for cobb-douglas function

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _alphaNumerator | int256 | the numerator of the alpha |
| _alphaDenominator | int256 | the denominator of the alpha |

### setBeta

```solidity
function setBeta(int256 _betaNumerator, int256 _betaDenominator) public
```

Update the beta for cobb-douglas function

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _betaNumerator | int256 | the numerator of the beta |
| _betaDenominator | int256 | the denominator of the beta |

### setGamma

```solidity
function setGamma(int256 _gammaNumerator, int256 _gammaDenominator) public
```

Update the gamma for cobb-douglas function

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _gammaNumerator | int256 | the numerator of the gamma |
| _gammaDenominator | int256 | the denominator of the gamma |

### setMinerPlayerPer

```solidity
function setMinerPlayerPer(uint256 _minerMaxPer, uint256 _minerMinPer, uint256 _playerMaxNum) public
```

Update the miner and player reward percent

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _minerMaxPer | uint256 | The miner max percent of reward |
| _minerMinPer | uint256 | The miner min percent of reward |
| _playerMaxNum | uint256 | The player max games number when reach minerMaxPer |

### work

```solidity
function work(address prover, address player, address miner) external
```

Add work(labor) to current epoch & prover, only call from TaskMarket

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| player | address | player account |
| miner | address | miner account |

### minerCollect

```solidity
function minerCollect(uint256 epoch, address prover, address miner) public
```

Miner collect reward in epoch and prover, collect reward to unstaking list

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| prover | address | the prover address |
| miner | address | the miner account |

### playerCollect

```solidity
function playerCollect(uint256 epoch, address prover, address player) public
```

Player collect reward in epoch and prover, collect to player wallet

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| prover | address | the prover address |
| player | address | the player account |

### minerBatchCollect

```solidity
function minerBatchCollect(uint256 epoch, address miner) external
```

Miner batch collect all provers in a epoch

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| miner | address | the miner account |

### playerBatchCollect

```solidity
function playerBatchCollect(uint256 epoch, address player) external
```

Player batch collect all provers in a epoch

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| player | address | the player account |

