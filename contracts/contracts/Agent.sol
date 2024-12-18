// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IController.sol";
import "./interface/IEpoch.sol";
import "./interface/IProver.sol";
import "./interface/IReward.sol";
import "./interface/IStake.sol";
import "./interface/ITask.sol";
import "./interface/IVerifier.sol";

/// @notice 1:M mode agent & payment
contract Agent is Initializable, OwnableUpgradeable, IAgent {
    /// @notice Miner's task State
    struct State {
        /// @notice latest reported time
        uint256 time;
        /// @notice latest reported state
        bytes32 state;
    }

    /// @notice PaymentTask struct
    struct AgentTask {
        /// @notice The prover address
        address prover;
        /// @notice The player account
        address player;
        /// @notice The creator account
        address creator;
        /// @notice The total fee
        uint256 fee;
        /// @notice How many miner can join this payment task
        uint256 capacity;
        /// @notice The start time
        uint256 starttime;
        /// @notice The end time;
        uint256 endtime;
        /// @notice the fee of every miner and every seconds
        uint256 perFee;
        /// @notice the joined miners
        address[] miners;
        /// @notice the miners' state
        mapping(address => State) states;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Next Task id
    uint256 public nextTaskId;

    /// @notice Store all controllers by account
    mapping(uint256 => AgentTask) tasks;

    /// @notice Emit when new agent task created
    event CreateAgent(uint256 id, address prover, address player, uint256 fee, uint256 capacity, uint256 starttime, uint256 endtime);

    /// @notice Emit when new miner accepted the agent task
    event AcceptAgent(uint256 id, address miner, string url);

    /// @notice Emit when miner settle the agent task status
    event SettleAgent(uint256 id, address miner);

    /// @notice Emit when player withdraw unused fee when time passed
    event withdrawAgent(uint256 id, uint256 amount);

    /// @notice Emit when task into disputed
    event DisputeAgent(uint256 id, address sender);

    /// @notice Emit when task have beed adjudicated
    event AdjudicateAgent(uint256 id, address sender, uint256 daoAmount, bool slash);

    /// @notice Initialize
    /// @param _addresses the common Addresses contract
    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    /// @notice Set common Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice player create a agent task
    function create(address prover, address player, uint256 fee, uint256 capacity, uint256 endtime) external returns(uint256) {
        // transfer fee from msg.sender
        require(fee > 0, "A01");
        IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(msg.sender, address(this), fee);

        // check time
        uint256 starttime = block.timestamp;
        require(endtime > starttime, "A02");

        IProver iprover = IProver(IAddresses(addresses).get(Contracts.Prover));
        require(iprover.isProver(prover), "A03");
        require(IVerifier(iprover.verifier(prover)).permission(msg.sender), "A11");

        AgentTask storage task = tasks[nextId];
        task.prover = prover;
        task.player = player;
        task.creator = msg.sender;
        task.fee = fee;
        task.capacity = capacity;
        task.starttime = starttime;
        task.endtime = endtime;
        task.perFee = fee / capacity / (endtime - starttime);

        emit CreateAgent(nextId, prover, player, fee, capacity, starttime, endtime);

        nextId += 1;
        return nextId - 1;

    }
        //

    /// @notice Miner accept the task
    function accept(uint256 tid) external {
        //
    }

    /// @notice Miner report state & claim rewards
    function settle(uint256 tid, address miner) external {
        //
    }

    /// @notice Player/Creator extend agent time or add more capacity, cannot change perFee
    function extend(uint256 tid, uint256 endtime, uint256 addCapacity) external {
        // TODO check sender

        AgentTask storage task = tasks[tid];
        task.capacity += addCapacity;

        if (endtime > task.endtime) {
            time = endtime - task.endtime;
        } else {
            time = task.endtime - block.timestamp;
        }
    }

    /// @notice Player/Creator withdraw remain amount of task
    function withdraw(uint256 tid) external {
        //
    }

    /// @notice Dispute the task, payer & miner can call this, no need deposit
    function dispute(uint256 tid) external {
        //
    }

    /// @notice Adjudicate the task dispute by DAO
    function adjudicate(uint256 tid, uint256 daoAmount) external {
        //
    }
}
