// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IController.sol";
import "./interface/IGameMarket.sol";
import "./interface/IReward.sol";
import "./interface/IStake.sol";
import "./interface/ITaskMarket.sol";
import "./interface/IVerifier.sol";

contract TaskMarket is Initializable, OwnableUpgradeable, ITaskMarket {
    using SafeERC20 for IERC20;

    struct Task {
        TaskStatus status;
        address game;
        address player;
        uint256 fee;
        address miner;
        uint256 overtime;
        bytes data;
    }

    address addresses;

    /// next task id
    uint256 public nextId;

    mapping(uint256 => Task) private tasks;

    event CreateTask(uint256 id, address game, address player, uint256 fee, bytes data);
    event AcceptTask(uint256 id, address miner, uint256 overtime);
    event SubmitTask(uint256 id, uint256 fee);

    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    function create(address game, address player, uint256 fee, bytes calldata data) external returns(uint256) {
        // transfer fee from msg.sender
        if (fee > 0) {
            IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(msg.sender, address(this), fee);
        }

        // check game is valid
        require(IGameMarket(IAddresses(addresses).get(Contracts.GameMarket)).isGame(game), "T01");

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
        require(IController(IAddresses(addresses).get(Contracts.Controller)).check(miner, msg.sender), "T02");

        Task storage task = tasks[tid];
        require(IStake(IAddresses(addresses).get(Contracts.Stake)).isMiner(task.game, miner), "T03");

        bool acceptable = task.status == TaskStatus.Waiting || task.overtime < block.timestamp;
        require(acceptable, "T04");

        uint256 overtime = IGameMarket(IAddresses(addresses).get(Contracts.GameMarket)).overtime(task.game);
        task.status = TaskStatus.Proving;
        task.miner = miner;
        task.overtime = block.timestamp + overtime;

        emit AcceptTask(tid, miner, task.overtime);
    }

    function submit(uint256 tid, bytes calldata publics, bytes calldata proof) external {
        Task storage task = tasks[tid];

        require(task.status == TaskStatus.Proving, "T05");

        // zk verifier
        address verifier = IGameMarket(IAddresses(addresses).get(Contracts.GameMarket)).verifier(task.game);
        require(IVerifier(verifier).verify(publics, proof), "T99");

        // send fee to miner
        if (task.fee > 0) {
            IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(task.miner, task.fee);
        }
        emit SubmitTask(tid, task.fee);

        // PoZK to reward
        IReward(IAddresses(addresses).get(Contracts.Reward)).work(task.game, task.player, task.miner);

        delete tasks[tid];
    }
}
