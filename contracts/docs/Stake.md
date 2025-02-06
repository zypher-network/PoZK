# Solidity API

## Stake

Stake Contract, including player, miner & prover,
every change will work in next epoch, and unstake can claim in next epoch

### Staking

Unit struct about staking/unstaking

```solidity
struct Staking {
  uint256 value;
  uint256 newValue;
  uint256 newEpoch;
}
```

### ProverStaking

Unit struct about staking in a prover

```solidity
struct ProverStaking {
  struct Stake.Staking proverTotal;
  mapping(address => struct Stake.Staking) provers;
  struct Stake.Staking minerTotal;
  mapping(address => struct Stake.Staking) miners;
}
```

### ZkTest

Unit struct about ZK test

```solidity
struct ZkTest {
  address payer;
  address miner;
  address prover;
  uint256 amount;
  uint256 overAt;
  bytes publics;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### minStakeAmount

```solidity
uint256 minStakeAmount
```

Miner minStakeAmount

### allowlist

```solidity
mapping(address => uint256) allowlist
```

Store all miner allowlist

### ProverStakeChange

```solidity
event ProverStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 staking, uint256 total)
```

Emit when prover staking change

### MinerStakeChange

```solidity
event MinerStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 staking, uint256 total)
```

Emit when miner staking change

### PlayerStakeChange

```solidity
event PlayerStakeChange(uint256 epoch, address account, int256 changed, uint256 staking, uint256 total)
```

Emit when player staking change

### AddUnstaking

```solidity
event AddUnstaking(uint256 epoch, address account, uint256 amount)
```

Emit when account add unstaking/reward

### ClaimUnstaking

```solidity
event ClaimUnstaking(address account, uint256 amount)
```

Emit when account claimed the unstaking

### AddAllowlist

```solidity
event AddAllowlist(address[] accounts, uint256[] amounts)
```

Emit when add new account to miner allowlist

### SubAllowlist

```solidity
event SubAllowlist(address account, uint256 amount)
```

Emit when add account used the allowlist amounts

### MinerTestRequire

```solidity
event MinerTestRequire(uint256 id, address account, address prover, uint256 amount)
```

Emit when miner need do a test

### MinerTestCreate

```solidity
event MinerTestCreate(uint256 id, address account, address prover, uint256 overtime, bytes inputs, bytes publics)
```

Emit when test have been created and start

### MinerTestSubmit

```solidity
event MinerTestSubmit(uint256 id, uint256 submitAt)
```

Emit when pass the miner test

### MinerTestCancel

```solidity
event MinerTestCancel(uint256 id)
```

Emit when cancel the miner test

### initialize

```solidity
function initialize(address _addresses, uint256 _minStakeAmount) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |
| _minStakeAmount | uint256 | the minimum value of miner staking |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |

### setMinStakeAmount

```solidity
function setMinStakeAmount(uint256 _minStakeAmount) external
```

Set minimum stake amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _minStakeAmount | uint256 | the minimum value of miner staking |

### addAllowlist

```solidity
function addAllowlist(address[] accounts, uint256[] amounts) external
```

Add allowlist accounts (multiple)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| accounts | address[] | the accounts |
| amounts | uint256[] | the true or false |

### proverTotalStaking

```solidity
function proverTotalStaking(address prover) external view returns (uint256)
```

Get total prover staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | total prover staking amount |

### proverStaking

```solidity
function proverStaking(address prover, address account) external view returns (uint256)
```

Get prover staking by account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| account | address | the staking account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the staking amount of this account |

### proverStake

```solidity
function proverStake(address prover, uint256 amount) external
```

Stake by prover self(others)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| amount | uint256 | new staking amount |

### proverUnstake

```solidity
function proverUnstake(address prover, uint256 amount) external
```

Unstake by prover self(others)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| amount | uint256 | the unstaking amount |

### minerTotalStaking

```solidity
function minerTotalStaking(address prover) external view returns (uint256)
```

Get total miner staking (no vesting)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the total miner staking amount |

### minerStaking

```solidity
function minerStaking(address prover, address account) public view returns (uint256)
```

Get miner staking (with vesting)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| account | address | miner account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the miner staking amount |

### isMiner

```solidity
function isMiner(address prover, address account) external view returns (bool)
```

Check account is miner or not

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| account | address | the checking account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | account is miner or not |

### minerStake

```solidity
function minerStake(address prover, uint256 amount) external
```

Stake by miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| amount | uint256 | the new staking amount |

### minerStakeFor

```solidity
function minerStakeFor(address miner, address prover, uint256 amount) public
```

Stake by someone for the miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| miner | address | the miner address |
| prover | address | the prover address |
| amount | uint256 | the new staking amount |

### minerTest

```solidity
function minerTest(uint256 id, bytes inputs, bytes publics) external
```

DAO create the test

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | the test id |
| inputs | bytes | the zk input data |
| publics | bytes | the zk public inputs |

### minerTestSubmit

```solidity
function minerTestSubmit(uint256 id, bool autoNew, bytes proof) external
```

Miner submit the proof of the test

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | the test id |
| autoNew | bool | auto renew the task if over time |
| proof | bytes | the zk proof |

### minerTestCancel

```solidity
function minerTestCancel(uint256 id) external
```

Miner cancel the proof of the test

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | the test id |

### minerUnstake

```solidity
function minerUnstake(address prover, uint256 amount) external
```

Unstake by miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| amount | uint256 | the unstaking amount |

### minerTransferStaking

```solidity
function minerTransferStaking(address from, address to, uint256 amount) external
```

Miner can transfer staking from one prover to another without unclaim lock time

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| from | address | the from prover address |
| to | address | the to prover address |
| amount | uint256 | the staking amount |

### minerSlashStaking

```solidity
function minerSlashStaking(address miner, address prover, address player, uint256 amount) external
```

get miner staking

### playerTotalStaking

```solidity
function playerTotalStaking() external view returns (uint256)
```

Get total player staking

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the total staking amount of players |

### playerStaking

```solidity
function playerStaking(address account) external view returns (uint256)
```

Get player staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the player account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the staking amount of player |

### playerStake

```solidity
function playerStake(uint256 amount) external
```

Stake by player

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| amount | uint256 | the new staking amount of player |

### playerStakeFor

```solidity
function playerStakeFor(address player, uint256 amount) public
```

Stake by player

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| player | address | the player address |
| amount | uint256 | the new staking amount of player |

### playerUnstake

```solidity
function playerUnstake(uint256 amount) external
```

Unstake by player

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| amount | uint256 | the unstaking amount |

### addUnstaking

```solidity
function addUnstaking(address account, uint256 amount) external
```

Add new unstaking to next epoch, only this contract and reward contract.

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the unstaking account |
| amount | uint256 | the unstaking amount |

### claimable

```solidity
function claimable(address account) external view returns (uint256)
```

Get claimable unstaking amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the claiming account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the amount which can claim now |

### claim

```solidity
function claim(address account) external
```

Claim unstaking to account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the claiming account |

