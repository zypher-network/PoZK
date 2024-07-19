# Solidity API

## Controller

User can set multiple controllers to help them with some functions,

### addresses

```solidity
address addresses
```

Common Addresses contract

### controllers

```solidity
mapping(address => mapping(address => bool)) controllers
```

Store all controllers by account

### ChangeController

```solidity
event ChangeController(address account, address controller, bool isAdd)
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
| _addresses | address | the common Addresses contract |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set common Addresses contract

### check

```solidity
function check(address account, address controller) external view returns (bool)
```

Check if controller belongs to account or account self

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the account address |
| controller | address | the controller address |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | success or failure |

### add

```solidity
function add(address controller) external
```

Add new controller to account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| controller | address | the controller address |

### remove

```solidity
function remove(address controller) external
```

Remove a controller from account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| controller | address | the controller address |

