// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IController.sol";
import "./interface/IProver.sol";
import "./interface/IReward.sol";
import "./interface/IStake.sol";
import "./interface/ITask.sol";
import "./interface/IVerifier.sol";

/// @notice Manage all proof tasks, player create new zk task, and miner can accept it,
/// when miner acceped, miner need submit the proof within overtime, if overflow, others
/// can accept and replace, and previous miner will be punished
contract Task is Initializable, OwnableUpgradeable, ITask {
    using SafeERC20 for IERC20;

    /// @notice Struct of ZK Task
    struct Task {
        /// notice TaskStatus including: Over, Waiting, Proving
        TaskStatus status;
        /// @notice the prover address
        address prover;
        /// @notice the player account
        address player;
        /// @notice the fee for this task
        uint256 fee;
        /// @notice the miner account
        address miner;
        /// @notice the overtime of this task
        uint256 overtime;
        /// @notice the proof public inputs data
        bytes publics;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Next task id
    uint256 public nextId;

    /// @notice Store all tasks
    mapping(uint256 => Task) private tasks;

    /// @notice Emit when created a new task
    event CreateTask(uint256 id, address prover, address player, uint256 fee, bytes inputs, bytes publics);

    /// @notice Emit when miner accepted a task
    event AcceptTask(uint256 id, address miner, uint256 overtime);

    /// @notice Emit when miner submit a proof for a task
    event SubmitTask(uint256 id, bytes proof);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Create new zk task of a prover
    /// @param prover the prover address
    /// @param player the player account
    /// @param fee the fee fot this task
    /// @param inputs the zk serialized inputs data
    /// @param publics the zk serialized publics data
    /// @return the task id
    function create(address prover, address player, uint256 fee, bytes calldata inputs, bytes calldata publics) external returns(uint256) {
        // transfer fee from msg.sender
        if (fee > 0) {
            IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(msg.sender, address(this), fee);
        }

        // check prover is valid
        require(IProver(IAddresses(addresses).get(Contracts.Prover)).isProver(prover), "T01");

        Task storage task = tasks[nextId];
        task.prover = prover;
        task.player = player;
        task.fee = fee;
        task.publics = publics;

        emit CreateTask(nextId, prover, player, fee, inputs, publics);

        nextId += 1;
        return nextId - 1;
    }

    /// @notice Accept a task by miner, can be called by miner or controller
    /// @param tid the task id
    /// @param miner the miner account
    function accept(uint256 tid, address miner) external {
        require(IController(IAddresses(addresses).get(Contracts.Controller)).check(miner, msg.sender), "T02");

        Task storage task = tasks[tid];
        require(IStake(IAddresses(addresses).get(Contracts.Stake)).isMiner(task.prover, miner), "T03");

        bool acceptable = task.status == TaskStatus.Waiting || task.overtime < block.timestamp;
        require(acceptable, "T04");

        uint256 overtime = IProver(IAddresses(addresses).get(Contracts.Prover)).overtime(task.prover);
        task.status = TaskStatus.Proving;
        task.miner = miner;
        task.overtime = block.timestamp + overtime;

        emit AcceptTask(tid, miner, task.overtime);
    }

    /// @notice Submit a proof for a task, will call verifier to verify
    /// @param tid the task id
    /// @param proof the zk proof
    function submit(uint256 tid, bytes calldata proof) external {
        Task storage task = tasks[tid];

        require(task.status == TaskStatus.Proving, "T05");

        // zk verifier
        address verifier = IProver(IAddresses(addresses).get(Contracts.Prover)).verifier(task.prover);
        require(IVerifier(verifier).verify(task.publics, proof), "T99");

        // send fee to miner
        if (task.fee > 0) {
            IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(task.miner, task.fee);
        }
        emit SubmitTask(tid, proof);

        // PoZK to reward
        IReward(IAddresses(addresses).get(Contracts.Reward)).work(task.prover, task.player, task.miner);

        delete tasks[tid];
    }
}
