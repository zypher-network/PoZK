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

### height

```solidity
uint256 height
```

Current epoch height

### maintenance

```solidity
bool maintenance
```

Enter/esc maintenance mode, when entry maintenance mode, stake and reward will be stopped

### dao

```solidity
mapping(address => bool) dao
```

the DAO accounts for the network (use for miner & prover cert)

### NewEpoch

```solidity
event NewEpoch(uint256 height, uint256 startTime)
```

Emitted when entry new epoch

### AddDao

```solidity
event AddDao(address account, bool ok)
```

Emitted when entry new DAO account

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

### setPeriod

```solidity
function setPeriod(uint256 _period) external
```

Update period time

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _period | uint256 | the period time in seconds |

### setMaintenance

```solidity
function setMaintenance(bool open) external
```

Set maintenance mode status

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| open | bool | open or false the maintenance mode |

### setNetworkMode

```solidity
function setNetworkMode(enum NetworkMode _mode) external
```

Set network mode

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _mode | enum NetworkMode | the network mode |

### addDao

```solidity
function addDao(address account, bool ok) external
```

Set network mode

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the new DAO account |
| ok | bool | the new status |

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

### networkMode

```solidity
function networkMode() external view returns (enum NetworkMode)
```

Get current network mode

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | enum NetworkMode | Current network mode |

### isDao

```solidity
function isDao(address account) external view returns (bool)
```

Check DAO account

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | Check result |

