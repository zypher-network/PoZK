# Solidity API

## Task

Manage all proof tasks, player create new zk task, and miner can accept it,
when miner acceped, miner need submit the proof within overtime, if overflow, others
can accept and replace, and previous miner will be punished

### GameTask

Struct of ZK Task

```solidity
struct GameTask {
  enum TaskStatus status;
  address prover;
  address player;
  uint256 fee;
  address miner;
  string url;
  uint256 overtime;
  bytes publics;
  uint256 dispute;
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

Next task id, start from 1

### disputeDeposit

```solidity
uint256 disputeDeposit
```

the deposit for security when dispute

### tasks

```solidity
mapping(uint256 => struct Task.GameTask) tasks
```

Store all tasks

### tasksResults

```solidity
mapping(bytes32 => uint256) tasksResults
```

Store all tasks results

### proxyList

```solidity
mapping(address => bool) proxyList
```

Store all proxy allow list

### CreateTask

```solidity
event CreateTask(uint256 id, address prover, address player, uint256 fee, bytes inputs, bytes publics)
```

Emit when created a new task

### AcceptTask

```solidity
event AcceptTask(uint256 id, address miner, uint256 overtime, string url)
```

Emit when miner accepted a task

### SubmitTask

```solidity
event SubmitTask(uint256 id, bytes proof)
```

Emit when miner submit a proof for a task

### ProxyTask

```solidity
event ProxyTask(uint256 id, address prover, address player, address miner)
```

Emit when sent proxy task

### DisputeTask

```solidity
event DisputeTask(uint256 id, address sender, uint256 deposit)
```

Emit when task into disputed

### AdjudicateTask

```solidity
event AdjudicateTask(uint256 id, address sender, uint256 playerAmount, uint256 minerAmount, uint256 daoAmount, bool slash)
```

Emit when task have beed adjudicated

### initialize

```solidity
function initialize(address _addresses, uint256 _disputeDeposit) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |
| _disputeDeposit | uint256 |  |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |

### setDisputeDeposit

```solidity
function setDisputeDeposit(uint256 _disputeDeposit) external
```

Set the dispute deposit

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _disputeDeposit | uint256 | the amount |

### create

```solidity
function create(address prover, address player, uint256 fee, bytes inputs, bytes publics) external returns (uint256)
```

Create new zk task of a prover

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |
| player | address | the player account |
| fee | uint256 | the fee fot this task |
| inputs | bytes | the zk serialized inputs data |
| publics | bytes | the zk serialized publics data |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the task id |

### accept

```solidity
function accept(uint256 tid, address miner, string url) external
```

Accept a task by miner, can be called by miner or controller

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| tid | uint256 | the task id |
| miner | address | the miner account |
| url | string | the url which can reach the miner |

### submit

```solidity
function submit(uint256 tid, bytes proof) external
```

Submit a proof for a task, will call verifier to verify

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| tid | uint256 | the task id |
| proof | bytes | the zk proof |

### setProxyList

```solidity
function setProxyList(address account, bool isOk) external
```

Set the proxy allow account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the allow account |
| isOk | bool | the status |

### proxy

```solidity
function proxy(address[] provers, address[] players, address[] miners) external
```

Submit multiple proxy tasks

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| provers | address[] | the prover list |
| players | address[] | the player list |
| miners | address[] | the miner list |

### dispute

```solidity
function dispute(uint256 tid) external
```

Dispute the task, player & miner can call this, and need deposit for security,
and DAO will check.

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| tid | uint256 | the task id |

### adjudicate

```solidity
function adjudicate(uint256 tid, uint256 playerAmount, uint256 minerAmount, bool slash) external
```

Adjudicate the task dispute, if playerAmount > minerAmount, player win, otherwise miner win.

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| tid | uint256 | the task id |
| playerAmount | uint256 | the amount will send to player |
| minerAmount | uint256 | the amount will send to miner |
| slash | bool | if need slash miner's staking to player |

