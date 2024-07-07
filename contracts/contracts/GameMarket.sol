// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

import "./Epoch.sol";

contract GameMarket is Ownable {
    enum GameStatus {
        Reviewing,
        Working,
        Upgrading,
        Stopped
    }

    /// uint for staking/unstaking
    struct GameWork {
        uint256 value;
        uint256 newValue;
        uint256 newEpoch;
    }

    struct GameVerifier {
        address value;
        address newValue;
        address newEpoch;
    }

    struct Game {
        GameStatus status;
        address owner;
        GameWork work;
        GameWork version;
        GameVerifier verifier;
        bool minable;
        string name;
    }

    /// total game work
    GameWork private totalWork;

    /// game list
    mapping(address => Game) private games;

    // TODO
    address epoch;

    event RegisterGame(address game, uint256 work, uint256 version, address verifier, string name);
    event TransferGame(address game, address owner);
    event UpgradeGame(address game, uint256 work, uint256 version, address verifier, string name);
    event ApproveGame(address game, uint256 work, uint256 version, address verifier, string name, bool minable, bool approved);
    event StopGame(address game);

    function register(address game, uint256 work, uint256 version, address verifier, string name) external {
        require(games[game].version == 0 && version > 0, "G01");

        Game storage g = games[game];
        g.status = GameStatus.Reviewing;
        g.owner = msg.sender;
        g.work = GameWork(work, work, 0);
        g.version = version;
        g.verifier = verifier;
        g.minable = false;
        g.name = name;

        emit RegisterGame(game, work, version, verifier, name);
    }

    function unregister(address game) external {
        require(games[game].owner == msg.sender, "G02");

        games[game].status = GameStatus.Stopped;

        emit StopGame(game);
    }

    function upgrade(address game, uint256 work, uint256 version, address verifier, string name) external {
        require(games[game].owner == msg.sender, "G02");
        uint256 currentEpoch = Epoch(epoch).getAndUpdate();

        Game storage g = games[game];
        if (g.status == GameStatus.Working || g.status == GameStatus.Upgrading) {
            g.status = GameStatus.Upgrading;
        } else {
            g.status = GameStatus.Reviewing;
        }

        g.name = name; // name can update immediately

        // update newValue
        if (currentEpoch >= g.work.newEpoch) {
            g.work.value = g.work.newValue;
        }
        g.work.newValue = work;
        g.work.newEpoch = type(uint256).max;

        if (currentEpoch >= g.version.newEpoch) {
            g.version.value = g.version.newValue;
        }
        g.version.newValue = version;
        g.version.newEpoch = type(uint256).max;

        if (currentEpoch >= g.verifier.newEpoch) {
            g.verifier.value = g.verifier.newValue;
        }
        g.verifier.newValue = verifier;
        g.verifier.newEpoch = type(uint256).max;

        emit UpgradeGame(game, work, version, verifier, name);
    }

    function transferGameOwner(address game, address owner) external {
        require(games[game].owner == msg.sender, "G02");

        games[game].owner = owner;

        emit TransferGame(game, owner);
    }

    function approve(address game, uint256 minable, bool approved) external onlyOwner {
        Game storage g = games[game];
        require(g.status == GameStatus.Reviewing || g.status == GameStatus.Upgrading, "G03");

        uint256 currentEpoch = Epoch(epoch).getAndUpdate();

        g.minable = minable;

        // update work & version
        g.version.newEpoch = currentEpoch;  // version update immediately
        g.verifier.newEpoch = currentEpoch; // verifier update immediately
        if (approved) {
            g.work.newEpoch = currentEpoch + 1; // work need upgrade next epoch

            g.version.value = g.version.newValue;
            g.verifier.value = g.verifier.newValue;

            // update totalWork
            if (currentEpoch >= totalWork.newEpoch) {
                totalWork.value = totalWork.newValue;
            }
            bool isAdd = g.work.newValue > g.work.value;
            if (isAdd) {
                totalWork.newValue += g.work.newValue - g.work.value;
            } else {
                totalWork.newValue -= g.work.value - g.work.newValue;
            }
            totalWork.newEpoch = currentEppoch + 1;
        } else {
            // revoke
            g.work.newEpoch = currentEpoch;
            g.work.newValue = g.work.value;

            g.version.newValue = g.version.value;
            g.verifier.newValue = g.verifier.value;
        }

        emit ApproveGame(game, g.work.newValue, g.version.newValue, g.verifier.newValue, g.name, minable, approved);
    }

    function stop(address game) external onlyOwner {
        games[game].status = GameStatus.Stopped;

        emit StopGame(game);
    }

    function totalWork() external view returns(uint256) {
        uint256 currentEpoch = Epoch(epoch).get();

        if (currentEpoch >= totalWork.newEpoch) {
            return totalWork.newValue;
        } else {
            return totalWork.value;
        }
    }

    function work(address game) external view returns(uint256) {
        uint256 currentEpoch = Epoch(epoch).get();
        GameWork storage w = games[game].work;

        if (currentEpoch >= w.newEpoch) {
            return w.newValue;
        } else {
            return w.value;
        }
    }

    function version(address game) external view returns(uint256) {
        uint256 currentEpoch = Epoch(epoch).get();
        GameWork storage v = games[game].version;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }

    function verifier(address game) external view returns(address) {
        uint256 currentEpoch = Epoch(epoch).get();
        GameVerifier storage v = games[game].verifier;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }
}
