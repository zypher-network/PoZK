# Solidity API

## Addresses

Store all contracts address and can update them

### initialize

```solidity
function initialize() public
```

Initialize

### set

```solidity
function set(enum Contracts c, address _address) external
```

Owner can update some contract address

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| c | enum Contracts | the Contract enum |
| _address | address | the contract new address |

### batchSet

```solidity
function batchSet(enum Contracts[] _cs, address[] _addresses) external
```

Owner can batch update contracts' address

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _cs | enum Contracts[] | the Contracts list |
| _addresses | address[] | the contracts new addresses |

### get

```solidity
function get(enum Contracts c) external view returns (address)
```

Get contract address

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| c | enum Contracts | the Contract enum |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | address | the contract address |

