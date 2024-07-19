# Solidity API

## Stake

Stake Contract, including player, miner & game,
every change will work in next epoch, and unstake can claim in next epoch

### Staking

Unit struct about staking/unstaking

```solidity
struct Staking {
  uint256 value;
  uint256 newValue;
  uint256 newEpoch;
}
```

### GameStaking

Unit struct about staking in a game

```solidity
struct GameStaking {
  struct Stake.Staking gamerTotal;
  mapping(address => uint256) gamers;
  struct Stake.Staking minerTotal;
  mapping(address => struct Stake.Staking) miners;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### minStakeAmount

```solidity
uint256 minStakeAmount
```

Miner minStakeAmount

### GameStakeChange

```solidity
event GameStakeChange(uint256 epoch, address game, address account, int256 changed, uint256 total)
```

Emit when game staking change

### MinerStakeChange

```solidity
event MinerStakeChange(uint256 epoch, address game, address account, int256 changed, uint256 total)
```

Emit when miner staking change

### PlayerStakeChange

```solidity
event PlayerStakeChange(uint256 epoch, address account, int256 changed, uint256 total)
```

Emit when player staking change

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

### setMinStakeAmount

```solidity
function setMinStakeAmount(uint256 _minStakeAmount) external
```

Set minimum stake amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _minStakeAmount | uint256 | the minimum value of miner staking |

### gameTotalStaking

```solidity
function gameTotalStaking(address game) external view returns (uint256)
```

Get total game staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | total game staking amount |

### gameStaking

```solidity
function gameStaking(address game, address account) external view returns (uint256)
```

Get game staking by account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| account | address | the staking account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the staking amount of this account |

### gameStake

```solidity
function gameStake(address game, uint256 amount) external
```

Stake by game self(others)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| amount | uint256 | new staking amount |

### gameUnstake

```solidity
function gameUnstake(address game, uint256 amount) external
```

Unstake by game self(others)

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| amount | uint256 | the unstaking amount |

### minerTotalStaking

```solidity
function minerTotalStaking(address game) external view returns (uint256)
```

Get total miner staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the total miner staking amount |

### minerStaking

```solidity
function minerStaking(address game, address account) public view returns (uint256)
```

Get miner staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| account | address | miner account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the miner staking amount |

### isMiner

```solidity
function isMiner(address game, address account) external view returns (bool)
```

Check account is miner or not

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| account | address | the checking account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | account is miner or not |

### minerStake

```solidity
function minerStake(address game, uint256 amount) external
```

Stake by miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| amount | uint256 | the new staking amount |

### minerUnStake

```solidity
function minerUnStake(address game, uint256 amount) external
```

Unstake by miner

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| amount | uint256 | the unstaking amount |

### playerTotalStaking

```solidity
function playerTotalStaking() external view returns (uint256)
```

Get total player staking

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the total staking amount of players |

### playerStaking

```solidity
function playerStaking(address account) external view returns (uint256)
```

Get player staking

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the player account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the staking amount of player |

### playerStake

```solidity
function playerStake(uint256 amount) external
```

Stake by player

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| amount | uint256 | the new staking amount of player |

### playerUnStake

```solidity
function playerUnStake(uint256 amount) external
```

Unstake by player

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| amount | uint256 | the unstaking amount |

### claimable

```solidity
function claimable(address account) external view returns (uint256)
```

Get claimable unstaking amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the claiming account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the amount which can claim now |

### claim

```solidity
function claim(address account) external
```

Claim unstaking to account

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the claiming account |

## TaskMarket

Manage all proof tasks, player create new zk task, and miner can accept it,
when miner acceped, miner need submit the proof within overtime, if overflow, others
can accept and replace, and previous miner will be punished

### Task

Struct of ZK Task

```solidity
struct Task {
  enum TaskStatus status;
  address game;
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
event CreateTask(uint256 id, address game, address player, uint256 fee, bytes data)
```

Emit when created a new task

### AcceptTask

```solidity
event AcceptTask(uint256 id, address miner, uint256 overtime)
```

Emit when miner accepted a task

### SubmitTask

```solidity
event SubmitTask(uint256 id, uint256 fee)
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
function create(address game, address player, uint256 fee, bytes data) external returns (uint256)
```

Create new zk task of a game

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
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

## Token

the main Token(ERC20) for zypher network

### constructor

```solidity
constructor(uint256 initialSupply) public
```

## Vesting

Token lock status and unlock period

### addresses

```solidity
address addresses
```

Common Addresses contract

### rewardPerEpoch

```solidity
uint256 rewardPerEpoch
```

Rewards of every epoch will be released for mine & play

### miners

```solidity
mapping(address => uint256) miners
```

Store all miners vesting

### initialize

```solidity
function initialize(address _addresses, uint256 _rewardPerEpoch) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |
| _rewardPerEpoch | uint256 | the reward amount of every epoch |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |

### mine

```solidity
function mine(uint256 epoch) external view returns (uint256)
```

Get the mine amount of every epoch
epoch the epoch height

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the amount of reward |

### setMinerAmount

```solidity
function setMinerAmount(address[] _miners, uint256[] amounts) external
```

Batch set miner vesting amounts

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _miners | address[] | the miners list |
| amounts | uint256[] | the amounts list |

### minersTotal

```solidity
function minersTotal() external view returns (uint256)
```

Get all miners vesting amount

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | total amount of all miners |

### miner

```solidity
function miner(address account) external view returns (uint256)
```

Get miner vesting amount

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| account | address | the miner account |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | the amount of this miner |

## Reward

Reward Contract for reward distribute and claim, miner & player can get reward,
when they play game and prove zkp, all of them can get reward,
use cobb-douglas function for work and labor

### StakerLabor

The unit struct of stake and labor

```solidity
struct StakerLabor {
  uint256 staking;
  uint256 working;
}
```

### GamePool

The struct of game pool

```solidity
struct GamePool {
  uint256 totalWorking;
  uint256 totalStaking;
  uint256 totalMinerStaking;
  uint256 totalPlayerStaking;
  uint256 totalMinerReward;
  uint256 totalPlayerReward;
  uint256 unclaimReward;
  uint256 unclaimLabor;
  mapping(address => struct Reward.StakerLabor) minerLabor;
  mapping(address => struct Reward.StakerLabor) playerLabor;
}
```

### RunningGame

The struct of a game status

```solidity
struct RunningGame {
  uint256 unclaim;
  address[] games;
  mapping(address => uint256) index;
}
```

### EpochPool

The struct of the epoch status

```solidity
struct EpochPool {
  uint256 unclaim;
  uint256 totalGameStaking;
  mapping(address => struct Reward.GamePool) gamePools;
  mapping(address => struct Reward.RunningGame) minerUnclaimedGames;
  mapping(address => struct Reward.RunningGame) playerUnclaimedGames;
}
```

### addresses

```solidity
address addresses
```

Common Addresses contract

### alphaNumerator

```solidity
int256 alphaNumerator
```

The numerator of Percentage of the game stake and labor (1-alpha) in the total

### alphaDenominator

```solidity
int256 alphaDenominator
```

The denominator of the alpha

### betaNumerator

```solidity
int256 betaNumerator
```

The numerator of Percentage of the miner stake and labor (1-beta) in the total

### betaDenominator

```solidity
int256 betaDenominator
```

The denominator of the beta

### gammaNumerator

```solidity
int256 gammaNumerator
```

The numerator of Percentage of the player stake and labor (1-beta) in the total

### gammaDenominator

```solidity
int256 gammaDenominator
```

The denominator of the gamma

### Alpha

```solidity
event Alpha(int256 alphaNumerator, int256 alphaDenominator)
```

Emitted when update the alpha for cobb-douglas function

### Beta

```solidity
event Beta(int256 betaNumerator, int256 betaDenominator)
```

Emitted when update the alpha for cobb-douglas function

### Gamma

```solidity
event Gamma(int256 betaNumerator, int256 betaDenominator)
```

Emitted when update the alpha for cobb-douglas function

### MinerLabor

```solidity
event MinerLabor(uint256 epoch, address game, address miner, uint256 work)
```

Emitted when add Labor(reward) for current pool

### PlayerLabor

```solidity
event PlayerLabor(uint256 epoch, address game, address player, uint256 play)
```

Emitted when add Labor(reward) for current pool

### MinerCollect

```solidity
event MinerCollect(uint256 epoch, address game, address miner, uint256 amount)
```

Emitted when collect reward (stake) from pool

### PlayerCollect

```solidity
event PlayerCollect(uint256 epoch, address game, address player, uint256 amount)
```

Emitted when collect reward (stake) from pool

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

### setAlpha

```solidity
function setAlpha(int256 _alphaNumerator, int256 _alphaDenominator) public
```

Update the alpha for cobb-douglas function

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _alphaNumerator | int256 | the numerator of the alpha |
| _alphaDenominator | int256 | the denominator of the alpha |

### setBeta

```solidity
function setBeta(int256 _betaNumerator, int256 _betaDenominator) public
```

Update the beta for cobb-douglas function

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _betaNumerator | int256 | the numerator of the beta |
| _betaDenominator | int256 | the denominator of the beta |

### setGamma

```solidity
function setGamma(int256 _gammaNumerator, int256 _gammaDenominator) public
```

Update the gamma for cobb-douglas function

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _gammaNumerator | int256 | the numerator of the gamma |
| _gammaDenominator | int256 | the denominator of the gamma |

### work

```solidity
function work(address game, address player, address miner) external
```

Add work(labor) to current epoch & game, only call from TaskMarket

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| game | address | the game address |
| player | address | player account |
| miner | address | miner account |

### minerCollect

```solidity
function minerCollect(uint256 epoch, address game, address miner) public
```

Miner collect reward in epoch and game, collect reward to unstaking list

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| game | address | the game address |
| miner | address | the miner account |

### playerCollect

```solidity
function playerCollect(uint256 epoch, address game, address player) public
```

Player collect reward in epoch and game, collect to player wallet

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| game | address | the game address |
| player | address | the player account |

### minerBatchCollect

```solidity
function minerBatchCollect(uint256 epoch, address miner) external
```

Miner batch collect all games in a epoch

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| miner | address | the miner account |

### playerBatchCollect

```solidity
function playerBatchCollect(uint256 epoch, address player) external
```

Player batch collect all games in a epoch

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| epoch | uint256 | the epoch height |
| player | address | the player account |

## Addresses

Store all contracts address and can update them

### initialize

```solidity
function initialize() public
```

Initialize

### set

```solidity
function set(enum Contracts c, address _address) external
```

Owner can update some contract address

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| c | enum Contracts | the Contract enum |
| _address | address | the contract new address |

### batchSet

```solidity
function batchSet(enum Contracts[] _cs, address[] _addresses) external
```

Owner can batch update contracts' address

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _cs | enum Contracts[] | the Contracts list |
| _addresses | address[] | the contracts new addresses |

### get

```solidity
function get(enum Contracts c) external view returns (address)
```

Get contract address

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| c | enum Contracts | the Contract enum |

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | address | the contract address |

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

## Epoch

Phases in the network, simulating "block height" in blockchain,
stake and reward are effective and issued according to the epoch

### addresses

```solidity
address addresses
```

Common Addresses contract

### period

```solidity
uint256 period
```

Period time in seconds

### startTime

```solidity
uint256 startTime
```

Current epoch start time

### now

```solidity
uint256 now
```

Current epoch height

### maintenance

```solidity
bool maintenance
```

Enter/esc maintenance mode, when entry maintenance mode, stake and reward will be stopped

### initialize

```solidity
function initialize(address _addresses, uint256 _period) public
```

Initialize

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract |
| _period | uint256 | the epoch period time in seconds |

### setAddresses

```solidity
function setAddresses(address _addresses) external
```

Set the Addresses contract

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _addresses | address | the Addresses contract address |

### setMaintenance

```solidity
function setMaintenance(bool open) external
```

Set maintenance mode status

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| open | bool | open or false the maintenance mode |

### setPeriod

```solidity
function setPeriod(uint256 _period) external
```

Update period time

#### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| _period | uint256 | the period time in seconds |

### getAndUpdate

```solidity
function getAndUpdate() external returns (uint256)
```

Update and get latest epoch height

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | latest epoch height |

### get

```solidity
function get() external view returns (uint256)
```

Get current epoch height

#### Return Values

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | Current epoch height |

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

