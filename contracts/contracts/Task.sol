// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
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

/// @notice Manage all proof tasks, player create new zk task, and miner can accept it,
/// when miner acceped, miner need submit the proof within overtime, if overflow, others
/// can accept and replace, and previous miner will be punished
contract Task is Initializable, OwnableUpgradeable, ITask {
    using SafeERC20 for IERC20;

    /// @notice Struct of ZK Task
    struct GameTask {
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
        /// @notice the miner url
        string url;
        /// @notice the overtime of this task
        uint256 overtime;
        /// @notice the proof public inputs data
        bytes publics;
        /// @notice the dispute deposit when disputing
        uint256 dispute;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Next task id, start from 1
    uint256 public nextId;

    /// @notice the deposit for security when dispute
    uint256 public disputeDeposit;

    /// @notice Store all tasks
    mapping(uint256 => GameTask) public tasks;

    /// @notice Store all tasks results
    mapping(bytes32 => uint256) public tasksResults;

    /// @notice Store all proxy allow list
    mapping(address => bool) public proxyList;

    /// @notice Emit when created a new task
    event CreateTask(uint256 id, address prover, address player, uint256 fee, bytes inputs, bytes publics);

    /// @notice Emit when miner accepted a task
    event AcceptTask(uint256 id, address miner, uint256 overtime, string url);

    /// @notice Emit when miner submit a proof for a task
    event SubmitTask(uint256 id, bytes proof);

    /// @notice Emit when sent proxy task
    event ProxyTask(uint256 id, address prover, address player, address miner);

    /// @notice Emit when task into disputed
    event DisputeTask(uint256 id, address sender, uint256 deposit);

    /// @notice Emit when task have beed adjudicated
    event AdjudicateTask(uint256 id, address sender, uint256 playerAmount, uint256 minerAmount, uint256 daoAmount, bool slash);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    function initialize(address _addresses, uint256 _disputeDeposit) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
        disputeDeposit = _disputeDeposit;
        nextId = 1;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Set the dispute deposit
    /// @param _disputeDeposit the amount
    function setDisputeDeposit(uint256 _disputeDeposit) external onlyOwner {
        disputeDeposit = _disputeDeposit;
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

        // check prover is valid and permission
        IProver iprover = IProver(IAddresses(addresses).get(Contracts.Prover));
        require(iprover.isProver(prover), "T01");
        require(IVerifier(iprover.verifier(prover)).permission(msg.sender), "T11");

        GameTask storage task = tasks[nextId];
        task.status = TaskStatus.Waiting;
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
    /// @param url the url which can reach the miner
    function accept(uint256 tid, address miner, string calldata url) external {
        require(IController(IAddresses(addresses).get(Contracts.Controller)).check(miner, msg.sender), "T02");

        GameTask storage task = tasks[tid];
        require(IStake(IAddresses(addresses).get(Contracts.Stake)).isMiner(task.prover, miner), "T03");

        bool acceptable = task.status == TaskStatus.Waiting || (task.overtime != 0 && task.overtime < block.timestamp);
        require(acceptable, "T04");

        IProver iprover = IProver(IAddresses(addresses).get(Contracts.Prover));
        require(iprover.checkUrl(task.prover, url), "T11");
        uint256 overtime = iprover.overtime(task.prover);

        task.status = TaskStatus.Proving;
        task.miner = miner;
        task.url = url;
        task.overtime = block.timestamp + overtime;

        emit AcceptTask(tid, miner, task.overtime, url);
    }

    /// @notice Submit a proof for a task, will call verifier to verify
    /// @param tid the task id
    /// @param proof the zk proof
    function submit(uint256 tid, bytes calldata proof) external {
        GameTask storage task = tasks[tid];
        require(task.status == TaskStatus.Proving, "T05");

        bytes32 hash = keccak256(abi.encode(task.publics, proof));
        require(tasksResults[hash] == 0, "T98");
        tasksResults[hash] = tid;

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

    /// @notice Set the proxy allow account
    /// @param account the allow account
    /// @param isOk the status
    function setProxyList(address account, bool isOk) external onlyOwner {
        proxyList[account] = isOk;
    }

    /// @notice Submit multiple proxy tasks
    /// @param provers the prover list
    /// @param players the player list
    /// @param miners the miner list
    function proxy(address[] calldata provers, address[] calldata players, address[] calldata miners) external {
        // check sender in whitelist
        require(proxyList[msg.sender], "T06");

        for (uint i = 0; i < provers.length; i++) {
            // PoZK to reward
            IReward(IAddresses(addresses).get(Contracts.Reward)).work(provers[i], players[i], miners[i]);

            emit ProxyTask(nextId, provers[i], players[i], miners[i]);
            nextId += 1;
        }
    }

    /// @notice Dispute the task, player & miner can call this, and need deposit for security,
    /// and DAO will check.
    /// @param tid the task id
    function dispute(uint256 tid) external {
        GameTask storage task = tasks[tid];
        require(task.status == TaskStatus.Proving, "T05");

        require(
            msg.sender == task.player ||
            IController(IAddresses(addresses).get(Contracts.Controller)).check(task.miner, msg.sender),
        "T07");

        // transfer
        IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(msg.sender, address(this), disputeDeposit);

        task.status = TaskStatus.Disputing;
        task.dispute = disputeDeposit;

        emit DisputeTask(tid, msg.sender, disputeDeposit);
    }

    /// @notice Adjudicate the task dispute, if playerAmount > minerAmount, player win, otherwise miner win.
    /// @param tid the task id
    /// @param playerAmount the amount will send to player
    /// @param minerAmount the amount will send to miner
    /// @param slash if need slash miner's staking to player
    function adjudicate(uint256 tid, uint256 playerAmount, uint256 minerAmount, bool slash) external {
        GameTask storage task = tasks[tid];
        require(task.status == TaskStatus.Disputing, "T08");
        require(task.dispute >= playerAmount + minerAmount, "T09");
        require(IEpoch(IAddresses(addresses).get(Contracts.Epoch)).isDao(msg.sender), "T10");

        if (playerAmount > 0) {
            IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(task.player, playerAmount);
        }

        if (minerAmount > 0) {
            IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(task.miner, minerAmount);
        }

        uint256 daoAmount = task.dispute - playerAmount - minerAmount;
        if (daoAmount > 0) {
            IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(msg.sender, daoAmount);
        }

        if (playerAmount > minerAmount && slash) {
            // slash miner staking amount to player
            IStake(IAddresses(addresses).get(Contracts.Stake)).minerSlashStaking(task.miner, task.prover, task.player, task.dispute);
        }

        if (minerAmount > playerAmount) {
            // give miner reward directly, and player change to dao
            IReward(IAddresses(addresses).get(Contracts.Reward)).work(task.prover, msg.sender, task.miner);
        }

        delete tasks[tid];
        emit AdjudicateTask(tid, msg.sender, playerAmount, minerAmount, daoAmount, slash);
    }
}
