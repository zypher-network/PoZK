// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IVesting {
    function mine(uint256 epoch) external view returns (uint256);

    function minersTotal() external view returns(uint256);

    function miner(address account) external view returns(uint256);
}
