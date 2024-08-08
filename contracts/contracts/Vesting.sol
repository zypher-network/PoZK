// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IEpoch.sol";
import "./interface/IVesting.sol";

/// @notice Token lock status and unlock period
contract Vesting is Initializable, OwnableUpgradeable, IVesting {
    using SafeERC20 for IERC20;

    /// @notice Unit struct about mine reward
    struct MineReward {
        uint256 value;
        uint256 newValue;
        uint256 newEpoch;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Miner total vesting amount
    uint256 private _minersTotal;

    /// @notice Rewards of every epoch will be released for mine & play
    MineReward mineReward;

    /// @notice Store all miners vesting
    mapping(address => uint256) miners;

    /// @notice Emit when controller changed, isAdd if true is add, if false is remove
    event NewMineReward(uint256 epoch, uint256 amount);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    /// @param _amount the mine reward for per epoch
    function initialize(address _addresses, uint256 _amount) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;

        mineReward.value = _amount;
        mineReward.newValue = _amount;
        mineReward.newEpoch = 0;
        emit NewMineReward(0, _amount);
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Set the mine amount pre epoch
    /// @param amount new amount
    function setMineReward(uint256 amount) external onlyOwner {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        if (currentEpoch >= mineReward.newEpoch) {
            mineReward.value = mineReward.newValue;
            mineReward.newEpoch = currentEpoch + 1;
        }
        mineReward.newValue = amount;

        emit NewMineReward(currentEpoch + 1, amount);
    }

    /// @notice Approve enough token for reward
    /// @param amount new amount
    function approveForReward(uint256 amount) external onlyOwner {
        address reward = IAddresses(addresses).get(Contracts.Reward);
        IERC20(IAddresses(addresses).get(Contracts.Token)).approve(reward, amount);
    }

    /// @notice Get the mine amount of every epoch
    /// @notice epoch the epoch height
    /// @return the amount of reward
    function mine(uint256 epoch) external view returns(uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();

        if (currentEpoch >= mineReward.newEpoch) {
            return mineReward.newValue;
        } else {
            return mineReward.value;
        }
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
