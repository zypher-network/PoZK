# Solidity API

## L2Vesting

Token lock status and unlock period

### Plan

Vesting plan

```solidity
struct Plan {
  uint256 initial;
  uint256 period;
  uint256 start;
}
```

### token

```solidity
address token
```

the token address

### plans

```solidity
struct L2Vesting.Plan[] plans
```

vesting plans

### allocations

```solidity
mapping(uint256 => mapping(address => uint256)) allocations
```

store the allocations for user by planId: planId => user => amount

### claimed

```solidity
mapping(uint256 => mapping(address => uint256)) claimed
```

store claimed amount for user by planId: planId => user => amount

### AddPlan

```solidity
event AddPlan(uint256 planId, uint256 initial, uint256 period, uint256 start)
```

Emitted when new plan was created or updated

### Allocated

```solidity
event Allocated(uint256 planId, address user, uint256 allocation)
```

Emitted when new allocation was added to a user by planId

### Claimed

```solidity
event Claimed(uint256 planId, address user, uint256 amount)
```

Emitted when user claimed vested token

### initialize

```solidity
function initialize(address _token) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _token | address | the Token contract |

### setToken

```solidity
function setToken(address _token) external
```

Set the Token contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _token | address | the Token contract |

### addPlan

```solidity
function addPlan(uint256 initial, uint256 period, uint256 start) external
```

Add new plan

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| initial | uint256 | the initial amount without lock |
| period | uint256 | the vesting period |
| start | uint256 | the start date |

### startPlan

```solidity
function startPlan(uint256 planId, uint256 period, uint256 start) external
```

Update plan start date

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| planId | uint256 | the plan id |
| period | uint256 | the vesting period |
| start | uint256 | the start date |

### allocate

```solidity
function allocate(uint256[] _planIds, address[] _users, uint256[] _allocations) external
```

Batch set the allocations

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _planIds | uint256[] | the plan id list |
| _users | address[] | the user list |
| _allocations | uint256[] | the allocation list |

### claim

```solidity
function claim(uint256 planId, address user) external
```

Claim vested amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| planId | uint256 | the plan id |
| user | address | the user account |

### claimable

```solidity
function claimable(uint256 planId, address user) public view returns (uint256)
```

Get claimable amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| planId | uint256 | the plan id |
| user | address | the user account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the amount |

### withdrawByAdmin

```solidity
function withdrawByAdmin() external
```

Withdraw all vesting token to admin for suspend

