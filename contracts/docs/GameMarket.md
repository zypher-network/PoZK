# Solidity API

## GameMarket

Manage all registered games

### GameWork

Unit struct for number change

```solidity
struct GameWork {
  uint256 value;
  uint256 newValue;
  uint256 newEpoch;
}
```

### GameVerifier

Unit struct for game verifier

```solidity
struct GameVerifier {
  address value;
  address newValue;
  uint256 newEpoch;
}
```

### Game

Game struct

```solidity
struct Game {
  enum GameStatus status;
  address owner;
  struct GameMarket.GameWork work;
  struct GameMarket.GameWork version;
  struct GameMarket.GameWork overtime;
  struct GameMarket.GameVerifier verifier;
  bool minable;
  string name;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### RegisterGame

```solidity
event RegisterGame(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string name)
```

Emit when new game register and waiting reviewing

### TransferGame

```solidity
event TransferGame(address game, address owner)
```

Emit when game owner transfer to others

### UpgradeGame

```solidity
event UpgradeGame(address game, uint256 work, uint256 version, address verifier, string name)
```

Emit when the game start upgrading and waiting reviewing, before approve, it will still use old info

### ApproveGame

```solidity
event ApproveGame(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string name, bool minable, bool approved)
```

Emit when the game is approved or reject

### StopGame

```solidity
event StopGame(address game)
```

Emit when the game is stopped

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

### register

```solidity
function register(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string name) external
```

Register new game to market, the sender is game owner, and waiting review

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game contract(or not) address (unique identifier) |
| work | uint256 | the game pozk work, calculation based on zk scheme and circuit size |
| version | uint256 | the game prover version |
| overtime | uint256 | the limit time when doing zkp, if overflow the time, others miner can accept the task again |
| verifier | address | the verifier contract, uses the IVerifier interface |
| name | string | the game name |

### unregister

```solidity
function unregister(address game) external
```

Game owner can unregister the game and cannot register same game address again

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |

### upgrade

```solidity
function upgrade(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string name) external
```

Game owner can upgrade the game to new verison and new info

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |
| work | uint256 | the game next pozk work, calculation based on zk scheme and circuit size |
| version | uint256 | the game next prover version |
| overtime | uint256 | the limit time when doing zkp, if overflow the time, others miner can accept the task again |
| verifier | address | the next verifier contract, uses the IVerifier interface |
| name | string | the game name, only name update immediately |

### transferGameOwner

```solidity
function transferGameOwner(address game, address owner) external
```

Game owner can transfer ownership to others

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |
| owner | address | the new owner account |

### approve

```solidity
function approve(address game, bool minable, bool approved) external
```

DAO can approve or reject the game register and upgrade, if approve, it will works in next epoch

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |
| minable | bool | if the game is minable, that means when create/accept the game task, will get reward from network |
| approved | bool | approve or reject |

### stop

```solidity
function stop(address game) external
```

DAO can stop a game

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |

### isGame

```solidity
function isGame(address game) external view returns (bool)
```

Check a game is working (working or upgrading)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | working or not |

### totalWork

```solidity
function totalWork() external view returns (uint256)
```

Get all games work

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the work of all games |

### work

```solidity
function work(address game) external view returns (uint256)
```

Get a game work

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the work of the game |

### version

```solidity
function version(address game) external view returns (uint256)
```

notice Get a game version

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the version of the game |

### overtime

```solidity
function overtime(address game) external view returns (uint256)
```

notice Get a game overtime

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the overtime of the game |

### verifier

```solidity
function verifier(address game) external view returns (address)
```

notice Get a game verifier

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | address | the verifier of the game |

