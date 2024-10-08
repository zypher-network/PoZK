// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IEpoch.sol";

/// @notice Phases in the network, simulating "block height" in blockchain,
/// stake and reward are effective and issued according to the epoch
contract Epoch is Initializable, OwnableUpgradeable, IEpoch {
    /// @notice Common Addresses contract
    address addresses;

    /// @notice Period time in seconds
    uint256 public period;

    /// @notice Current epoch start time
    uint256 public startTime;

    /// @notice Current epoch height
    uint256 public height;

    /// @notice Enter/esc maintenance mode, when entry maintenance mode, stake and reward will be stopped
    bool public maintenance;

    /// @notice Current network mode
    NetworkMode private _networkMode;

    /// @notice the DAO accounts for the network (use for miner & prover cert)
    mapping(address => bool) public dao;

    /// @notice Emitted when entry new epoch
    event NewEpoch(uint256 height, uint256 startTime);

    /// @notice Emitted when entry new DAO account
    event AddDao(address account, bool ok);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    /// @param _period the epoch period time in seconds
    function initialize(address _addresses, uint256 _period) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
        period = _period;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract address
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Update period time
    /// @param _period the period time in seconds
    function setPeriod(uint256 _period) external onlyOwner {
        period = _period;
    }

    /// @notice Set maintenance mode status
    /// @param open open or false the maintenance mode
    function setMaintenance(bool open) external onlyOwner {
        maintenance = open;
    }

    /// @notice Set network mode
    /// @param _mode the network mode
    function setNetworkMode(NetworkMode _mode) external onlyOwner {
        _networkMode = _mode;
    }

    /// @notice Set network mode
    /// @param account the new DAO account
    /// @param ok the new status
    function addDao(address account, bool ok) external onlyOwner {
        dao[account] = ok;
    }

    /// @notice Update and get latest epoch height
    /// @return latest epoch height
    function getAndUpdate() external returns (uint256) {
        require(!maintenance, "E00");

        if (startTime + period < block.timestamp) {
            height++;
            startTime = block.timestamp;

            emit NewEpoch(height, startTime);
        }

        return height;
    }

    /// @notice Get current epoch height
    /// @return Current epoch height
    function get() external view returns (uint256) {
        require(!maintenance, "E00");

        if (startTime + period < block.timestamp) {
            return height + 1;
        } else {
            return height;
        }
    }

    /// @notice Get current network mode
    /// @return Current network mode
    function networkMode() external view returns (NetworkMode) {
        return _networkMode;
    }

    /// @notice Check DAO account
    /// @return Check result
    function isDao(address account) external view returns (bool) {
        return dao[account];
    }
}
