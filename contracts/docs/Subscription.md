# Solidity API

## Subscription

1:M mode subscription & payment

### State

Miner's task State

```solidity
struct State {
  uint256 epoch;
  bytes32 state;
}
```

### Task

PaymentTask struct

```solidity
struct Task {
  address payer;
  uint256 total;
  uint256 capacity;
  uint256 endAt;
  bool hasUrl;
  address[] miners;
  mapping(address => struct Subscription.State) states;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### nextTaskId

```solidity
uint256 nextTaskId
```

Next Task id

### tasks

```solidity
mapping(uint256 => struct Subscription.Task) tasks
```

Store all controllers by account

### CreateSubscription

```solidity
event CreateSubscription(uint256 id, address payer, uint256 total, uint256 capacity, uint256 endtime)
```

Emit when new Subscription task created

### AcceptSubscription

```solidity
event AcceptSubscription(uint256 id, address miner, string url)
```

### SettleSubscription

```solidity
event SettleSubscription(uint256 id, address miner)
```

### withdrawSubscription

```solidity
event withdrawSubscription(uint256 id, uint256 amount)
```

### DisputeSubscription

```solidity
event DisputeSubscription(uint256 id, address sender)
```

### AdjudicateSubscription

```solidity
event AdjudicateSubscription(uint256 id, address sender, uint256 daoAmount, bool slash)
```

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

### create

```solidity
function create(address prover, address payer, uint256 fee, uint256 capacity, uint256 end) external returns (uint256)
```

payer create a subscription task

### accept

```solidity
function accept(uint256 tid) external
```

Miner accept the task

### settle

```solidity
function settle(uint256 tid, address miner) external
```

Miner report state & claim rewards

### withdraw

```solidity
function withdraw(uint256 tid) external
```

Withdraw remain amount of task

### dispute

```solidity
function dispute(uint256 tid) external
```

Dispute the task, payer & miner can call this, no need deposit

### adjudicate

```solidity
function adjudicate(uint256 tid, uint256 daoAmount) external
```

Adjudicate the task dispute by DAO

