# Solidity API

## Vesting

Token lock status and unlock period

### MineReward

Unit struct about mine reward

```solidity
struct MineReward {
  uint256 value;
  uint256 newValue;
  uint256 newEpoch;
}
```

### MinerVesting

Unit struct about miner's vesting

```solidity
struct MinerVesting {
  uint256 amount;
  uint256 end;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### mineReward

```solidity
struct Vesting.MineReward mineReward
```

Rewards of every epoch will be released for mine & play

### miners

```solidity
mapping(address => struct Vesting.MinerVesting) miners
```

Store all miners vesting

### NewMineReward

```solidity
event NewMineReward(uint256 epoch, uint256 amount)
```

Emit when controller changed, isAdd if true is add, if false is remove

### initialize

```solidity
function initialize(address _addresses, uint256 _amount) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |
| _amount | uint256 | the mine reward for per epoch |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |

### setMineReward

```solidity
function setMineReward(uint256 amount) external
```

Set the mine amount pre epoch

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| amount | uint256 | new amount |

### approveForReward

```solidity
function approveForReward(uint256 amount) external
```

Approve enough token for reward

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| amount | uint256 | new amount |

### mine

```solidity
function mine(uint256 epoch) external view returns (uint256)
```

Get the mine amount of every epoch
epoch the epoch height

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the amount of reward |

### setMinerAmount

```solidity
function setMinerAmount(address[] _miners, uint256[] amounts, uint256[] ends) external
```

Batch set miner vesting amounts

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _miners | address[] | the miners list |
| amounts | uint256[] | the amounts list |
| ends | uint256[] | the ends list |

### minersTotal

```solidity
function minersTotal() external view returns (uint256)
```

Get all miners vesting amount

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | total amount of all miners |

### miner

```solidity
function miner(address account) external view returns (uint256)
```

Get miner vesting amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the miner account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the amount of this miner |

