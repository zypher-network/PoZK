// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IVesting.sol";

contract Vesting is Initializable, OwnableUpgradeable, IVesting {
    address addresses;

    uint256 private _minersTotal;

    uint256 rewardPerEpoch;

    mapping(address => uint256) miners;

    function initialize(address _addresses, uint256 _rewardPerEpoch) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
        rewardPerEpoch = _rewardPerEpoch;
    }

    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    function mine(uint256 epoch) external view returns(uint256) {
        return rewardPerEpoch;
    }

    function setMinerAmount(address[] calldata _miners, uint256[] calldata amounts) external onlyOwner {
        for (uint i = 0; i < _miners.length; i++) {
            miners[_miners[i]] += amounts[i];
        }
    }

    function minersTotal() external view returns(uint256) {
        return _minersTotal;
    }

    function miner(address account) external view returns(uint256) {
        return miners[account];
    }
}
