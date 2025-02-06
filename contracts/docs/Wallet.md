# Solidity API

## Wallet

### NAME

```solidity
string NAME
```

### DOMAIN_TYPE

```solidity
bytes32 DOMAIN_TYPE
```

### CALL_MESSAGE_TYPE

```solidity
bytes32 CALL_MESSAGE_TYPE
```

### MULTICALL_MESSAGE_ITEM_TYPE

```solidity
bytes32 MULTICALL_MESSAGE_ITEM_TYPE
```

### MULTICALL_MESSAGE_TYPE

```solidity
bytes32 MULTICALL_MESSAGE_TYPE
```

### FUNCTION_CALL_TIP

```solidity
bytes32 FUNCTION_CALL_TIP
```

### FUNCTION_MULTICALL_TIP

```solidity
bytes32 FUNCTION_MULTICALL_TIP
```

### MessageItem

```solidity
struct MessageItem {
  address from;
  address to;
  uint256 value;
  bytes data;
}
```

### domain

```solidity
bytes32 domain
```

### owner

```solidity
address owner
```

### nonce

```solidity
uint256 nonce
```

### constructor

```solidity
constructor(address _owner) public
```

### functionCall

```solidity
function functionCall(address from, address payable to, uint256 value, bytes data, uint8 v, bytes32 r, bytes32 s) public payable
```

### functionMulticall

```solidity
function functionMulticall(struct Wallet.MessageItem[] items, uint8 v, bytes32 r, bytes32 s) public payable
```

### receive

```solidity
receive() external payable
```

