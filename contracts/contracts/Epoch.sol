// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IEpoch.sol";

contract Epoch is Initializable, OwnableUpgradeable, IEpoch {
    address addresses;

    /// period time in seconds
    uint256 public period;

    /// current epoch
    uint256 public startTime;
    uint256 public now;

    /// enter/esc maintenance mode
    bool public maintenance;

    function initialize(address _addresses, uint256 _period) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
        period = _period;
    }

    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    function setMaintenance(bool open) external onlyOwner {
        maintenance = open;
    }

    function setPeriod(uint256 _period) external onlyOwner {
        period = _period;
    }

    function getAndUpdate() external returns (uint256) {
        require(!maintenance, "E00");

        if (startTime + period < block.timestamp) {
            now++;
            startTime = block.timestamp;
        }

        return now;
    }

    function get() external view returns (uint256) {
        require(!maintenance, "E00");

        if (startTime + period < block.timestamp) {
            return now + 1;
        } else {
            return now;
        }
    }
}
