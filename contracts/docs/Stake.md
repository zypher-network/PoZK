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

### ProverStakeChange

```solidity
event ProverStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 total)
```

Emit when prover staking change

### MinerStakeChange

```solidity
event MinerStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 total)
```

Emit when miner staking change

### PlayerStakeChange

```solidity
event PlayerStakeChange(uint256 epoch, address account, int256 changed, uint256 total)
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

### setMinStakeAmount

```solidity
function setMinStakeAmount(uint256 _minStakeAmount) external
```

Set minimum stake amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _minStakeAmount | uint256 | the minimum value of miner staking |

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

Get total miner staking

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

Get miner staking

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

### minerUnStake

```solidity
function minerUnStake(address prover, uint256 amount) external
```

Unstake by miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| amount | uint256 | the unstaking amount |

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

### playerUnStake

```solidity
function playerUnStake(uint256 amount) external
```

Unstake by player

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| amount | uint256 | the unstaking amount |

### addUnstaking

```solidity
function addUnstaking(address account, uint256 amount) public
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

