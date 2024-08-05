# Solidity API

## TaskMarket

Manage all proof tasks, player create new zk task, and miner can accept it,
when miner acceped, miner need submit the proof within overtime, if overflow, others
can accept and replace, and previous miner will be punished

### Task

Struct of ZK Task

```solidity
struct Task {
  enum TaskStatus status;
  address prover;
  address player;
  uint256 fee;
  address miner;
  uint256 overtime;
  bytes data;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### nextId

```solidity
uint256 nextId
```

Next task id

### CreateTask

```solidity
event CreateTask(uint256 id, address prover, address player, uint256 fee, bytes data)
```

Emit when created a new task

### AcceptTask

```solidity
event AcceptTask(uint256 id, address miner, uint256 overtime)
```

Emit when miner accepted a task

### SubmitTask

```solidity
event SubmitTask(uint256 id, bytes publics, bytes proof)
```

Emit when miner submit a proof for a task

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

### create

```solidity
function create(address prover, address player, uint256 fee, bytes data) external returns (uint256)
```

Create new zk task of a prover

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| player | address | the player account |
| fee | uint256 | the fee fot this task |
| data | bytes | the zk serialized inputs data |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the task id |

### accept

```solidity
function accept(uint256 tid, address miner) external
```

Accept a task by miner, can be called by miner or controller

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| tid | uint256 | the task id |
| miner | address | the miner account |

### submit

```solidity
function submit(uint256 tid, bytes publics, bytes proof) external
```

Submit a proof for a task, will call verifier to verify

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| tid | uint256 | the task id |
| publics | bytes | the zk serialized publics data |
| proof | bytes | the zk proof |

