# Solidity API

## DemoProver

### supportsInterface

```solidity
function supportsInterface(bytes4 interfaceId) public view virtual returns (bool)
```

_See {IERC165-supportsInterface}._

### name

```solidity
function name() external pure returns (string)
```

game (prover/verifier) name

### permission

```solidity
function permission(address _sender) external view returns (bool)
```

### verify

```solidity
function verify(bytes _publics, bytes _proof) external view returns (bool)
```

### inputs

```solidity
function inputs() external pure returns (string)
```

show how to serialize/deseriaze the inputs params
e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"

### publics

```solidity
function publics() external pure returns (string)
```

show how to serialize/deserialize the publics params
e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"

### types

```solidity
function types() external pure returns (string)
```

show the prover supported types
e.g. "zk", "risc0,sp1", "candle,ollama"

