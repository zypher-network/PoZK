// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

contract Epoch is Ownable {
    /// period time in seconds
    uint256 public period;

    /// current epoch
    uint256 public startTime;
    uint256 public now;

    /// enter/esc maintenance mode
    bool public maintenance;

    constructor() Ownable(msg.sender) {}

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
