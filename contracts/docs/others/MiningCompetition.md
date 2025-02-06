# Solidity API

## Status

```solidity
enum Status {
  Waiting,
  Working,
  Stopped
}
```

## MiningCompetition

Phases in the network, simulating "block height" in blockchain,
stake and reward are effective and issued according to the epoch

### addresses

```solidity
address addresses
```

Common Addresses contract

### status

```solidity
enum Status status
```

Competition status

### initProver

```solidity
address initProver
```

### registerReward

```solidity
uint256 registerReward
```

### inviteReward

```solidity
uint256 inviteReward
```

### realToken

```solidity
address realToken
```

### decimal

```solidity
uint256 decimal
```

### allowlist

```solidity
mapping(address => bool) allowlist
```

admin list for register account

### users

```solidity
mapping(address => uint256) users
```

user => user invited rewards & invite link is user's address

### Register

```solidity
event Register(address user, address inviter, uint256 inviterTotal)
```

### Exchange

```solidity
event Exchange(address user, uint256 amount, uint256 realAmount)
```

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
| _addresses | address | the Addresses contract address |

### changeStatus

```solidity
function changeStatus(enum Status _status, address _initProver, uint256 _registerReward, uint256 _inviteReward) external
```

### setExchange

```solidity
function setExchange(address _realToken, uint256 _decimal) external
```

### allow

```solidity
function allow(address account, bool _allow) external
```

### register

```solidity
function register(address account, address inviter) external
```

### exchange

```solidity
function exchange(address account, uint256 amount) external
```

