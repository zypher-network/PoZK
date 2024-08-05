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
mapping(address => uint256) miners
```

Store all miners vesting

### NewMineReward

```solidity
event NewMineReward(uint256 epoch, uint256 amount)
```

Emit when controller changed, isAdd if true is add, if false is remove

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

### setMineReward

```solidity
function setMineReward(uint256 amount) external
```

Set the mine amount pre epoch

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
function setMinerAmount(address[] _miners, uint256[] amounts) external
```

Batch set miner vesting amounts

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _miners | address[] | the miners list |
| amounts | uint256[] | the amounts list |

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

