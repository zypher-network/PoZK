# Solidity API

## Stake

Stake Contract, including player, miner & game,
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

### GameStaking

Unit struct about staking in a game

```solidity
struct GameStaking {
  struct Stake.Staking gamerTotal;
  mapping(address => uint256) gamers;
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

### GameStakeChange

```solidity
event GameStakeChange(uint256 epoch, address game, address account, int256 changed, uint256 total)
```

Emit when game staking change

### MinerStakeChange

```solidity
event MinerStakeChange(uint256 epoch, address game, address account, int256 changed, uint256 total)
```

Emit when miner staking change

### PlayerStakeChange

```solidity
event PlayerStakeChange(uint256 epoch, address account, int256 changed, uint256 total)
```

Emit when player staking change

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

### gameTotalStaking

```solidity
function gameTotalStaking(address game) external view returns (uint256)
```

Get total game staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | total game staking amount |

### gameStaking

```solidity
function gameStaking(address game, address account) external view returns (uint256)
```

Get game staking by account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| account | address | the staking account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the staking amount of this account |

### gameStake

```solidity
function gameStake(address game, uint256 amount) external
```

Stake by game self(others)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| amount | uint256 | new staking amount |

### gameUnstake

```solidity
function gameUnstake(address game, uint256 amount) external
```

Unstake by game self(others)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| amount | uint256 | the unstaking amount |

### minerTotalStaking

```solidity
function minerTotalStaking(address game) external view returns (uint256)
```

Get total miner staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the total miner staking amount |

### minerStaking

```solidity
function minerStaking(address game, address account) public view returns (uint256)
```

Get miner staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| account | address | miner account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the miner staking amount |

### isMiner

```solidity
function isMiner(address game, address account) external view returns (bool)
```

Check account is miner or not

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| account | address | the checking account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | account is miner or not |

### minerStake

```solidity
function minerStake(address game, uint256 amount) external
```

Stake by miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| amount | uint256 | the new staking amount |

### minerUnStake

```solidity
function minerUnStake(address game, uint256 amount) external
```

Unstake by miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
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

