// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IStake.sol";
import "./interface/IController.sol";
import "./interface/IReward.sol";
import "./interface/ITaskMarket.sol";

contract TaskMarket is Initializable, OwnableUpgradeable, ITaskMarket {
    struct Task {
        TaskStatus status;
        address game;
        address player;
        uint256 fee;
        address miner;
        uint256 overTime;
        bytes data;
    }

    address addresses;

    /// next task id
    uint256 public nextId;

    mapping(uint256 => Task) private tasks;

    event CreateTask(uint256 id, address game, address player, uint256 fee, bytes data);
    event AcceptTask(uint256 id, address miner, uint256 overTime);
    event SubmitTask(uint256 id, uint256 fee);

    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

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
        require(IController(IAddresses(addresses).get(Contracts.Controller)).check(miner, msg.sender), "T01");

        Task storage task = tasks[tid];
        require(IStake(IAddresses(addresses).get(Contracts.Stake)).isMiner(task.game, miner), "T02");

        bool acceptable = task.status == TaskStatus.Waiting || task.overTime < block.timestamp;
        require(acceptable, "T03");

        task.status = TaskStatus.Proving;
        task.miner = miner;
        task.overTime = block.timestamp + 10; // TODO overtime

        emit AcceptTask(tid, miner, task.overTime);
    }

    function submit(uint256 tid, bytes calldata proof) external {
        Task storage task = tasks[tid];

        require(task.status == TaskStatus.Proving, "T04");

        // TODO zk verify

        // TODO send fee to miner
        if (task.fee > 0) {
            //
        }
        emit SubmitTask(tid, task.fee);

        // TODO PoZK to reward
        IReward(IAddresses(addresses).get(Contracts.Reward)).work(task.game, task.player, task.miner);

        delete tasks[tid];
    }
}
