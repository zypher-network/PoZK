// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IVesting.sol";

/// @notice Token lock status and unlock period
contract Vesting is Initializable, OwnableUpgradeable, IVesting {
    /// @notice Common Addresses contract
    address addresses;

    /// @notice Miner total vesting amount
    uint256 private _minersTotal;

    /// @notice Rewards of every epoch will be released for mine & play
    uint256 rewardPerEpoch;

    /// @notice Store all miners vesting
    mapping(address => uint256) miners;

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    /// @param _rewardPerEpoch the reward amount of every epoch
    function initialize(address _addresses, uint256 _rewardPerEpoch) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
        rewardPerEpoch = _rewardPerEpoch;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Get the mine amount of every epoch
    /// @notice epoch the epoch height
    /// @return the amount of reward
    function mine(uint256 epoch) external view returns(uint256) {
        return rewardPerEpoch; // TODO
    }

    /// @notice Batch set miner vesting amounts
    /// @param _miners the miners list
    /// @param amounts the amounts list
    function setMinerAmount(address[] calldata _miners, uint256[] calldata amounts) external onlyOwner {
        for (uint i = 0; i < _miners.length; i++) {
            miners[_miners[i]] += amounts[i];
        }
    }

    /// @notice Get all miners vesting amount
    /// @return total amount of all miners
    function minersTotal() external view returns(uint256) {
        return _minersTotal;
    }

    /// @notice Get miner vesting amount
    /// @param account the miner account
    /// @return the amount of this miner
    function miner(address account) external view returns(uint256) {
        return miners[account];
    }
}
