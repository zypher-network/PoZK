// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "./Stake.sol";
import "./Controller.sol";
import "./Reward.sol";

contract TaskMarket is Ownable {
    enum TaskStatus {
        Waiting,
        Proving
    }

    struct Task {
        TaskStatus status;
        address game;
        address player;
        uint256 fee;
        address miner;
        uint256 overTime;
        bytes data;
    }

    /// next task id
    uint256 public nextId;

    // TODO
    address controller;
    address stake;
    address reward;

    mapping(uint256 => Task) private tasks;

    event CreateTask(uint256 id, address game, address player, uint256 fee, bytes data);
    event AcceptTask(uint256 id, address miner, uint256 overTime);
    event SubmitTask(uint256 id, uint256 fee);

    constructor() Ownable(msg.sender) {}


    function create(address game, address player, uint256 fee, bytes calldata data) external returns(uint256) {
        // TODO transfer fee from msg.sender
        if (fee > 0) {
            //
        }

        // TODO check game is valid

        Task storage task = tasks[nextId];
        task.game = game;
        task.player = player;
        task.fee = fee;
        task.data = data;

        emit CreateTask(nextId, game, player, fee, data);

        nextId += 1;
        return nextId - 1;
    }

    function accept(uint256 tid, address miner) external {
        Controller(controller).check(miner, msg.sender);

        Task storage task = tasks[tid];
        require(Stake(stake).isMiner(task.game, miner), "T01");

        bool acceptable = task.status == TaskStatus.Waiting || task.overTime < block.timestamp;
        require(acceptable, "T02");

        task.status = TaskStatus.Proving;
        task.miner = miner;
        task.overTime = block.timestamp + 10; // TODO overtime

        emit AcceptTask(tid, miner, task.overTime);
    }

    function submit(uint256 tid, bytes calldata proof) external {
        Task storage task = tasks[tid];

        require(task.status == TaskStatus.Proving, "T03");

        // TODO zk verify

        // TODO send fee to miner
        if (task.fee > 0) {
            //
        }
        emit SubmitTask(tid, task.fee);

        // TODO PoZK to reward
        Reward(reward).work(task.game, task.player, task.miner);

        delete tasks[tid];
    }
}
