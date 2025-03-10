# Solidity API

## Prover

Manage all registered provers

### ProverWork

Unit struct for number change

```solidity
struct ProverWork {
  uint256 value;
  uint256 newValue;
  uint256 newEpoch;
}
```

### ProverVerifier

Unit struct for prover verifier

```solidity
struct ProverVerifier {
  address value;
  address newValue;
  uint256 newEpoch;
}
```

### GameProver

GameProver struct

```solidity
struct GameProver {
  enum ProverStatus status;
  enum ProverType ptype;
  address owner;
  struct Prover.ProverWork work;
  struct Prover.ProverWork version;
  struct Prover.ProverWork overtime;
  struct Prover.ProverVerifier verifier;
  string types;
  bool minable;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### RegisterProver

```solidity
event RegisterProver(address prover, enum ProverType ptype, uint256 work, uint256 version, uint256 overtime, address verifier, string name, string types)
```

Emit when new prover register and waiting reviewing

### TransferProver

```solidity
event TransferProver(address prover, address owner)
```

Emit when prover owner transfer to others

### UpgradeProver

```solidity
event UpgradeProver(address prover, enum ProverType ptype, uint256 work, uint256 version, uint256 overtime, address verifier, string name, string types)
```

Emit when the prover start upgrading and waiting reviewing, before approve, it will still use old info

### ApproveProver

```solidity
event ApproveProver(address prover, enum ProverType ptype, uint256 work, uint256 epoch, uint256 version, uint256 overtime, address verifier, string types, bool minable, bool approved)
```

Emit when the prover is approved or reject

### StopProver

```solidity
event StopProver(address prover)
```

Emit when the prover is stopped

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
function register(address prover, enum ProverType _ptype, uint256 _work, uint256 _version, uint256 _overtime, address _verifier) external
```

Register new prover to market, the sender is prover owner, and waiting review

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover contract(or not) address (unique identifier) |
| _ptype | enum ProverType | the prover type |
| _work | uint256 | the prover pozk work, calculation based on zk scheme and circuit size |
| _version | uint256 | the prover prover version |
| _overtime | uint256 | the limit time when doing zkp, if overflow the time, others miner can accept the task again |
| _verifier | address | the verifier contract, uses the IVerifier interface |

### unregister

```solidity
function unregister(address prover) external
```

Prover owner can unregister the prover and cannot register same prover address again

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover address |

### upgrade

```solidity
function upgrade(address prover, enum ProverType _ptype, uint256 _work, uint256 _version, uint256 _overtime, address _verifier) external
```

Prover owner can upgrade the prover to new verison and new info

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |
| _ptype | enum ProverType | the prover type |
| _work | uint256 | the prover next pozk work, calculation based on zk scheme and circuit size |
| _version | uint256 | the prover next prover version |
| _overtime | uint256 | the limit time when doing zkp, if overflow the time, others miner can accept the task again |
| _verifier | address | the next verifier contract, uses the IVerifier interface |

### transferProverOwner

```solidity
function transferProverOwner(address prover, address _owner) external
```

Prover owner can transfer ownership to others

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |
| _owner | address | the new owner account |

### approve

```solidity
function approve(address prover, bool minable, bool approved) external
```

DAO can approve or reject the prover register and upgrade, if approve, it will works in next epoch

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |
| minable | bool | if the prover is minable, that means when create/accept the prover task, will get reward from network |
| approved | bool | approve or reject |

### stop

```solidity
function stop(address prover) external
```

DAO can stop a prover

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |

### isProver

```solidity
function isProver(address prover) external view returns (bool)
```

Check a prover is working (working or upgrading)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | working or not |

### work

```solidity
function work(address prover) external view returns (uint256)
```

Get a prover work

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the work of the prover |

### version

```solidity
function version(address prover) external view returns (uint256)
```

notice Get a prover version

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the version of the prover |

### overtime

```solidity
function overtime(address prover) external view returns (uint256)
```

notice Get a prover overtime

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the overtime of the prover |

### verifier

```solidity
function verifier(address prover) external view returns (address)
```

notice Get a prover verifier

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | address | the verifier of the prover |

### owner

```solidity
function owner(address prover) external view returns (address)
```

notice Get a prover owner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | address | the owner account of the prover |

### checkUrl

```solidity
function checkUrl(address prover, string url) external view returns (bool)
```

notice Check the prover is need URL or not

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| prover | address | the prover |
| url | string | the url string |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | the result |

