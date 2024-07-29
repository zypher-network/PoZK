# Solidity API

## Epoch

Phases in the network, simulating "block height" in blockchain,
stake and reward are effective and issued according to the epoch

### addresses

```solidity
address addresses
```

Common Addresses contract

### period

```solidity
uint256 period
```

Period time in seconds

### startTime

```solidity
uint256 startTime
```

Current epoch start time

### now

```solidity
uint256 now
```

Current epoch height

### maintenance

```solidity
bool maintenance
```

Enter/esc maintenance mode, when entry maintenance mode, stake and reward will be stopped

### NewEpoch

```solidity
event NewEpoch(uint256 now, uint256 startTime)
```

Emitted when entry new epoch

### initialize

```solidity
function initialize(address _addresses, uint256 _period) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |
| _period | uint256 | the epoch period time in seconds |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract address |

### setMaintenance

```solidity
function setMaintenance(bool open) external
```

Set maintenance mode status

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| open | bool | open or false the maintenance mode |

### setPeriod

```solidity
function setPeriod(uint256 _period) external
```

Update period time

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _period | uint256 | the period time in seconds |

### getAndUpdate

```solidity
function getAndUpdate() external returns (uint256)
```

Update and get latest epoch height

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | latest epoch height |

### get

```solidity
function get() external view returns (uint256)
```

Get current epoch height

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | Current epoch height |

