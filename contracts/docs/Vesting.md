# Solidity API

## Vesting

Token lock status and unlock period

### addresses

```solidity
address addresses
```

Common Addresses contract

### rewardPerEpoch

```solidity
uint256 rewardPerEpoch
```

Rewards of every epoch will be released for mine & play

### miners

```solidity
mapping(address => uint256) miners
```

Store all miners vesting

### initialize

```solidity
function initialize(address _addresses, uint256 _rewardPerEpoch) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |
| _rewardPerEpoch | uint256 | the reward amount of every epoch |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |

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

