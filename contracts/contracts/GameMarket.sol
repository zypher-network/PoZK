// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/introspection/ERC165Checker.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IEpoch.sol";
import "./interface/IGameMarket.sol";
import "./interface/IVerifier.sol";

/// @notice Manage all registered games
contract GameMarket is Initializable, OwnableUpgradeable, IGameMarket {
    using ERC165Checker for address;

    /// @notice Unit struct for number change
    struct GameWork {
        /// @notice Current value
        uint256 value;
        /// @notice Next epoch value
        uint256 newValue;
        /// @notice Next epoch height
        uint256 newEpoch;
    }

    /// @notice Unit struct for game verifier
    struct GameVerifier {
        /// @notice Current verifier
        address value;
        /// @notice Next epoch verifier
        address newValue;
        /// @notice Next epoch height
        uint256 newEpoch;
    }

    /// @notice Game struct
    struct Game {
        /// @notice Game status, include: Reviewing, Working, Upgrading, Stopped
        GameStatus status;
        /// @notice The game owner account
        address owner;
        /// @notice Current & future work status
        GameWork work;
        /// @notice Current & future version status
        GameWork version;
        /// @notice Current & future overtime status
        GameWork overtime;
        /// @notice Current & future verifier
        GameVerifier verifier;
        /// @notice The game is minable, control by game DAO
        bool minable;
        /// @notice The game name
        string name;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Current & future total game work
    GameWork private gamesTotalWork;

    /// @notice Store all game list
    mapping(address => Game) private games;

    /// @notice Emit when new game register and waiting reviewing
    event RegisterGame(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string name);

    /// @notice Emit when game owner transfer to others
    event TransferGame(address game, address owner);

    /// @notice Emit when the game start upgrading and waiting reviewing, before approve, it will still use old info
    event UpgradeGame(address game, uint256 work, uint256 version, address verifier, string name);

    /// @notice Emit when the game is approved or reject
    event ApproveGame(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string name, bool minable, bool approved);

    /// @notice Emit when the game is stopped
    event StopGame(address game);

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

    /// @notice Register new game to market, the sender is game owner, and waiting review
    /// @param game the game contract(or not) address (unique identifier)
    /// @param work the game pozk work, calculation based on zk scheme and circuit size
    /// @param version the game prover version
    /// @param overtime the limit time when doing zkp, if overflow the time, others miner can accept the task again
    /// @param verifier the verifier contract, uses the IVerifier interface
    /// @param name the game name
    function register(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string calldata name) external {
        require(games[game].version.value == 0 && version > 0, "G01");
        require(verifier.supportsInterface(type(IVerifier).interfaceId), "G04");

        Game storage g = games[game];
        g.status = GameStatus.Reviewing;
        g.owner = msg.sender;
        g.work = GameWork(work, work, 0);
        g.version = GameWork(version, version, 0);
        g.overtime = GameWork(overtime, overtime, 0);
        g.verifier = GameVerifier(verifier, verifier, 0);
        g.minable = false;
        g.name = name;

        emit RegisterGame(game, work, version, overtime, verifier, name);
    }

    /// @notice Game owner can unregister the game and cannot register same game address again
    /// @param game the game address
    function unregister(address game) external {
        require(games[game].owner == msg.sender, "G02");

        games[game].status = GameStatus.Stopped;

        emit StopGame(game);
    }

    /// @notice Game owner can upgrade the game to new verison and new info
    /// @param game the game
    /// @param work the game next pozk work, calculation based on zk scheme and circuit size
    /// @param version the game next prover version
    /// @param overtime the limit time when doing zkp, if overflow the time, others miner can accept the task again
    /// @param verifier the next verifier contract, uses the IVerifier interface
    /// @param name the game name, only name update immediately
    function upgrade(address game, uint256 work, uint256 version, uint256 overtime, address verifier, string calldata name) external {
        require(games[game].owner == msg.sender, "G02");
        require(verifier.supportsInterface(type(IVerifier).interfaceId), "G04");

        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        Game storage g = games[game];
        if (g.status == GameStatus.Working || g.status == GameStatus.Upgrading) {
            g.status = GameStatus.Upgrading;
        } else {
            g.status = GameStatus.Reviewing;
        }

        g.name = name; // name can update immediately

        // update work
        if (currentEpoch >= g.work.newEpoch) {
            g.work.value = g.work.newValue;
        }
        g.work.newValue = work;
        g.work.newEpoch = type(uint256).max;

        // update version
        if (currentEpoch >= g.version.newEpoch) {
            g.version.value = g.version.newValue;
        }
        g.version.newValue = version;
        g.version.newEpoch = type(uint256).max;

        // update overtime
        if (currentEpoch >= g.overtime.newEpoch) {
            g.overtime.value = g.overtime.newValue;
        }
        g.overtime.newValue = overtime;
        g.overtime.newEpoch = type(uint256).max;

        // update verifier
        if (currentEpoch >= g.verifier.newEpoch) {
            g.verifier.value = g.verifier.newValue;
        }
        g.verifier.newValue = verifier;
        g.verifier.newEpoch = type(uint256).max;

        emit UpgradeGame(game, work, version, verifier, name);
    }

    /// @notice Game owner can transfer ownership to others
    /// @param game the game
    /// @param owner the new owner account
    function transferGameOwner(address game, address owner) external {
        require(games[game].owner == msg.sender, "G02");

        games[game].owner = owner;

        emit TransferGame(game, owner);
    }

    /// @notice DAO can approve or reject the game register and upgrade, if approve, it will works in next epoch
    /// @param game the game
    /// @param minable if the game is minable, that means when create/accept the game task, will get reward from network
    /// @param approved approve or reject
    function approve(address game, bool minable, bool approved) external onlyOwner {
        Game storage g = games[game];
        require(g.status == GameStatus.Reviewing || g.status == GameStatus.Upgrading, "G03");

        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        g.minable  = minable;

        // update work & version
        g.version.newEpoch = currentEpoch;  // version update immediately
        g.overtime.newEpoch = currentEpoch; // overtime update immediately
        g.verifier.newEpoch = currentEpoch; // verifier update immediately
        if (approved) {
            g.work.newEpoch = currentEpoch + 1; // work need upgrade next epoch

            g.version.value = g.version.newValue;
            g.overtime.value = g.overtime.newValue;
            g.verifier.value = g.verifier.newValue;

            // update gamesTotalWork
            if (currentEpoch >= gamesTotalWork.newEpoch) {
                gamesTotalWork.value = gamesTotalWork.newValue;
            }
            bool isAdd = g.work.newValue > g.work.value;
            if (isAdd) {
                gamesTotalWork.newValue += g.work.newValue - g.work.value;
            } else {
                gamesTotalWork.newValue -= g.work.value - g.work.newValue;
            }
            gamesTotalWork.newEpoch = currentEpoch + 1;
        } else {
            // revoke
            g.work.newEpoch = currentEpoch;
            g.work.newValue = g.work.value;

            g.version.newValue = g.version.value;
            g.overtime.newValue = g.overtime.value;
            g.verifier.newValue = g.verifier.value;
        }

        emit ApproveGame(game, g.work.newValue, g.version.newValue, g.overtime.newValue, g.verifier.newValue, g.name, minable, approved);
    }

    /// @notice DAO can stop a game
    /// @param game the game
    function stop(address game) external onlyOwner {
        games[game].status = GameStatus.Stopped;

        emit StopGame(game);
    }

    /// @notice Check a game is working (working or upgrading)
    /// @param game the game
    /// @return working or not
    function isGame(address game) external view returns (bool) {
        return games[game].status == GameStatus.Working || games[game].status == GameStatus.Upgrading;
    }

    /// @notice Get all games work
    /// @return the work of all games
    function totalWork() external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();

        if (currentEpoch >= gamesTotalWork.newEpoch) {
            return gamesTotalWork.newValue;
        } else {
            return gamesTotalWork.value;
        }
    }

    /// @notice Get a game work
    /// @param game the game
    /// @return the work of the game
    function work(address game) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        GameWork storage w = games[game].work;

        if (currentEpoch >= w.newEpoch) {
            return w.newValue;
        } else {
            return w.value;
        }
    }

    /// notice Get a game version
    /// @param game the game
    /// @return the version of the game
    function version(address game) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        GameWork storage v = games[game].version;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }

    /// notice Get a game overtime
    /// @param game the game
    /// @return the overtime of the game
    function overtime(address game) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        GameWork storage v = games[game].overtime;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }

    /// notice Get a game verifier
    /// @param game the game
    /// @return the verifier of the game
    function verifier(address game) external view returns (address) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        GameVerifier storage v = games[game].verifier;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }
}
